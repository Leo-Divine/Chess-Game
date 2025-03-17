use std::sync::{Arc, Mutex};

use iced::Point;
use ui::UI;
use grid::{Board, Grid};

mod ui;
mod grid;

fn main() {
    let grid: Board = Arc::new(Mutex::new(Grid::new()));
    
    let ui:UI = UI::new(Arc::clone(&grid));
    assert_eq!(ui.board.lock().unwrap()[(0, 0)], grid.lock().unwrap()[(0, 0)]);
    iced::run("Chess", UI::update, UI::view);
}

#[derive(Debug, Clone)]
pub enum Message {
    CursorEntered,
    CursorLeft,
    CursorMoved(Point),
    LeftButtonPressed,
    LeftButtonReleased,
}
