mod chomp;
use chomp::*;

fn main() {
	let chomp_board = BoardState::new(3, 4, true);
	println!("{}", chomp_board.minimax());
}
