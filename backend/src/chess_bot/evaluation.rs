use std::{ops::{Add, AddAssign, Sub, SubAssign}, cmp::Ordering, fmt::Display};

#[derive(Debug, Clone, Copy)]
pub struct Evaluation {
    pub material: i16,
    pub pst: i16,
    pub pawn_structure: i16,
    pub space: i16,
    pub king_dist: i16,
}

impl Evaluation {
    pub fn total(self) -> i32 {
        self.material as i32 + self.pst as i32 + self.pawn_structure as i32 + self.space as i32 + self.king_dist as i32
    }

    pub fn new() -> Self {
        Self {
            material: 0,
            pst: 0,
            pawn_structure: 0,
            space: 0,
            king_dist: 0,
        }
    }

    pub fn with_positional_factor(self, factor: i32) -> Self {
        Self {
            material: self.material,
            pst: (self.pst as i32 * factor / 100) as i16,
            pawn_structure: (self.pawn_structure as i32 * factor / 100) as i16,
            space: (self.space as i32 * factor / 100) as i16,
            king_dist: self.king_dist,
        }
    }

    pub const MAX: Self = Evaluation {
        material: 32767,
        pst: 32767,
        pawn_structure: 32767,
        space: 32767,
        king_dist: 32767,
    };

    pub const MIN: Self = Evaluation {
        material: -32768,
        pst: -32768,
        pawn_structure: -32768,
        space: -32768,
        king_dist: -32768,
    };

    pub const MAX_MATERIAL: Self = Evaluation {
        material: 32767,
        pst: 0,
        pawn_structure: 0,
        space: 0,
        king_dist: 0,
    };

    pub const MIN_MATERIAL: Self = Evaluation {
        material: -32768,
        pst: 0,
        pawn_structure: 0,
        space: 0,
        king_dist: 0,
    };
}

impl Add for Evaluation {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            material: self.material + rhs.material,
            pst: self.pst + rhs.pst,
            pawn_structure: self.pawn_structure + rhs.pawn_structure,
            space: self.space + rhs.space,
            king_dist: self.king_dist + rhs.king_dist,
        }
    }
}

impl AddAssign for Evaluation {
    fn add_assign(&mut self, rhs: Self) {
        self.material += rhs.material;
        self.pst += rhs.pst;
        self.pawn_structure += rhs.pawn_structure;
        self.space += rhs.space;
        self.king_dist += rhs.king_dist;
    }
}

impl Sub for Evaluation {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            material: self.material - rhs.material,
            pst: self.pst - rhs.pst,
            pawn_structure: self.pawn_structure - rhs.pawn_structure,
            space: self.space - rhs.space,
            king_dist: self.king_dist - rhs.king_dist,
        }
    }
}

impl SubAssign for Evaluation {
    fn sub_assign(&mut self, rhs: Self) {
        self.material -= rhs.material;
        self.pst -= rhs.pst;
        self.pawn_structure -= rhs.pawn_structure;
        self.space -= rhs.space;
        self.king_dist -= rhs.king_dist;
    }
}

impl PartialEq for Evaluation {
    fn eq(&self, other: &Self) -> bool {
        self.total() == other.total()
    }
}

impl Eq for Evaluation {}

impl PartialOrd for Evaluation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.total().partial_cmp(&other.total())
    }
}

impl Ord for Evaluation {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total().cmp(&other.total())
    }
}

impl Display for Evaluation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "material: {}, piece-square tables: {}, pawn structure: {}, space: {}, king distance: {}, TOTAL: {}",
            self.material,
            self.pst,
            self.pawn_structure,
            self.space,
            self.king_dist,
            self.total(),
        )
    }
}
