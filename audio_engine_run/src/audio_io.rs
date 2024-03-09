use std::io::stdin;

use anyhow::anyhow;
use audio_engine::audio_channel::{AudioChannel, AudioRx, AudioTx};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device,
    SupportedBufferSize::Range,
    SupportedStreamConfig,
};

use ringbuf::{
    ring_buffer::{RbRead, RbRef, RbWrite},
    Consumer, HeapRb, Producer,
};

fn err_fn(e: cpal::StreamError) {
    eprintln!("error in stream: {}", e);
}

#[derive(Default)]
pub struct AudioIo {
    input_device: Option<cpal::Device>,
    output_device: Option<cpal::Device>,
    input_config: Option<cpal::SupportedStreamConfig>,
    output_config: Option<cpal::SupportedStreamConfig>,
    input_stream: Option<cpal::Stream>,
    output_stream: Option<cpal::Stream>,
    num_channels: usize,
    num_samples_per_channel: usize,
}

impl AudioIo {
    pub fn new() -> AudioIo {
        AudioIo::default()
    }

    pub fn get_total_buffer_size(&self) -> usize {
        self.num_channels * self.num_samples_per_channel
    }

    pub fn get_num_channels(&self) -> usize {
        self.num_channels
    }

    pub fn get_num_samples_per_channel(&self) -> usize {
        self.num_samples_per_channel
    }

    pub fn start(
        &mut self,
        engine_in: &mut AudioChannel,
        engine_out: &mut AudioChannel,
    ) -> anyhow::Result<()> {
        let latency_samples = self.get_total_buffer_size();
        let buffer_size = self.get_total_buffer_size();

        let output_device = self
            .output_device
            .as_mut()
            .ok_or(anyhow!("no output device"))?;

        let input_device = self
            .input_device
            .as_mut()
            .ok_or(anyhow!("no input device"))?;

        let ring = HeapRb::<f32>::new((latency_samples * 2) as usize);
        let (mut output_tx, mut input_rx) = ring.split();

        for _ in 0..latency_samples {
            if let Err(e) = output_tx.push(0.0) {
                return Err(anyhow!("couldn't init ring buffer: {}", e));
            }
        }

        let in_callback = move |data: &[f32], _: &cpal::InputCallbackInfo| {
            AudioIo::input_callback(data, &mut output_tx);
        };

        let mut tx_engine = engine_in.take_tx().ok_or(anyhow!("no tx on engine"))?;

        let mut rx_engine = engine_out.take_rx().ok_or(anyhow!("no rx on engine"))?;

        let out_callback = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            AudioIo::output_callback(data, &mut input_rx, &mut tx_engine, &mut rx_engine);
        };

        let out_stream = output_device.build_output_stream(
            &self
                .output_config
                .as_ref()
                .ok_or(anyhow!("no output config"))?
                .config(),
            out_callback,
            err_fn,
            None,
        )?;

        let in_stream = input_device.build_input_stream(
            &self
                .input_config
                .as_ref()
                .ok_or(anyhow!("no input config"))?
                .config(),
            in_callback,
            err_fn,
            None,
        )?;

        out_stream.play()?;
        in_stream.play()?;

        self.input_stream = Some(in_stream);
        self.output_stream = Some(out_stream);

        Ok(())
    }

    fn input_callback<R: RbRef>(data: &[f32], output_tx: &mut Producer<f32, R>)
    where
        <R as RbRef>::Rb: RbWrite<f32>,
    {
        let count = output_tx.push_slice(data);

        if count != data.len() {
            eprintln!("failed pushing sample, samples pushed: {}", count);
        }
    }

    fn output_callback<R: RbRef>(
        data: &mut [f32],
        input_rx: &mut Consumer<f32, R>,
        engine_tx: &mut AudioTx,
        engine_rx: &mut AudioRx,
    ) where
        <R as RbRef>::Rb: RbRead<f32>,
    {
        let _count = input_rx.pop_slice(data);

        engine_tx.push_slice(data);

        engine_rx.collect_samples(data);
    }

    pub fn stop(&mut self) -> anyhow::Result<()> {
        // We drop the streams here because they use RAII
        // instead of a stop method. Because we want to have
        // control over when they stop we do so.
        drop(
            self.input_stream
                .take()
                .ok_or(anyhow!("no input stream to stop"))?,
        );
        drop(
            self.output_stream
                .take()
                .ok_or(anyhow!("no output stream to stop"))?,
        );
        Ok(())
    }

    pub fn configure(&mut self) -> anyhow::Result<()> {
        let host = AudioIo::configure_host()?;

        println!("Configure Input Device: ");
        let input_device = AudioIo::configure_device_from_host(&host)?;

        println!("Configure Output Device: ");
        let output_device = AudioIo::configure_device_from_host(&host)?;

        let input_config = AudioIo::get_default_input_config(&input_device)?;
        let output_config = AudioIo::get_default_output_config(&output_device)?;

        if input_config.channels() != output_config.channels() {
            return Err(anyhow!("for now, io channels must match"));
        }

        let min_cmp: usize;
        let max_cmp: usize;

        if let Range { min, max } = output_config.buffer_size() {
            min_cmp = *min as usize;
            max_cmp = *max as usize;
        } else {
            return Err(anyhow!("no supported output buffer size"));
        }

        if let Range { min, max } = input_config.buffer_size() {
            if min != max {
                return Err(anyhow!("buffer min and max don't match"));
            }
            if min_cmp != *min as usize {
                return Err(anyhow!("min buffer config doesn't match for i/o"));
            }
            if max_cmp != *max as usize {
                return Err(anyhow!("max buffer config doesn't match for i/o"));
            }
            self.num_samples_per_channel = *max as usize;
        } else {
            return Err(anyhow!("no supported input buffer size"));
        }

        self.num_channels = output_config.channels() as usize;

        self.input_device = Some(input_device);
        self.output_device = Some(output_device);

        self.input_config = Some(input_config);
        self.output_config = Some(output_config);

        Ok(())
    }

    fn get_user_choice() -> anyhow::Result<usize> {
        let mut choice = String::new();
        stdin().read_line(&mut choice)?;
        let index: usize = choice.trim().parse()?;
        Ok(index)
    }

    fn configure_host() -> anyhow::Result<cpal::Host> {
        let mut available_hosts = cpal::available_hosts();

        available_hosts
            .iter()
            .enumerate()
            .for_each(|(i, host)| println!("{}: {}", i, host.name()));

        let index = AudioIo::get_user_choice()?;

        let id = available_hosts
            .get_mut(index)
            .ok_or(anyhow!("index: {}, out of bounds", index))?;

        Ok(cpal::host_from_id(*id)?)
    }

    fn configure_device_from_host(host: &cpal::Host) -> anyhow::Result<Device> {
        host.devices()?
            .filter(|d| d.name().is_ok())
            .enumerate()
            .try_for_each(|(i, device)| -> anyhow::Result<()> {
                println!("{}: {}", i, device.name()?);
                Ok(())
            })?;

        let index = AudioIo::get_user_choice()?;

        let device = host
            .devices()?
            .nth(index)
            .ok_or(anyhow!("index: {}, out of bounds", index))?;

        Ok(device)
    }

    fn get_default_input_config(device: &Device) -> anyhow::Result<SupportedStreamConfig> {
        let input_config = device.default_input_config()?;

        println!("Default Input Config: ");
        println!("Input Config: {:?}", input_config);

        Ok(input_config)
    }

    fn get_default_output_config(device: &Device) -> anyhow::Result<SupportedStreamConfig> {
        let output_config = device.default_output_config()?;

        println!("Default Output Config: ");
        println!("Output Config: {:?}", output_config);

        Ok(output_config)
    }
}
