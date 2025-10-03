use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::{app::App, kana_type::KanaType};

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(f.area());

    let input = Paragraph::new(app.input.as_str())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Input (Romaji)"));
    f.render_widget(input, chunks[0]);

    f.set_cursor_position((
        chunks[0].x + app.cursor_position as u16 + 1,
        chunks[0].y + 1,
    ));

    let output_text = app.get_output();
    let mode_str = match app.output_mode {
        KanaType::Hiragana => "Hiragana",
        KanaType::Katakana => "Katakana",
    };

    let output = Paragraph::new(output_text)
        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).title(format!("Output ({})", mode_str)))
        .wrap(Wrap { trim: false });
    f.render_widget(output, chunks[1]);

    let help_text = vec![Line::from(vec![
        Span::styled("Tab", Style::default().fg(Color::Yellow)),
        Span::raw(": Toggle Hiragana/Katakana | "),
        Span::styled("Delete", Style::default().fg(Color::Yellow)),
        Span::raw(": Clear | "),
        Span::styled("Ctrl+C", Style::default().fg(Color::Yellow)),
        Span::raw(": Copy | "),
        Span::styled("Esc/Ctrl+Q", Style::default().fg(Color::Yellow)),
        Span::raw(": Quit"),
    ])];

    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL).title("Help"));
    f.render_widget(help, chunks[2]);
}