#![warn(clippy::pedantic)]
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
        for (x, col) in board.iter().enumerate().take(board.len() - 1).skip(1) {
            if col[y] == 1 {
                let x = x.try_into().unwrap();
                let y = y.try_into().unwrap();
                stdout
                    .queue(cursor::MoveTo(x, y))?
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
                        let x = x.try_into().unwrap();
                        let y = y.try_into().unwrap();
                        stdout
                            .queue(cursor::MoveTo(x, y))?
                            .queue(style::PrintStyledContent("█".white()))?;
                    }
                    1
                } else {
                    if board_old[y][x] == 1 {
                        let x = x.try_into().unwrap();
                        let y = y.try_into().unwrap();
                        stdout
                            .queue(cursor::MoveTo(x, y))?
                            .queue(style::PrintStyledContent(" ".white()))?;
                    }
                    0
                };
            }
        }

        let elapsed = tick_beg.elapsed().as_nanos();
        let diff = 100_000_000_u128.checked_sub(elapsed);

        if let Some(diff) = diff {
            let diff = diff.try_into().unwrap();
            let duration = time::Duration::new(0, diff);
            thread::sleep(duration)
        }
    }
}
