use std::{io::{stdout, Stdout}, sync::{atomic::{self, AtomicBool}, Arc}, os::unix::thread, thread::{spawn, JoinHandle}};

use crossterm::{ExecutableCommand, terminal::{EnterAlternateScreen, enable_raw_mode, LeaveAlternateScreen, disable_raw_mode}};
use ratatui::{backend::CrosstermBackend, widgets::Paragraph, style::Stylize};


#[derive(Default)]
pub struct Terminal {
    pub counter: i32,
    pub run: Arc<AtomicBool>,
    pub handle: Option<JoinHandle<()>>,
}

impl Terminal {
    pub fn new() -> Terminal {
        Terminal::default()
    }

    pub fn run(&mut self) -> anyhow::Result<()> {

        stdout().execute(EnterAlternateScreen)?;

        enable_raw_mode()?;

        let mut terminal = ratatui::Terminal::new(CrosstermBackend::new(stdout()))?;

        let run_clone = self.run.clone();
        terminal.clear()?;
        run_clone.store(true, atomic::Ordering::Relaxed);

        let handle = spawn(move || {
            while run_clone.load(atomic::Ordering::Relaxed) {
                terminal.draw(|frame|{
                    let area = frame.size();
                    frame.render_widget(
                        Paragraph::new("This is a lot better")
                        .black()
                        .on_light_green(),
                        area,
                    );
                }).unwrap();
            }
        });

        self.handle = Some(handle);

        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    fn main_loop(&self, terminal: &mut ratatui::Terminal<CrosstermBackend<Stdout>>) -> anyhow::Result<()> {
        // loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        //     if !self.run.load(atomic::Ordering::Relaxed) {
        //         break;
        //     }
        // }
        Ok(())
    }
}
