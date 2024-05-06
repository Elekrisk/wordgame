use uuid::Uuid;


pub struct Board {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>
}

pub struct Cell {
    pub bonusCell: Option<BonusCell>,
    pub tiles: Vec<Tile>,
}

pub enum BonusCell {
    TimesLetter(usize),
    TimesWord(usize),
}

pub struct Tile {
    pub tile_type: TileType,
    pub placed_by: Player
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileType {
    Letter(char),
    Blank(Option<char>),
    Stop,
    Turn(TurnDir),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum TurnDir {
    Rightwards,
    Downwards,
}

pub struct Player {
    pub id: Uuid,
    pub name: String,
}
