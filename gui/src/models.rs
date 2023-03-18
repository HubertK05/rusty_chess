pub struct Square(pub usize, pub usize);

impl ToString for Square {
    fn to_string(&self) -> String {
        format!("{}{}", ((self.0 + 97) as u8) as char, self.1 + 1)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum PieceType {
    #[default]
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, Debug)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Debug)]
pub struct ChessPiece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl ToString for PieceType {
    fn to_string(&self) -> String {
        match self {
            PieceType::Pawn => "P".to_string(),
            PieceType::Knight => "N".to_string(),
            PieceType::Bishop => "B".to_string(),
            PieceType::Rook => "R".to_string(),
            PieceType::Queen => "Q".to_string(),
            PieceType::King => "K".to_string(),
        }
    }
}

impl ToString for ChessPiece {
    fn to_string(&self) -> String {
        let piece_letter = self.piece_type.to_string();
        match self.color {
            Color::White => piece_letter,
            Color::Black => piece_letter.to_lowercase(),
        }
    }
}
