mod chomp;
use chomp::*;

fn main() {
	let chomp_board = BoardState::new(4, 4, true);
	println!("{}", chomp_board.minimax());
}
