use std::io;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    Terminal,
};


// TOOD: make `Chunks` more advanced by having it store a list of layouts and having a split that takes in a size/vertical. 
// Basically improve tui's layout functionality 
struct Chunks {
    layout: Vec<tui::layout::Rect>,
    size: tui::layout::Rect,
}

impl Chunks {
    fn set_size(&mut self, size: tui::layout::Rect, direction : Direction, constraints: Vec<Constraint>) {
        self.layout = Layout::default()
            .direction(direction)
            .margin(2)
            .constraints(constraints,
            ).split(size);
        self.size = size;
    }
}

impl Default for Chunks {
    fn default() -> Self {
        Chunks {
            layout: Layout::default().split(tui::layout::Rect::default()),
            size: tui::layout::Rect::default(),
        }
    }
}

mod interface;
use interface::{Copyright, Entries, Lists};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // prepares the terminal for use
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    enable_raw_mode()?;

    let db_file = std::fs::read_to_string("./data/db.json")?;
    // let file = File::open("./data/db.json")?;
    let data: serde_json::Value =
        serde_json::from_str(&db_file[..]).expect("File not formatted correctly");

    let mut menu = Lists::default();
    let mut entry = Entries::default();
    let copyright = Copyright::from("Springwood CLI 2021 - all rights reserved");
    let mut chunks = Chunks::default();
    let mut side_test = Chunks::default();
    let mut side_open = false;
    
    loop {
        terminal.draw(|rect| {
            // gets terminal size and splits it into vertical chunks


            chunks.set_size(rect.size(), Direction::Vertical, [
                Constraint::Length(3),
                Constraint::Min(2),
                Constraint::Length(3)].into());
                
            if side_open {
                side_test.set_size(chunks.layout[1], Direction::Horizontal, [Constraint::Percentage(50),Constraint::Percentage(50)].into());
            } else {
                side_test.set_size(chunks.layout[1], Direction::Horizontal, [Constraint::Percentage(100)].into());
            }

            // retrieves lists and saves them to display on the menu
            menu.lists = data["lists"].as_object().unwrap().keys().cloned().collect();

            // retrieves the entries and sets the `entry` variable to them
            entry.entries = data["lists"]
                .get(menu.repr())
                .expect("1")
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_str().unwrap().to_string())
                .collect();

            // renders the widgets
            rect.render_widget(menu.render_menu(), chunks.layout[0]);
            rect.render_stateful_widget(
                entry.render_entries(menu.repr()),
                side_test.layout[0],
                &mut entry.state.clone(),
            );

            if side_open {
                rect.render_widget(copyright.render_copyright(), side_test.layout[1]);
            }
        
            rect.render_widget(copyright.render_copyright(), chunks.layout[2]);
        })?;

        // event recognition
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Char('Q') => break,
                KeyCode::Left => {
                    menu.previous();
                    entry.select(0);
                }
                KeyCode::Right => {
                    menu.next();
                    entry.select(0);
                }
                KeyCode::Up => entry.previous(),
                KeyCode::Down => entry.next(),
                KeyCode::Enter => {
                    side_open = !side_open;
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    terminal.set_cursor(0, 0)?;
    terminal.show_cursor()?;
    terminal.clear()?;
    println!();

    Ok(())
}

// TODO: Implement this json structure
/*
{
    "lists" : {
        "inbox" : ["entry 1", "entry 2", "etc"],
        "todo" : ["thing 1", "thing 2"],
        "etc": []
    },

    "settings" : {
        "do_this": true,
        "have_this" : false
    }
}
*/
// can be edited in config with...
// springwood --config do_this false have_this true
