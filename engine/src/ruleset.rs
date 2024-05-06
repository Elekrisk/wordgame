use std::collections::HashMap;

use crate::board::{Tile, TileType};
use rand::{seq::SliceRandom, thread_rng};


pub struct Ruleset {
    letterPile: LetterPile,
    wordValidator: WordValidator
}

pub struct LetterPile {
    letters: HashMap<TileType, usize>
}

impl LetterPile {
    pub fn draw(&mut self) -> Option<TileType> {
        let mut tiles = self.letters.iter_mut().collect::<Vec<_>>();
        let choice = tiles.choose_weighted_mut(&mut thread_rng(), |(_, b)| **b).ok()?;
        *choice.1 -= 1;
        Some(*choice.0)
    }
}

pub enum WordValidator {
    Swedish,
    English,
}


