use wasm_bindgen::prelude::wasm_bindgen;

const MAX_STEP: i32 = 9;

#[wasm_bindgen]
pub fn mancala_operator(flag: i32, status: &[i32]) -> i32 {
    let mut max_expectation = f32::MIN;
    let mut best_action = -1;

    for i in 0..6 {
        let mut next_situation = GameSituation::from(flag, &status);

        match next_situation.act(flag * 10 + i + 1) {
            ILLEGAL => {
                continue;
            }
            _ => {}
        }

        let current_expectation = decide(&next_situation, flag, MAX_STEP);
        if current_expectation > max_expectation {
            max_expectation = current_expectation;
            best_action = flag * 10 + i + 1;
        }
    }

    return if best_action == -1 {
        // expected not to reach
        flag * 10 + 1
    } else {
        best_action
    };
}

fn decide(game_situation: &GameSituation, decide_for: i32, remain_step: i32) -> f32 {
    if game_situation.ended || remain_step == 0 {
        return value_for(&game_situation.board, decide_for);
    }

    let mut values = Vec::new();
    for i in 0..6 {
        let mut next_situation =
            GameSituation::from(game_situation.actor, &game_situation.board);

        match next_situation.act(next_situation.actor * 10 + i + 1) {
            ILLEGAL => {
                continue;
            }
            _ => {}
        }
        values.push(decide(&next_situation, decide_for, remain_step - 1));
    }


    if game_situation.actor == decide_for {
        let mut max = f32::MIN;
        for value in values {
            if value > max {
                max = value;
            }
        }
        max
    } else {
        let mut min = f32::MAX;
        for value in values {
            if value < min {
                min = value;
            }
        }
        min
    }
}

fn value_for(status: &[i32], decide_for: i32) -> f32 {
    match decide_for {
        1 => { value(status) }
        _ => { -value(status) }
    }
}

// E(score) for player one
fn value(status: &[i32]) -> f32 {
    (status[PLAYER_1_SCORE_HOLE] - status[PLAYER_2_SCORE_HOLE]) as f32
}

const HOLE_NUMBER: usize = 14;

pub struct GameSituation {
    pub actor: i32,
    board: [i32; HOLE_NUMBER],
    ended: bool,
}

const PLAYER_1_SCORE_HOLE: usize = 6;
const PLAYER_2_SCORE_HOLE: usize = 13;


const ENDED: i32 = 15000;
const NOT_ENDED: i32 = 20000;
const ILLEGAL: i32 = 30000;

impl GameSituation {
    pub fn new(first_actor: i32) -> GameSituation {
        let mut situation = GameSituation {
            actor: first_actor,
            board: [4; HOLE_NUMBER],
            ended: false,
        };

        situation.board[PLAYER_1_SCORE_HOLE] = 0;
        situation.board[PLAYER_2_SCORE_HOLE] = 0;

        situation
    }

    pub fn from(first_actor: i32, status: &[i32]) -> GameSituation {
        let mut situation = GameSituation {
            actor: first_actor,
            board: [0; HOLE_NUMBER],
            ended: false,
        };

        for i in 0..14 {
            situation.board[i] = status[i];
        }

        situation
    }

    pub fn act(&mut self, action: i32) -> i32 {
        let actor = action / 10;
        let hole_index = (action % 10 + (actor - 1) * 7 - 1) as usize;

        // the game should not be ended
        if self.ended {
            return ILLEGAL;
        }

        // the actor should be correct
        if actor != self.actor {
            return ILLEGAL;
        }

        // the hole should not be empty
        if self.board[hole_index] == 0 {
            return ILLEGAL;
        }

        // get all pieces form this hole
        let mut mancala_pieces = self.board[hole_index];
        self.board[hole_index] = 0;

        // sow!
        let mut current_hole = Self::next_hole(hole_index);
        while mancala_pieces > 0 {
            // if opponent's score hole, skip it
            if self.is_opponent_score_hole(current_hole) {
                current_hole = Self::next_hole(current_hole);
            }
            // else, sow one piece in current hole
            self.board[current_hole] += 1;
            mancala_pieces -= 1;

            // if this is the last piece
            if mancala_pieces == 0 {
                // move again
                if self.can_move_again(current_hole) {
                    self.actor = 3 - self.actor;
                }
                // critical hit
                self.try_critical_hit(current_hole);
            }
            current_hole = Self::next_hole(current_hole);
        }

        // change actor
        self.actor = 3 - self.actor;

        if self.try_end() {
            ENDED
        } else {
            NOT_ENDED
        }
    }

    fn next_hole(hole_index: usize) -> usize {
        (hole_index + 1usize) % HOLE_NUMBER
    }

    fn is_opponent_score_hole(&self, hole_index: usize) -> bool {
        self.actor == 1 && hole_index == PLAYER_2_SCORE_HOLE ||
            self.actor == 2 && hole_index == PLAYER_1_SCORE_HOLE
    }

    fn can_move_again(&self, hole_index: usize) -> bool {
        self.actor == 1 && hole_index == PLAYER_1_SCORE_HOLE ||
            self.actor == 2 && hole_index == PLAYER_2_SCORE_HOLE
    }

    fn try_critical_hit(&mut self, hole_index: usize) {
        if self.is_my_six_hole(hole_index) &&
            self.board[hole_index] == 1 &&
            self.board[self.opposite_hole(hole_index)] > 0 {
            self.board[self.my_score_hole()] += 1 + self.board[self.opposite_hole(hole_index)];
            self.board[hole_index] = 0;
            self.board[self.opposite_hole(hole_index)] = 0;
        }
    }

    fn my_score_hole(&self) -> usize {
        if self.actor == 1 {
            PLAYER_1_SCORE_HOLE
        } else {
            PLAYER_2_SCORE_HOLE
        }
    }

    fn is_my_six_hole(&self, hole_index: usize) -> bool {
        ((self.actor - 1) * 7) as usize <= hole_index &&
            hole_index <= ((self.actor - 1) * 7 + 6) as usize
    }

    fn opposite_hole(&self, hole_index: usize) -> usize {
        assert!(
            hole_index != PLAYER_1_SCORE_HOLE &&
                hole_index != PLAYER_2_SCORE_HOLE
        );
        12 - hole_index
    }

    fn try_end(&mut self) -> bool {
        let mut piece_number = 0;
        for i in 0..6 {
            piece_number += self.board[i];
        }
        if piece_number == 0 {
            for i in 7..13 {
                self.board[PLAYER_2_SCORE_HOLE] += self.board[i];
                self.board[i] = 0;
                self.ended = true;
            }
        }
        piece_number = 0;
        for i in 7..13 {
            piece_number += self.board[i];
        }
        if piece_number == 0 {
            for i in 0..6 {
                self.board[PLAYER_1_SCORE_HOLE] += self.board[i];
                self.board[i] = 0;
                self.ended = true;
            }
        }
        self.ended
    }
}

#[cfg(test)]
mod tests {}
