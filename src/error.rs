#[derive(Debug)]
pub enum GameError {
    // TODO Implement.
}

pub type GameResult<T = ()> = Result<T, GameError>;
