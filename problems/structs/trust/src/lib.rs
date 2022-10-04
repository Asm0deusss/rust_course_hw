#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoundOutcome {
    BothCooperated,
    LeftCheated,
    RightCheated,
    BothCheated,
}

pub struct Game {
    left: Box<dyn Player>,
    right: Box<dyn Player>,
}

impl Game {
    pub fn new(left: Box<dyn Player>, right: Box<dyn Player>) -> Self {
        Self {
            left: (left),
            right: (right),
        }
    }

    pub fn left_score(&self) -> i32 {
        self.left.get_score()
    }

    pub fn right_score(&self) -> i32 {
        self.right.get_score()
    }

    pub fn play_round(&mut self) -> RoundOutcome {
        let left_move = self.left.get_choice();
        let right_move = self.right.get_choice();

        self.left.set_last_choice(right_move);
        self.right.set_last_choice(left_move);

        if left_move && right_move {
            self.left.add_score(2);
            self.right.add_score(2);
            RoundOutcome::BothCooperated
        } else if right_move {
            self.left.add_score(3);
            self.right.add_score(-1);
            RoundOutcome::LeftCheated
        } else if left_move {
            self.left.add_score(-1);
            self.right.add_score(3);
            RoundOutcome::RightCheated
        } else {
            RoundOutcome::BothCheated
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub trait Player {
    fn get_score(&self) -> i32;
    fn add_score(&mut self, add: i32);
    fn get_choice(&self) -> bool;
    fn set_last_choice(&mut self, last_choice: bool);
}

#[derive(Default)]
pub struct CheatingAgent {
    last_choice: bool,
    score: i32,
}

impl Player for CheatingAgent {
    fn get_score(&self) -> i32 {
        self.score
    }

    fn add_score(&mut self, add: i32) {
        self.score += add;
    }

    fn get_choice(&self) -> bool {
        false
    }

    fn set_last_choice(&mut self, last_choise: bool) {
        self.last_choice = last_choise;
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CooperatingAgent {
    last_choice: bool,
    score: i32,
}

impl Player for CooperatingAgent {
    fn get_score(&self) -> i32 {
        self.score
    }

    fn add_score(&mut self, add: i32) {
        self.score += add;
    }

    fn get_choice(&self) -> bool {
        true
    }

    fn set_last_choice(&mut self, last_choise: bool) {
        self.last_choice = last_choise;
    }
}
////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct GrudgerAgent {
    last_choice: bool,
    score: i32,
    is_fooled: bool,
}

impl Player for GrudgerAgent {
    fn get_score(&self) -> i32 {
        self.score
    }

    fn add_score(&mut self, add: i32) {
        self.score += add;
    }

    fn get_choice(&self) -> bool {
        !self.is_fooled
    }

    fn set_last_choice(&mut self, last_choise: bool) {
        self.last_choice = last_choise;
        if !last_choise {
            self.is_fooled = true;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct CopycatAgent {
    last_choice: bool,
    score: i32,
}

impl Player for CopycatAgent {
    fn get_score(&self) -> i32 {
        self.score
    }

    fn add_score(&mut self, add: i32) {
        self.score += add;
    }

    fn get_choice(&self) -> bool {
        self.last_choice
    }

    fn set_last_choice(&mut self, last_choise: bool) {
        self.last_choice = last_choise;
    }
}
impl Default for CopycatAgent {
    fn default() -> Self {
        CopycatAgent {
            last_choice: true,
            score: 0,
        }
    }
}
////////////////////////////////////////////////////////////////////////////////

pub struct DetectiveAgent {
    last_choice: bool,
    score: i32,
    moves: [bool; 4],
    cur_move: usize,
    is_fooled: bool,
}

impl Player for DetectiveAgent {
    fn get_score(&self) -> i32 {
        self.score
    }

    fn add_score(&mut self, add: i32) {
        self.score += add;
    }

    fn get_choice(&self) -> bool {
        if self.cur_move < 4 {
            self.moves[self.cur_move]
        } else if self.is_fooled {
            self.last_choice
        } else {
            false
        }
    }

    fn set_last_choice(&mut self, last_choise: bool) {
        self.last_choice = last_choise;
        if !last_choise {
            self.is_fooled = true;
        }
        self.cur_move += 1;
    }
}
impl Default for DetectiveAgent {
    fn default() -> Self {
        DetectiveAgent {
            last_choice: true,
            score: 0,
            moves: [true, false, true, true],
            cur_move: 0,
            is_fooled: false,
        }
    }
}
