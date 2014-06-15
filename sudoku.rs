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
			print!("{}", b.get(row, col))
		}
		println!("");
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

	fn get(&self, row: uint, col: uint) -> u8 {
		self.nums[row][col];
	}

	fn set(&mut self, row: uint, col: uint, num: u8) {
		self.nums[row][col] = num;
	}
}