use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, CurrentScreen, CurrentlyEditing};

macro_rules! styled_span {
    ($label:tt, $color:expr) => {
        Span::styled($label, Style::default().fg($color))
    };
}

pub fn ui(f: &mut Frame, app: &App) {
    enum LayoutChunks {
        Header = 0,
        Body = 1,
        Footer = 2,
    }

    enum LayoutFooter {
        CurrentActivity = 0,
        KeyBindings = 1,
    }

    /// helper function to create a centered rect using up certain percentage of the available rect `r`
    fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        // Cut the given rectangle into three vertical pieces
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        // Then cut the middle vertical piece into three width-wise pieces
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1] // Return the middle chunk
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Create New Json",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    f.render_widget(title, chunks[LayoutChunks::Header as usize]);

    let mut list_items = Vec::<ListItem>::new();

    for key in app.pairs.keys() {
        list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{: <25} : {}", key, app.pairs.get(key).unwrap()),
            Style::default().fg(Color::Yellow),
        ))));
    }

    let list = List::new(list_items);

    f.render_widget(list, chunks[LayoutChunks::Body as usize]);

    let current_navigation_text = vec![
        // the first half of the text
        match app.current_screen {
            CurrentScreen::Main => styled_span!("Normal Mode", Color::Green),
            CurrentScreen::Editing => {
                styled_span!("Editing Mode", Color::Yellow)
            }
            CurrentScreen::Exiting => {
                styled_span!("Exiting Mode", Color::LightRed)
            }
        }
        .to_owned(),
        // a white divider bar to separate the two sections
        styled_span!(" | ", Color::White),
        // the final section of the text, with hings on what the user is editing
        {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Key => {
                        styled_span!("Editing Json Key", Color::Green)
                    }
                    CurrentlyEditing::Value => {
                        styled_span!("Editing Json Value", Color::LightGreen)
                    }
                }
            } else {
                styled_span!("Not Editing Json Anything", Color::DarkGray)
            }
        },
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => styled_span!("(q) to quit / (e) to make new pair", Color::Red),
            CurrentScreen::Editing => styled_span!(
                "(ESC) to cancel / (Tab) to switch boxes/enter to complete",
                Color::Red
            ),
            CurrentScreen::Exiting => {
                styled_span!("(q) to quit / (e) to make new pair", Color::Red)
            }
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[LayoutChunks::Footer as usize]);

    f.render_widget(
        mode_footer,
        footer_chunks[LayoutFooter::CurrentActivity as usize],
    );
    f.render_widget(
        key_notes_footer,
        footer_chunks[LayoutFooter::KeyBindings as usize],
    );

    if let Some(editing) = &app.currently_editing {
        let popup_block = Block::default()
            .title("Enter a new key-value pair")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let area = centered_rect(60, 25, f.size());

        f.render_widget(popup_block, area);

        let popup_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let mut key_block = Block::default().title("Key").borders(Borders::ALL);
        let mut value_block = Block::default().title("value").borders(Borders::ALL);

        let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

        match editing {
            CurrentlyEditing::Key => key_block = key_block.style(active_style),
            CurrentlyEditing::Value => value_block = value_block.style(active_style),
        };

        let key_text = Paragraph::new(app.key_input.clone()).block(key_block);

        f.render_widget(
            key_text,
            popup_chunks[LayoutFooter::CurrentActivity as usize],
        );

        let value_text = Paragraph::new(app.value_input.clone()).block(value_block);

        f.render_widget(value_text, popup_chunks[LayoutFooter::KeyBindings as usize]);
    }

    if let CurrentScreen::Exiting = app.current_screen {
        // clears entire screen
        f.render_widget(Clear, f.size());

        let popup_block = Block::default()
            .title("Y/N")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let exit_text = Text::styled(
            "Would you like to output the buffer as json? (y/n)",
            Style::default().fg(Color::Red),
        );

        //the `trim: false` will stop the text from being cut off when over the endge
        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = centered_rect(60, 25, f.size());

        f.render_widget(exit_paragraph, area);
    }
}
