use std::{error::Error, io};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod app;
mod ui;

use crate::{
    app::{App, CurrentScreen, CurrentlyEditing},
    ui::ui,
};

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|frame| ui(frame, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }

            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('e') => {
                        app.current_screen = CurrentScreen::Editing;
                        app.currently_editing = Some(CurrentlyEditing::Key);
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exiting;
                    }

                    _ => {}
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true);
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        return Ok(false);
                    }
                    _ => {}
                },
                CurrentScreen::Editing if key.kind == KeyEventKind::Press => {
                    match key.code {
                        // if editing a key, start editing value
                        // if editing a value, save and return to main screen
                        KeyCode::Enter => {
                            if let Some(editing) = &app.currently_editing {
                                match editing {
                                    CurrentlyEditing::Key => {
                                        app.currently_editing = Some(CurrentlyEditing::Value);
                                    }
                                    CurrentlyEditing::Value => {
                                        app.save_key_value();
                                        app.current_screen = CurrentScreen::Main;
                                    }
                                }
                            }
                        }
                        // remove the chars from the key or value from the end one at a time
                        KeyCode::Backspace => {
                            if let Some(editing) = &app.currently_editing {
                                match editing {
                                    CurrentlyEditing::Key => {
                                        app.key_input.pop();
                                    }
                                    CurrentlyEditing::Value => {
                                        app.value_input.pop();
                                    }
                                }
                            }
                        }
                        // escape is used to stop editing
                        KeyCode::Esc => {
                            app.current_screen = CurrentScreen::Main;
                            app.currently_editing = None;
                        }
                        // tab is used to switch editing selection
                        KeyCode::Tab => {
                            app.toggle_editing();
                        }
                        // capture valid chars for the key or value
                        KeyCode::Char(value) => {
                            if let Some(editing) = &app.currently_editing {
                                match editing {
                                    CurrentlyEditing::Key => {
                                        app.key_input.push(value);
                                    }
                                    CurrentlyEditing::Value => {
                                        app.value_input.push(value);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;

    // using stderr so stdout/json can be redirected to a file
    let mut stderr = io::stderr();

    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            app.print_json()?
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}
