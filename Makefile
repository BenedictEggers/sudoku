all: rust_doku

rust_doku: sudoku.rs
	rustc sudoku.rs

clean:
	rm -f sudoku