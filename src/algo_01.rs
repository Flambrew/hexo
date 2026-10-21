use std::io::{self, Write};

use crate::{board::{Board, Coord, Tile}, eval_01::Eval};

/* This selector implements pure unoptimized and untuned minimax.
 */

const DEFAULT_RANGE: usize = 2;
const DEFAULT_DEPTH: usize = 3;

struct Node<'a> {
    board: Board,
    evaluation: f32,
    children: Vec<Node<'a>>,
    evaluator: &'a Eval,
    change: Option<Coord>,
}

impl<'a> Node<'a> {
    fn new(board: Board, evalulator: &'a Eval, range: usize, depth: usize) -> Node<'a> {
        let eval: f32 = evalulator.evaluate(&board);
        let mut out: Node = Node {
            board: board,
            evaluation: eval,
            children: Vec::new(),
            evaluator: evalulator,
            change: None
        };

        if depth > 0 {
            out.step_down(range, depth);
        }

        out
    }

    fn step_down(&mut self, range: usize, depth: usize) {
        if depth > 2 { print!("\n{} stepdown {} ", "    ".repeat(DEFAULT_DEPTH-depth), depth); }
        if depth == 2 { print!(".")}
        let options: Vec<Coord> = self.board.region(range).into_iter().collect();
        for coord in options {
            let mut copy: Board = self.board.clone();
            copy.play(coord);
            let eval: f32 = self.evaluator.evaluate(&copy);
            self.children.push(Node {
                board: copy,
                evaluation: eval,
                children: Vec::new(),
                evaluator: self.evaluator,
                change: Some(coord),
            });
        }
        
        if depth > 1 {
            for child in self.children.iter_mut() {
                child.step_down(range, depth - 1);
            }
        }
    }
}

fn minimax(node: &mut Node, player: &Tile) {
    if node.children.len() == 0 {
        return 
    } else {
        for child in node.children.iter_mut() {
            minimax(child, player);
        }
    }

    let mut best: usize = 0;
    let max: bool = node.board.turn().player_tile() == *player;
    if max {
        let mut val: f32 = f32::NEG_INFINITY;
        for (i, child) in node.children.iter().enumerate() {
            if child.evaluation > val {
                val = child.evaluation;
                best = i;
            }
        }
    } else {
        let mut val: f32 = f32::INFINITY;
        for (i, child) in node.children.iter().enumerate() {
            if child.evaluation < val {
                val = child.evaluation;
                best = i;
            }
        }
    }

    for i in (0..node.children.len()).rev() {
        if i != best {
            node.children.remove(i);
        }
    }    
}

pub struct Algo {
    range: usize,
    depth: usize,
}

impl Algo {
    pub fn new() -> Algo {
        Algo {
            range: DEFAULT_RANGE,
            depth: DEFAULT_DEPTH,
        }
    }

    pub fn new_from(range: usize, depth: usize) -> Algo {
        Algo {
            range: range,
            depth: depth,
        }
    }

    pub fn decide(&self, board: &Board, evaluator: &Eval) -> Option<Coord> {
        let mut tree: Node = Node::new(board.clone(), evaluator, self.range, self.depth);
        minimax(&mut tree, &board.turn().player_tile());
        if tree.children.len() == 1 {
            match tree.children[0].change {
                Some(coord) => Some(coord),
                None => None
            }
        } else {
            None
        }
    }
}