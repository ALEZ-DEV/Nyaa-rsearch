#[derive(Debug)]
pub enum NyaaError {
    ImpossibleNext,
    ImpossiblePrevious,
}

impl std::error::Error for NyaaError {}

impl std::fmt::Display for NyaaError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            NyaaError::ImpossibleNext => {
                write!(f, "You have surely try to go higher page than the max page")
            }
            NyaaError::ImpossiblePrevious => write!(
                f,
                "You have surely try to go lower than page 1 (what are you doing bro??!)"
            ),
        }
    }
}
