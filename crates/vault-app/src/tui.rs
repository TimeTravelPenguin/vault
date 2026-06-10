use std::io;

use ratatui::{
    crossterm::{
        cursor::Show,
        event::{
            self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
            KeyModifiers,
        },
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    prelude::*,
};
use thiserror::Error;

use crate::{
    config::AppConfig,
    db::VaultStore,
};

#[derive(Debug, Error)]
pub enum TuiError {
    #[error("terminal error: {0}")]
    Terminal(#[from] io::Error),
}

pub async fn run(config: AppConfig, store: VaultStore) -> Result<(), TuiError> {
    install_panic_hook();

    let mut guard = TerminalGuard::new()?;
    let terminal = &mut guard.terminal;

    let mut app = App::new(config, store);
    let mut state = AppState {};

    while !app.exit {
        terminal.draw(|frame| frame.render_stateful_widget(&app, frame.area(), &mut state))?;
        app.handle_events()?;
    }

    Ok(())
}

fn install_panic_hook() {
    let original = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |info| {
        let _ = restore_terminal(&mut io::stderr());

        original(info);
    }));
}

fn restore_terminal(writer: &mut impl io::Write) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(writer, LeaveAlternateScreen, DisableMouseCapture, Show)?;

    Ok(())
}

/// RAII guard owning the terminal: enters raw mode and the alternate screen on
/// construction, and restores the terminal on [`Drop`].
struct TerminalGuard {
    terminal: Terminal<CrosstermBackend<io::Stderr>>,
}

impl TerminalGuard {
    /// Enter raw mode and the alternate screen, returning a guard over the
    /// terminal that will restore the terminal on drop.
    fn new() -> io::Result<Self> {
        enable_raw_mode()?;

        // Build the terminal first: on failure the alternate screen hasn't been
        // entered yet, so disabling raw mode is enough to undo our changes.
        let mut terminal = match Terminal::new(CrosstermBackend::new(io::stderr())) {
            Ok(terminal) => terminal,
            Err(err) => {
                let _ = disable_raw_mode();

                return Err(err);
            }
        };

        if let Err(err) = execute!(
            terminal.backend_mut(),
            EnterAlternateScreen,
            EnableMouseCapture
        ) {
            let _ = restore_terminal(terminal.backend_mut());

            return Err(err);
        }

        Ok(Self { terminal })
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = restore_terminal(self.terminal.backend_mut());
    }
}

#[derive(Debug)]
struct AppState {}

#[derive(Debug)]
enum AppMode {
    NoUserConfiguration,
}

#[derive(Debug)]
struct App {
    config: AppConfig,
    store: VaultStore,
    exit: bool,
}

impl App {
    pub fn new(config: AppConfig, store: VaultStore) -> Self {
        Self {
            config,
            store,
            exit: false,
        }
    }

    /// Signal the TUI to exit on the next iteration of the event loop.
    pub fn exit(&mut self) {
        self.exit = true;
    }

    fn handle_events(&mut self) -> Result<(), TuiError> {
        match event::read()? {
            // It's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            // In raw mode crossterm delivers Ctrl+C as a key event rather than a
            // SIGINT, so we handle it ourselves to quit.
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.exit()
            }
            KeyCode::Left => todo!(),
            KeyCode::Right => todo!(),
            _ => {}
        }
    }
}

impl StatefulWidget for &App {
    type State = AppState;

    fn render(self, _area: Rect, _buf: &mut Buffer, _state: &mut Self::State) {
        todo!()
    }
}
