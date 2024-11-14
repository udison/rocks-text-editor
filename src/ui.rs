use std::rc::Rc;


use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame
};

use crate::app::App;

pub fn app_layout(frame: &mut Frame) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // Titlebar
            Constraint::Min(7),    // Editor
            Constraint::Length(1), // Footer
        ])
        .split(frame.area())
}

pub fn render_title_bar(frame: &mut Frame, app: &App, block: Rect) {
    let title_block = Block::default()
        .borders(Borders::BOTTOM)
        .style(Style::default());

    let app_name = Span::styled(
        app.title.clone() + " ",
        Style::default().fg(Color::Black)
    );
    let version = Span::styled(
        app.version.clone(),
        Style::default().fg(Color::DarkGray)
    );
    let title_text = Text::from(Line::from(vec![app_name, version]));
    let title = Paragraph::new(title_text)
        .style(Style::default().bg(Color::Gray))
        .block(title_block);

    frame.render_widget(title, block);
}

pub fn render_text_editor(frame: &mut Frame, app: &App, block: Rect) {
    let text = Text::from(app.text.clone()).style(Style::default().fg(Color::White));
    let text_widget = Paragraph::new(text)
        .style(Style::default());

    frame.render_widget(text_widget, block);
}