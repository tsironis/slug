use crate::app::Mode;

use super::app::{App, TaskType};
use chrono::{Datelike, Month, NaiveDate};
use ratatui::{
    prelude::*,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph},
};

pub fn draw(f: &mut Frame, app: &App) {
    match app.mode {
        Mode::Normal => draw_normal_mode(f, app),
        Mode::Future => draw_future_mode(f),
        Mode::Insert | Mode::Command | Mode::Plan | Mode::Reflect => draw_misc_mode(f, app),
    }
}

fn draw_misc_mode(f: &mut Frame, app: &App) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(1)])
        .split(f.area());

    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(main_chunks[0]);

    draw_monthly_log(f, app, content_chunks[0]);
    draw_tasks(f, app, content_chunks[1]);
    draw_command_line(f, app, main_chunks[0], main_chunks[1]);
}

fn draw_normal_mode(f: &mut Frame, app: &App) {
    let normal_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(5), Constraint::Percentage(95)])
        .split(f.area());
    draw_active_mode(f, app, normal_chunks[0]);

    draw_normal_screen(f, app, normal_chunks[1]);
}

fn draw_future_mode(frame: &mut Frame) {
    // Get all months
    let months: Vec<Month> = (1u8..=12).filter_map(|m| Month::try_from(m).ok()).collect();

    // Create three rows
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(frame.area());

    // Counter for months
    let mut month_index = 0;

    // For each row
    for row in rows.iter() {
        // Create two columns in this row
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .split(*row);

        // Fill each column in this row
        for column in columns.iter() {
            if month_index < months.len() {
                let month = months[month_index];
                let block = Block::default().borders(Borders::ALL).title(format!(
                    " {} {} ",
                    month.name(),
                    "2025"
                ));

                // Create a paragraph for the month
                let paragraph = Paragraph::new(format!("\n  Month #{}", month_index + 1))
                    .style(Style::default().fg(Color::White))
                    .block(block);

                frame.render_widget(paragraph, *column);
                month_index += 1;
            }
        }
    }
}

fn draw_active_mode(f: &mut Frame, app: &App, area: Rect) {
    let widget = Paragraph::new(format!("{} Mode", app.mode))
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title_alignment(Alignment::Center),
        );
    f.render_widget(widget, area);
}
fn draw_normal_screen(f: &mut Frame, app: &App, area: Rect) {
    let current_date = NaiveDate::from_ymd_opt(
        app.current_date.year(),
        app.current_date.month(),
        app.current_date.day(),
    )
    .unwrap();

    let block = Block::default()
        .title(format!(" Today {}", current_date.format("%B %d, %Y")))
        .borders(Borders::ALL);
    let tasks = app.tasks.get(&current_date).cloned().unwrap_or_default();
    let items: Vec<ListItem> = tasks
        .iter()
        .map(|task| {
            let symbol = match task.task_type {
                TaskType::Todo => {
                    if task.completed {
                        "×"
                    } else {
                        "•"
                    }
                }
                TaskType::Event => "○",
                TaskType::Note => "-",
            };
            ListItem::new(format!("{} {}", symbol, task.content))
        })
        .collect();

    let tasks_list = List::new(items)
        .block(block)
        .style(Style::default().fg(Color::LightYellow));

    f.render_widget(tasks_list, area);
}

fn draw_monthly_log(f: &mut Frame, app: &App, area: Rect) {
    let current_date = app.current_date;
    let title = Block::default()
        .title(format!("{}", current_date.format("%B %Y")))
        .borders(Borders::ALL);

    let dates: Vec<String> = (1..=31)
        .map(|day| {
            if let Some(date) =
                NaiveDate::from_ymd_opt(current_date.year(), current_date.month(), day)
            {
                let has_tasks = app.tasks.contains_key(&date);
                let workday = date.format("%a");
                if has_tasks {
                    format!("{:2}* {}", day, workday)
                } else {
                    format!("{:2} {}", day, workday)
                }
            } else {
                "   ".to_string()
            }
        })
        .collect();

    let calendar_widget = Paragraph::new(format_calendar(&dates)).block(title);

    f.render_widget(calendar_widget, area);
}
fn draw_daily_log(f: &mut Frame, app: &App, area: Rect) {
    if let Some(current_date) = NaiveDate::from_ymd_opt(
        app.current_date.year(),
        app.current_date.month(),
        app.current_date.day(),
    ) {
        let task = app.tasks.contains_key(&current_date);
        let title = Paragraph::new(task.to_string())
            .style(Style::default().fg(Color::Yellow))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!(" Today {} ", current_date.format("%B %d, %Y")))
                    .border_type(BorderType::Rounded),
            );

        f.render_widget(title, area);
    }
}
fn draw_tasks(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title(format!(" {} ", app.current_date.format("%B %d, %Y")))
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL);

    let current_date = NaiveDate::from_ymd_opt(
        app.current_date.year(),
        app.current_date.month(),
        app.current_date.day(),
    )
    .unwrap();

    let tasks = app.tasks.get(&current_date).cloned().unwrap_or_default();
    let items: Vec<ListItem> = tasks
        .iter()
        .map(|task| {
            let symbol = match task.task_type {
                TaskType::Todo => {
                    if task.completed {
                        "×"
                    } else {
                        "•"
                    }
                }
                TaskType::Event => "○",
                TaskType::Note => "-",
            };
            ListItem::new(format!("{} {}", symbol, task.content))
        })
        .collect();

    let tasks_list = List::new(items)
        .block(block)
        .style(Style::default().fg(Color::Yellow));

    f.render_widget(tasks_list, area);
}

fn format_calendar(dates: &[String]) -> String {
    let mut calendar = String::from("");
    for chunk in dates {
        let line: String = chunk.to_string();
        calendar.push_str(&line);
        calendar.push('\n');
    }
    calendar
}

fn draw_command_line(f: &mut Frame, app: &App, area1: Rect, area2: Rect) {
    if let Mode::Command = app.mode {
        let area = area1;
        let width = area.width.min(60);
        let height = area.height.min(3);

        // Ensure we have minimum dimensions
        if width < 10 || height < 3 {
            return;
        }

        let x = area.x + (area.width.saturating_sub(width)) / 2;
        let y = area.y + (area.height.saturating_sub(height)) / 2;

        let popup_area = Rect::new(x, y, width, height);

        let command = Paragraph::new(format!(" > {}", app.input_buffer))
            .style(Style::default().fg(Color::Yellow))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Blue))
                    .title(" Command ")
                    .title_alignment(Alignment::Center),
            );

        f.render_widget(Clear, popup_area);
        f.render_widget(command, popup_area);
    } else {
        let mode = Paragraph::new(format!("{}", app.mode));
        f.render_widget(mode, area2);
    }
}
