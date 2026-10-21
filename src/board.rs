use std::{collections::{HashMap, HashSet}, fmt};

pub const PLACEMENTS_PER_TURN: usize = 2;
pub const PLAYER_COUNT:usize = 2;
pub const PLAYER_MAPPING: [Tile; PLAYER_COUNT] = [ Tile::O, Tile::X ];
pub const AXES: [(isize, isize); 3] = [(1, 0), (0, 1), (1, 1)];

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Tile {
    X,
    O,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Coordinate {
    xpos: isize,
    ypos: isize,
}

impl Coordinate {
    pub fn new(xpos: isize, ypos: isize) -> Coordinate {
        Coordinate {
            xpos: xpos,
            ypos: ypos,
        }
    }

    pub fn xpos(&self) -> isize {
        self.xpos
    }

    pub fn ypos(&self) -> isize {
        self.ypos
    }

    pub fn neighbors(&self, depth: usize) -> HashSet<Coordinate> {
        let mut out: HashSet<Coordinate> = HashSet::new();
        let depth: isize = depth as isize;
        for i in (-depth)..=depth {
            let min: isize = if i <= 0 { -depth } else { i-depth };
            let max: isize = if i <= 0 { i+depth } else { depth };
            for j in min..=max {
                out.insert(Coordinate {
                    xpos: j,
                    ypos: i,
                });
            }
        }

        out
    }
}

struct Bounds {
    xmin: isize,
    xmax: isize,
    ymin: isize,
    ymax: isize,
}

impl Bounds {
    fn new() -> Bounds {
        Bounds {
            xmin: isize::MAX, 
            xmax: isize::MIN, 
            ymin: isize::MAX, 
            ymax: isize::MIN,
        }
    }

    fn range(&self) -> Option<Coordinate> {
        let range: Coordinate = Coordinate::new(self.xmax - self.xmin + 1, self.ymax - self.ymin + 1);
        if range.xpos < 0 || range.ypos < 0 {
            return None;
        } else {
            return Some(range);
        }
    }
}

pub struct Turn {
    player: usize,
    placement: usize,
}

impl Turn {
    fn new() -> Turn {
        Turn { 
            player: 0,
            placement: 0,
        }
    }

    fn next(&mut self) {
        self.placement += 1;
        if self.placement == PLACEMENTS_PER_TURN {
            self.placement = 0;
            self.player = (self.player + 1) % PLAYER_COUNT;
        }
    }

    pub fn player_tile(&self) -> Tile {
        PLAYER_MAPPING[self.player]
    }

    pub fn placements_left(&self) -> usize {
        PLACEMENTS_PER_TURN - self.placement
    }
}

pub struct Board {
    tiles: HashMap<Coordinate, Tile>,
    bounds: Bounds,
    turn: Turn,
}

impl Board {
    pub fn new() -> Board {
        Board {
            turn: Turn::new(),
            bounds: Bounds::new(),
            tiles: HashMap::from([(
                Coordinate {
                    xpos: 0,
                    ypos: 0,
                }, 
                Tile::X
            )]),
        }
    }

    pub fn turn(&self) -> &Turn {
        &self.turn
    }

    pub fn tiles(&self) -> &HashMap<Coordinate, Tile> {
        &self.tiles
    }

    pub fn play(&mut self, coord: Coordinate) -> bool {
        match self.tiles.contains_key(&coord) {
            true => false,
            false => {
                self.tiles.insert(coord, self.turn.player_tile());
                self.turn.next();
                true
            }
        }
    }

    pub fn region(&self, depth: usize) -> HashSet<Coordinate> {
        let mut out: HashSet<Coordinate> = HashSet::new();
        for key in self.tiles.keys() {
            for coord in key.neighbors(depth) {
                out.insert(coord);
            }
        }

        out
    }

    pub fn winner(&self) -> Option<Tile> {
        for (coord, tile) in &self.tiles {
            for (dx, dy) in AXES {
                let mut count: usize = 1;
                for i in 1..6 {
                    match self.tiles.get(&Coordinate::new(coord.xpos + dx * i, coord.ypos + dy * i)) {
                        Some(next) if next == tile => count += 1,
                        _ => break,
                    }
                }

                if count == 6 {
                    return Some(*tile);
                }
            }
        }
        
        None
    }
}

const SIGHT_RANGE: usize = 10;
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut range: Coordinate = match self.bounds.range() {
            Some(coord) => coord,
            None => return writeln!(f, "ERROR: INVALID BOARD RANGE"),
        };

        let _xmin: isize = self.bounds.xmin - SIGHT_RANGE as isize;
        let _ymin: isize = self.bounds.ymin - SIGHT_RANGE as isize;
        range.xpos += (2 * SIGHT_RANGE) as isize;
        range.ypos += (2 * SIGHT_RANGE) as isize;

        // TODO: impl

        return writeln!(f, "glue")
    }
}