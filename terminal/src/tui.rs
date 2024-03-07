mod app;

use std::{
    io::stdout,
    ops::ControlFlow,
    sync::{
        atomic::{self, AtomicBool},
        Arc,
    },
    thread::{spawn, JoinHandle, self}, time, panic,
};

use anyhow::Result;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::backend::CrosstermBackend;

const SLEEP_TIME_MS: u64 = 10;

fn setup_terminal() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    Ok(())
}

fn reset_terminal() -> Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn set_up_panic_hooks() {
    let original_hook = panic::take_hook();
    std::panic::set_hook(Box::new(move |panic| {
        reset_terminal().unwrap();
        original_hook(panic);
    }));
}

#[derive(Default)]
pub struct Tui {
    pub run: Arc<AtomicBool>,
    pub handle: Option<JoinHandle<Option<app::App>>>,
}

impl Tui {
    pub fn new() -> Tui {
        set_up_panic_hooks();
        Tui::default()
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        // Clone the run variable as this can be set across threads.
        let run_clone = self.run.clone();
        run_clone.store(true, atomic::Ordering::Relaxed);

        // We create the app local to this function because we can
        // move it into the run thread.
        let mut app = app::App::new();

        // Create the thread that runs our main loop
        let handle = spawn(move || {
            set_up_panic_hooks();
            match Tui::run_main_loop(run_clone, &mut app) {
                Err(e) => eprintln!("Error running main loop: {}", e),
                Ok(()) => {}
            }
            // Return the modified state out of the main thread
            Some(app)
        });

        // Store some handle for it to be joined later
        self.handle = Some(handle);

        Ok(())
    }

    fn run_main_loop(run: Arc<AtomicBool>, app: &mut app::App) -> anyhow::Result<()> {
        setup_terminal()?;
        let mut terminal = ratatui::Terminal::new(CrosstermBackend::new(stdout()))?;
        terminal.clear()?;

        while run.load(atomic::Ordering::Relaxed) {
            thread::sleep(time::Duration::from_millis(SLEEP_TIME_MS));
            app.handle_drawing(&mut terminal)?;

            match app.handle_input()? {
                ControlFlow::Break(_) => break,
                ControlFlow::Continue(_) => continue,
            }
        }

        reset_terminal()?;
        Ok(())
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        self.run.store(false, std::sync::atomic::Ordering::Relaxed);

        // Join the handle if it's still running.
        if let Some(handle) = self.handle.take() {
            if let Err(e) = handle.join() { 
                eprintln!("error joining thread: {:?}", e);
            }
        }
    }
}
