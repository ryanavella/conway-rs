//! Conway's Game of Life, in Rust.

use std::io::{stdout, Write};
use std::thread;
use std::time;

use crossterm::{
    cursor,
    style::{self, Colorize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};

fn main() -> Result<()> {
    let mut stdout = stdout();
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))?
        .execute(cursor::Hide)?;

    let mut board = [[0_u8; 32]; 32];
    board[board.len() / 2][board.len() / 2 - 1] = 1;
    board[board.len() / 2][board.len() / 2] = 1;
    board[board.len() / 2 - 1][board.len() / 2] = 1;
    board[board.len() / 2][board.len() / 2 + 1] = 1;
    board[board.len() / 2 + 1][board.len() / 2 + 1] = 1;

    stdout.queue(terminal::Clear(terminal::ClearType::All))?;
    for y in 1..(board.len() - 1) {
        for x in 1..(board.len() - 1) {
            if board[x][y] == 1 {
                stdout
                    .queue(cursor::MoveTo(x as u16, y as u16))?
                    .queue(style::PrintStyledContent("█".white()))?;
            }
        }
    }

    loop {
        let tick_beg = time::Instant::now();
        stdout.flush()?;

        let board_old = board;
        for y in 1..(board.len() - 1) {
            for x in 1..(board.len() - 1) {
                let sum = board_old[y - 1][x - 1]
                    + board_old[y][x - 1]
                    + board_old[y + 1][x - 1]
                    + board_old[y - 1][x]
                    + board_old[y + 1][x]
                    + board_old[y - 1][x + 1]
                    + board_old[y][x + 1]
                    + board_old[y + 1][x + 1];
                board[y][x] = if sum == 3 || (sum == 2 && board[y][x] == 1) {
                    if board_old[y][x] == 0 {
                        stdout
                            .queue(cursor::MoveTo(x as u16, y as u16))?
                            .queue(style::PrintStyledContent("█".white()))?;
                    }
                    1
                } else {
                    if board_old[y][x] == 1 {
                        stdout
                            .queue(cursor::MoveTo(x as u16, y as u16))?
                            .queue(style::PrintStyledContent(" ".white()))?;
                    }
                    0
                };
            }
        }

        let diff = 100_000_000 - (tick_beg.elapsed().as_nanos() as i128);

        if diff > 0 {
            thread::sleep(time::Duration::new(0, diff as u32))
        }
    }
}
