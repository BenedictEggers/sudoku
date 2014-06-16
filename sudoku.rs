// Ben Eggers
// GNU GPL'd

// Sudoku solver, in Rust.


fn main() {
	let b = box Board::new();
	print_board(b);
}

// Print out the board (called after a solution is found, and useful for debugging)
fn print_board(b: Box<Board> ) {
	for row in range(0u, 9u) {
		for col in range(0u, 9u) {
			let n = b.get(row, col);
			if n != 0 {
				print!("{} ", n)
			} else {
				// We want dashes, not 0s
				print!("- ")
			}
			if (col+1) % 3 == 0 {
				// block spacing
				print!(" ")
			}
		}
		println!("")
		if (row+1) % 3 == 0 {
			// block spacing
			println!("")
		}
	}
}

// How we represent the board
struct Board {
	nums: [[u8, ..9], ..9]
}

impl Board {
	fn new() -> Board {
		Board{ nums: [[0u8, ..9], ..9] }
	}

	// get the number at (row, col)
	fn get(&self, row: uint, col: uint) -> u8 {
		self.nums[row][col]
	}

	// sets (row, col) to the passed number
	fn set(&mut self, row: uint, col: uint, num: u8) {
		self.nums[row][col] = num;
	}

	// Whether or not the passed number is legal at (row, col)
	fn is_legal(&self, row: uint, col: uint, num: u8) -> bool {
		// If the number is already there, it's possible to have it there
		if self.nums[row][col] == num {
			return true
		}

		for i in range(0u, 9u) {
			if self.get(row, i) == num || self.get(i, col) == num {
				return false
			}
		}



		true
	}
}