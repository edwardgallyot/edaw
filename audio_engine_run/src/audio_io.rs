use std::io::stdin;

use anyhow::{anyhow, Result};
use cpal::{
    platform::AlsaSupportedInputConfigs,
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, SupportedInputConfigs, SupportedStreamConfig, SupportedBufferSize::{Unknown, Range},
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
}

impl AudioIo {
    pub fn new() -> AudioIo {
        AudioIo::default()
    }

    fn get_latency_samples_from_config(config: &SupportedStreamConfig) -> anyhow::Result<u32> {
        return match config.buffer_size() {
            Range { min: _, max } => Ok(max * config.channels() as u32),
            Unknown => Err(anyhow!("unknown buffer size")),
        };
    }

    pub fn start(&mut self) -> anyhow::Result<()> {
        let output_device = self
            .output_device
            .as_mut()
            .ok_or(anyhow!("no output device"))?;

        let input_device = self
            .input_device
            .as_mut()
            .ok_or(anyhow!("no input device"))?;

        let output_config = self
            .output_config
            .as_ref()
            .ok_or(anyhow!("no output device"))?;

        let input_config = self
            .input_config
            .as_ref()
            .ok_or(anyhow!("no input device"))?;

        let input_latency_samples = AudioIo::get_latency_samples_from_config(&input_config)?;
        let output_latency_samples = AudioIo::get_latency_samples_from_config(&output_config)?;

        if input_latency_samples != output_latency_samples {
            return Err(anyhow!(
                "in latency: {}, and out latency: {} don't match",
                input_latency_samples,
                output_latency_samples
            ));
        }

        let ring = HeapRb::<f32>::new((input_latency_samples * 2) as usize);
        let (mut tx, mut rx) = ring.split();

        for _ in 0..input_latency_samples {
            if let Err(e) = tx.push(0.0) {
                return Err(anyhow!("couldn't init ring buffer: {}", e));
            }
        }

        let in_callback = move |data: &[f32], _: &cpal::InputCallbackInfo| {
            AudioIo::input_callback(data, &mut tx);
        };

        let out_callback = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            AudioIo::output_callback(data, &mut rx);
        };

        let in_stream = input_device.build_input_stream(&input_config.config(), in_callback, err_fn, None)?;
        let out_stream = output_device.build_output_stream(&output_config.config(), out_callback, err_fn, None)?;

        in_stream.play()?;
        out_stream.play()?;

        self.input_stream = Some(in_stream);
        self.output_stream = Some(out_stream);

        Ok(())
    }

    fn input_callback<R: RbRef>(data: &[f32], tx: &mut Producer<f32, R>)
    where
        <R as RbRef>::Rb: RbWrite<f32>,
    {
        let mut failed = false;
        data.iter().for_each(|s: &f32| {
            if let Err(_) = tx.push(*s) {
                failed = true;
            }
        });
        if failed {
            eprintln!("failed pushing sample, adjust latency");
        }
    }

    fn output_callback<R: RbRef>(data: &mut [f32], rx: &mut Consumer<f32, R>)
    where
        <R as RbRef>::Rb: RbRead<f32>,
    {
        let mut failed = false;
        data.iter_mut().for_each(|s: &mut f32| {
            *s = match rx.pop() {
                Some(s) => s,
                None => {
                    failed = true;
                    0.0
                }
            };
        });
        if failed {
            eprintln!("failed pushing sample, adjust latency");
        }
    }

    pub fn stop(&mut self) -> anyhow::Result<()> {
        // We drop the streams here because they use RAII
        // instead of a stop method. Because we want to have
        // control over when they stop we do so.
        let _ = self.input_stream.take();
        let _ = self.output_stream.take();
        Ok(())
    }

    pub fn configure(&mut self) -> anyhow::Result<()> {
        let host = AudioIo::configure_host()?;
        let input_device = AudioIo::configure_device_from_host(&host)?;
        let output_device = AudioIo::configure_device_from_host(&host)?;
        let input_config = AudioIo::get_default_input_config(&input_device)?;
        let output_config = AudioIo::get_default_output_config(&output_device)?;

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
