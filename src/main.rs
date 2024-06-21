mod chomp;
use chomp::*;
use std::io::{stdin, BufRead};

fn main() {
	let mut chomp_board = BoardState::new(7, 4);
	let (mut best_outcome, mut move_opt) = chomp_board.minimax();

	let mut stdin = stdin().lock();
	let mut str = String::new();
	loop {
		if let Some((x, y)) = move_opt {
			println!(
				"best_outcome={best_outcome} with move: ({}, {})",
				x + 1,
				y + 1
			);
			println!("Enter opponent's move as two comma/space-separated 1-indexed integers, horizontal then vertical");

			str.clear();
			stdin
				.read_line(&mut str)
				.expect("Unable to read line from stdin");

			let (x, y) = str.split_once([',', ' '])
				.expect("Should have seen a comma or space delimeter between zero-indexed x,y coords of opponent's move");
			let x = x
				.trim()
				.parse::<usize>()
				.expect("Expected positive integer value");
			let y = y
				.trim()
				.parse::<usize>()
				.expect("Expected positive integer value");

			chomp_board = chomp_board.do_move(x - 1, y - 1);

			(best_outcome, move_opt) = chomp_board.minimax();
		} else {
			println!("Game Over!");
			break;
		}
	}
}
