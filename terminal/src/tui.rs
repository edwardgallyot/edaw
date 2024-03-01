mod app;

use std::{
    io::{stdout, Stdout},
    ops::ControlFlow,
    sync::{
        atomic::{self, AtomicBool},
        Arc,
    },
    thread::{spawn, JoinHandle},
};

use crossterm::{
    event::{self, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, style::Stylize, widgets::Paragraph};

#[derive(Default)]
pub struct Tui {
    pub run: Arc<AtomicBool>,
    pub handle: Option<JoinHandle<Option<app::App>>>,
    pub app: Option<app::App>,
}

impl Tui {
    pub fn new() -> Tui {
        Tui::default()
    }

    pub fn stop(&mut self) {
        self.run.store(false, std::sync::atomic::Ordering::Relaxed);

        // Join the handle if it's still running.
        if let Some(handle) = self.handle.take() {
            let inner_state = match handle.join() {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error joining thread: {:?}", e);
                    None
                }
            };
            self.app = inner_state;
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        // Clone the run variable as this can be set across threads.
        let run_clone = self.run.clone();
        run_clone.store(true, atomic::Ordering::Relaxed);

        // Take the state to be passed into the run thread
        let mut state = match self.app.take() {
            Some(s) => s,
            None => app::App::new(),
        };

        // Create the thread that runs our main loop
        let handle = spawn(move || {
            match Tui::run_main_loop(run_clone, &mut state) {
                Err(e) => eprintln!("Error running main loop: {}", e),
                Ok(()) => {}
            }
            // Return the modified state out of the main thread
            Some(state)
        });

        // Store some handle for it to be joined later
        self.handle = Some(handle);

        Ok(())
    }

    fn run_main_loop(run: Arc<AtomicBool>, app: &mut app::App) -> anyhow::Result<()> {
        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        let mut terminal = ratatui::Terminal::new(CrosstermBackend::new(stdout()))?;
        terminal.clear()?;

        while run.load(atomic::Ordering::Relaxed) {
            Tui::handle_drawing(&mut terminal, app)?;

            match Tui::handle_input(app)? {
                ControlFlow::Break(_) => break,
                ControlFlow::Continue(_) => continue,
            }
        }

        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }

    fn handle_drawing(
        terminal: &mut ratatui::Terminal<CrosstermBackend<Stdout>>,
        app: &mut app::App,
    ) -> anyhow::Result<()> {
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new("Sami Terminal").black().on_light_magenta(),
                area,
            );
        })?;
        Ok(())
    }

    fn handle_input(app: &mut app::App) -> anyhow::Result<ControlFlow<()>> {
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                return Tui::match_key_codes(key.code);
            }
        }
        Ok(ControlFlow::Continue(()))
    }

    fn match_key_codes(code: KeyCode) -> anyhow::Result<ControlFlow<()>> {
        match code {
            KeyCode::Char('q') => {
                return Ok(ControlFlow::Break(()));
            }
            KeyCode::Char('Q') => {
                return Ok(ControlFlow::Break(()));
            }
            _ => {}
        }
        Ok(ControlFlow::Continue(()))
    }
}
