use std::collections::HashMap;
use std::sync::RwLock;

type CachedMove = (i32, Option<(usize, usize)>);

lazy_static::lazy_static! {
	static ref MEMOIZATION_CACHE: RwLock<HashMap<BoardState, CachedMove>> = RwLock::new(HashMap::new());
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct BoardState {
	inner: Vec<Vec<bool>>,
}

pub enum WinState {
	Loss,
	NoWinner,
}

impl From<WinState> for i32 {
	fn from(value: WinState) -> Self {
		match value {
			WinState::Loss => -1,
			WinState::NoWinner => 0,
		}
	}
}

impl WinState {
	pub fn ended(&self) -> bool {
		match &self {
			WinState::NoWinner => false,
			WinState::Loss => true,
		}
	}
}

impl BoardState {
	pub fn new(width: usize, height: usize) -> Self {
		let mut inner = Vec::with_capacity(width);
		inner.resize_with(width, || {
			let mut nested = Vec::with_capacity(height);
			nested.resize(height, true); // True means chocolate is present
			nested
		});

		BoardState { inner }
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
		WinState::Loss
	}

	pub fn minimax(&mut self) -> (i32, Option<(usize, usize)>) {
		{
			let cache = MEMOIZATION_CACHE.read().unwrap();
			if let Some(&(cached_result, ref best_move)) = cache.get(self) {
				if let Some((x, y)) = best_move {
					if self.inner[*x][*y] {
						// Only if piece was not already eaten!
						*self = self.do_move(*x, *y);
						return (cached_result, *best_move);
					}
				}
			}
		}

		let win_state = self.win_state();
		if win_state.ended() {
			let result: i32 = win_state.into();
			let mut cache = MEMOIZATION_CACHE.write().unwrap();
			cache.insert(self.clone(), (result, None));
			return (result, None);
		}

		let mut best_value = i32::MIN;
		let mut best_move = None;
		for ((x, y), mut mov) in self.moves() {
			let (eval, _) = mov.minimax();
			let eval = -eval;

			if eval > best_value {
				best_value = eval;
				best_move = Some((x, y));
			}
		}

		let mut cache = MEMOIZATION_CACHE.write().unwrap();
		cache.insert(self.clone(), (best_value, best_move));
		if let Some((x, y)) = best_move {
			*self = self.do_move(x, y); // Assume guided player will do the move we say
		}
		(best_value, best_move)
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

	pub fn moves(&self) -> Vec<((usize, usize), BoardState)> {
		let mut moves = Vec::new();
		let (width, height) = (self.inner.len(), self.inner[0].len());
		for x in 0..width {
			for y in 0..height {
				if (x, y) == (0, 0) || !self.inner[x][y] {
					continue; // Skip the poisoned or already-eaten piece
				}

				moves.push(((x, y), self.do_move(x, y)));
			}
		}

		moves
	}
}
