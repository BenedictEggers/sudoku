// Ben Eggers
// GNU GPL'd

// Sudoku solver, in Rust.


fn main() {
	let b = box Board::new();
	print_board(b);
}

// Print out the board (called after a solution is found, and useful for debugging)
fn print_board(b: Box<Board> ) {
	for row in range(0, 9) {
		for col in range(0, 9) {
			print!("{}", b.At(row, col))
		}
	}
}

// How we represent the board
struct Board {
	nums: [[u8, ..9], ..9]
}

impl Board {
	fn new() -> Board {
		Board{ nums: [[u8, ..9], ..9] }
	}

	fn get(&self, row: u8, col: u8) -> u8 {
		self.nums[row][col];
	}

	fn set(&mut self, row: u8, col: u8, num: u8) {
		self.nums[row][col] = num;
	}
}