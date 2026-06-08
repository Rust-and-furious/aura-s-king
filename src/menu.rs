use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io;

pub struct MenuOption {
    pub label: String,
    pub is_special: bool,
}

impl MenuOption {
    pub fn new(label: impl Into<String>) -> Self {
        MenuOption {
            label: label.into(),
            is_special: false,
        }
    }

    pub fn special(label: impl Into<String>) -> Self {
        MenuOption {
            label: label.into(),
            is_special: true,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MenuResult {
    Selected(usize),
    Cancelled,
    Save,
    Quit,
}

pub fn select_from_menu(options: Vec<MenuOption>, header: &str) -> io::Result<MenuResult> {
    let mut selected = 0;

    loop {
        disable_raw_mode().ok();
        draw_menu(header, &options, selected)?;

        enable_raw_mode()?;
        let event = match event::read() {
            Ok(ev) => ev,
            Err(e) => {
                let _ = disable_raw_mode();
                return Err(e);
            }
        };

        if let Event::Key(key) = event {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Up => {
                        selected = if selected == 0 {
                            options.len() - 1
                        } else {
                            selected - 1
                        };
                    }
                    KeyCode::Down => {
                        selected = (selected + 1) % options.len();
                    }
                    KeyCode::Enter => {
                        disable_raw_mode()?;
                        return Ok(MenuResult::Selected(selected));
                    }
                    KeyCode::Esc => {
                        disable_raw_mode()?;
                        return Ok(MenuResult::Cancelled);
                    }
                    KeyCode::Char('s') | KeyCode::Char('S') => {
                        disable_raw_mode()?;
                        return Ok(MenuResult::Save);
                    }
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        disable_raw_mode()?;
                        return Ok(MenuResult::Quit);
                    }
                    _ => {}
                }
            }
        }
    }
}

fn draw_menu(header: &str, options: &[MenuOption], selected: usize) -> io::Result<()> {
    print!("\x1B[2J\x1B[1;1H");

    if !header.is_empty() {
        println!("{}", header);
    }

    for (i, option) in options.iter().enumerate() {
        let is_selected = i == selected;
        let highlight = if is_selected { ">" } else { " " };
        let index_str = if option.is_special {
            "[0]".to_string()
        } else {
            format!("[{}]", i + 1)
        };

        if is_selected {
            println!("{} \x1B[1;36m{}\x1B[0m {}", highlight, index_str, option.label);
        } else {
            println!("{} \x1B[32m{}\x1B[0m {}", highlight, index_str, option.label);
        }
    }

    println!("\n\x1B[90m(↑/↓ pour naviguer, Entrée pour confirmer, S=Sauvegarder, Q=Quitter)\x1B[0m");

    Ok(())
}


