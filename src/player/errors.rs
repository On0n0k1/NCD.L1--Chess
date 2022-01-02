pub enum ErrorResponse{
    // If movement starts in a position that has no piece.
    NoPiece,
    // If movement starts in a position owned by a rival player
    RivalPiece,
    // If target position is invalid for given piece
    InvalidMove,
    // If the game is already over.
    GameOver,
}
