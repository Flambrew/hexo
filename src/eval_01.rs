use std::{collections::HashMap};

use crate::board::{AXES, Board, Coord, PLAYER_COUNT, Tile};

/* This bot uses a very simple weighting scheme where each of the nineteen 
 * valid threat possibilities is weighted linearly based on a very surface
 * level review of what they could be worth. Minimax to a specified depth
 * decides the bot's decision for a given move. The board assessment phase
 * double(etc)-counts for each pattern center. 
 */

pub const BASE_PATTERN_COUNT: usize = 21;
pub const BASE_PATTERNS: [&str; BASE_PATTERN_COUNT] = [
    "____XX____",
    "O___XX____",
    "O___XX___O",
    "O__XX____",
    "O__XX___O",
    "O__XX__O",
    "O_XX____",
    "O_XX___O",
    "OXX____",
    "___XXX___",
    "O__XXX___",
    "O__XXX__O",
    "O_XXX___",
    "O_XXX__O",
    "OXXX___",
    "__XXXX__",
    "O_XXXX__",
    "O_XXXX_O",
    "OXXXX__",
    "_XXXXX_",
    "OXXXXX_",
];

pub fn pattern_permutations(line: &str) -> [String; 4] {
    let base: String = String::from(line);
    let swapped: String = line.chars().map(|c| match c {
        'X' => 'O',
        'O' => 'X',
        other => other,
    }).collect();
    let reversed = base.chars().rev().collect();
    let swapped_reversed = swapped.chars().rev().collect();
    
    [base, swapped, reversed, swapped_reversed]
}

pub struct Eval {
    weights: [f32; BASE_PATTERN_COUNT],
}

impl Eval {
    pub fn new() -> Eval {
        Eval {
            weights: [
                09.0, 08.0, 05.0, 07.0, 04.0, 02.0, 06.0, 01.0, 03.0, 
                15.0, 14.0, 12.0, 13.0, 10.0, 12.0, 
                19.0, 18.0, 16.0, 17.0,
                21.0, 20.0,
            ],
        }
    }

    pub fn new_from(weights: [f32; BASE_PATTERN_COUNT]) -> Eval {
        Eval {
            weights: weights,
        }
    }

    fn identify(line: String) -> Option<usize> {
        for (i, pattern) in BASE_PATTERNS.iter().enumerate() {
            for permutation in pattern_permutations(pattern) {
                if line.contains(&permutation) {
                    return Some(i);
                }
            }
        }
        
        None
    }

    fn assess(board: &Board) -> [[isize; PLAYER_COUNT]; BASE_PATTERN_COUNT] {
        let tiles: &HashMap<Coord, Tile> = board.tiles();
        let mut out: [[isize; PLAYER_COUNT]; BASE_PATTERN_COUNT] = [[0; PLAYER_COUNT]; BASE_PATTERN_COUNT];
        for (coord, tile) in tiles {
            for (dx, dy) in AXES {
                let mut line: String = String::with_capacity(12);
                for i in -5..=5 {
                    match tiles.get(&Coord::new(coord.xpos() + dx * i, coord.ypos() + dy * i)) {
                        Some(next) => match next {
                            Tile::X => line.insert((i + 5) as usize, 'X'),
                            Tile::O => line.insert((i + 5) as usize, 'O'),
                        },
                        None => line.insert((i + 5) as usize, '_'),
                    }
                }

                match Self::identify(line) {
                    Some(pattern) => out[pattern][match tile {
                        Tile::X => 1,
                        Tile::O => 0,
                    }] += 1,
                    None => {},
                };            
            }
        }

        out
    }

    pub fn evaluate(&self, board: &Board) -> f32 {
        match board.winner() {
            Some(winner) => match winner {
                Tile::X => f32::NEG_INFINITY,
                Tile::O => f32::INFINITY,
            },
            None => {
                let assessment: [[isize; PLAYER_COUNT]; BASE_PATTERN_COUNT] = Self::assess(board);
                let mut out: f32 = 0.0;
                for i in 0..BASE_PATTERN_COUNT {
                    out += self.weights[i] * (assessment[i][0] - assessment[i][1]) as f32;
                }
                
                out
            }
        }
    }
}


