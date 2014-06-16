// Ben Eggers
// GNU GPL'd

// Sudoku solver, in Rust.


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

	// Prints the board to stdout
	fn print(&self) {
		for col in range(0u, 9u) {
			for row in range(0u, 9u) {
				let n = self.get(row, col);
				if n != 0 {
					print!("{} ", n)
				} else {
					// We want dashes, not 0s
					print!("- ")
				}
				if (row+1) % 3 == 0 {
					// block spacing
					print!(" ")
				}
			}
			println!("")
			if (col+1) % 3 == 0 {
				// block spacing
				println!("")
			}
		}
	}

	// Solves the board. Returns true if the board can be solved and false
	// otherwise. If true was returned, the board will be in a solved state. If
	// false, the board will be in the state it was in when this method was
	// called.
	fn solve(&mut self) -> bool {
		// We're going to brute-force recursive-backtrack all the possible
		// combinations (yay NP-complete problems)
		let next = self.next_open_square();

		// Check to see if the board is complete--if it is, we're done!
		if next == (0, 0) {
			return true
		}

		// Niceties
		let next_x = next.val0();
		let next_y = next.val1();

		for num in range(1u8, 10u8) {
			if self.is_legal(next_x, next_y, num) {
				// We're allowed to put this number there, so do so
				self.set(next_x, next_y, num);

				// Then recurse to see if the board is solvable
				if self.solve() {
					// We did it!
					return true;
				}

				// Nope, keep trying
			}
		}
		
		// Couldn't solve it. Reset the considered square to 0 and return false
		self.set(next_x, next_y, 0);
		false
	}

	// Returns the next zero (row, col) on the sudoku board. "Next" is defined
	// as top coming before bottom, left coming before right. (0, 0) will be
	// returned if there are no open squares.
	fn next_open_square(&self) -> (uint, uint) {
		 for col in range(0u, 9u) {
		 	for row in range(0u, 9u) {
		 		if self.get(row, col) == 0 {
		 			return (row, col)
		 		}
		 	}
		 }

		(0, 0)
	}

	// Whether or not the passed number is legal at (row, col)
	fn is_legal(&self, row: uint, col: uint, num: u8) -> bool {
		// If the number is already there, it's possible to have it there
		if self.nums[row][col] == num {
			return true
		}

		// Check the row and column of the potential location for num
		for i in range(0u, 9u) {
			if self.get(row, i) == num || self.get(i, col) == num {
				return false
			}
		}

		// Now, check its "box" of 9 points
		let box_pt_r = row / 3 * 3;  // Woo, integer arithmetic!
		let box_pt_c = col / 3 * 3;
		for r in range(0u, 3u) {
			for c in range(0u, 3u) {
				if self.get(box_pt_r + r, box_pt_c + c) == num {
					return false
				}
			}
		}

		true
	}
}


fn main() {
	let mut b = box Board::new();
	b.print();
	b.set(0, 0, 1);
	b.set(4, 0, 2);
	b.print();
	b.solve();
	b.print();
}