use thiserror::Error;

#[derive(Error, Debug)]
pub enum BoggleError {
    //#[error("cannot construct {rows}x{cols} boggle board from {letters}")]
    //InvalidLetters { rows: usize, cols: usize, letters: usize },

    #[error("cannot create boggle board from letters")]
    InvalidLetters(#[from] core::array::TryFromSliceError),
}


pub struct BoggleBoard<'a, const ROWS: usize, const COLS: usize> {
    letters: [[&'a str; COLS]; ROWS],
}

impl<'a, const ROWS: usize, const COLS: usize> BoggleBoard<'a,ROWS,COLS> {

    pub fn new(letters: &[&'a str]) -> Result<BoggleBoard<'a,ROWS,COLS>, BoggleError> {

	let mut letters_arr = [[""; COLS]; ROWS];
	
	for row in 0..ROWS {
	    letters_arr[row] = letters[COLS*row..COLS*(row+1)].try_into()?
	}
	
	Ok(BoggleBoard::<'a,ROWS,COLS> { letters: letters_arr })
    }
}
