use std::cmp::max;

pub struct BoardState {
	inner: Vec<Vec<bool>>,
	is_max_player: bool,
}

impl Clone for BoardState {
	fn clone(&self) -> Self {
		BoardState {
			inner: self.inner.clone(),
			is_max_player: self.is_max_player,
		}
	}
}

pub enum WinState {
	MaxWin,
	MaxLoss,
	NoWinner,
}

impl From<WinState> for i32 {
	fn from(value: WinState) -> Self {
		match value {
			WinState::MaxWin => 1,
			WinState::MaxLoss => -1,
			WinState::NoWinner => 0,
		}
	}
}

impl WinState {
	pub fn ended(&self) -> bool {
		match &self {
			WinState::NoWinner => false,
			WinState::MaxLoss | WinState::MaxWin => true,
		}
	}
}

impl BoardState {
	pub fn new(width: usize, height: usize, max_first: bool) -> Self {
		let mut inner = Vec::with_capacity(width);
		inner.resize_with(width, || {
			let mut nested = Vec::with_capacity(height);
			nested.resize(height, true); // True means chocolate is present
			nested
		});

		BoardState {
			inner,
			is_max_player: max_first,
		}
	}

	pub fn win_state(&self) -> WinState {
		let (width, height) = (self.inner.len(), self.inner[0].len());
		for x in 0..width {
			for y in 0..height {
				if (x != 0 || y != 0) && self.inner[x][y] {
					// There is a piece of non-poisonous chocolate left
					return WinState::NoWinner;
				}
			}
		}
		// Only the poisonous piece remains
		if self.is_max_player {
			WinState::MaxLoss
		} else {
			WinState::MaxWin
		}
	}

	pub fn minimax(&self) -> i32 {
		let win_state = self.win_state();
		if win_state.ended() {
			return win_state.into();
		}

		let mut alpha = -1;
		for mov in self.moves().iter() {
			alpha = max(alpha, -mov.minimax());
		}
		alpha
	}

	pub fn do_move(&self, x: usize, y: usize) -> BoardState {
		let mut new_state = self.clone();
		let (width, height) = (self.inner.len(), self.inner[0].len());

		for j in x..width {
			for k in y..height {
				new_state.inner[j][k] = false;
			}
		}

		new_state
	}

	pub fn moves(&self) -> Vec<BoardState> {
		let mut moves = Vec::new();
		let (width, height) = (self.inner.len(), self.inner[0].len());
		for x in 0..width {
			for y in 0..height {
				if (x, y) == (0, 0) || !self.inner[x][y] {
					continue; // Skip the poisoned or already-eaten piece
				}

				moves.push(self.do_move(x, y));
			}
		}

		moves
	}
}
