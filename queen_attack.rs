#[derive(Debug)]
pub struct ChessPosition{
    row : i32,
    col : i32
}
#[derive(Debug)]
pub struct Queen{
    position : ChessPosition
}
impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || file < 0 || rank > 7 || file > 7 {return None;}
        Some(ChessPosition { row: rank, col: file})
    }
}
impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }
    pub fn can_attack(&self, other: &Queen) -> bool {
        self.position.row == other.position.row || self.position.col == other.position.col 
        || (self.position.row - other.position.row).abs() == (self.position.col- other.position.col).abs()
    }
}
