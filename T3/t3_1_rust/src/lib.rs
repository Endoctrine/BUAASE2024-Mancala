use wasm_bindgen::prelude::wasm_bindgen;
use js_sys::Int32Array;

#[wasm_bindgen]
pub fn mancala_board(flag: i32, seq: &[i32], size: i32) -> Int32Array {
    let mut game_situation = GameSituation::new(seq[0] / 10);
    for i in 0..size - 1 {
        game_situation.act(seq[i as usize]);
    }
    match game_situation.act(seq[(size - 1) as usize]) {
        ILLEGAL => {
            let mut tmp = [0; 15];
            for i in 0..14 {
                tmp[i] = game_situation.board[i];
            }
            if flag == 1 {
                tmp[14] = 200 + 2 * game_situation.board[PLAYER_1_SCORE_HOLE] - 48;
            } else {
                tmp[14] = 200 - 2 * game_situation.board[PLAYER_2_SCORE_HOLE] + 48;
            }
            return Int32Array::from(&tmp[..]);
        }
        _ => {
            let mut tmp = [0; 15];
            for i in 0..14 {
                tmp[i] = game_situation.board[i];
            }

            if game_situation.ended {
                tmp[14] = 200 + game_situation.board[PLAYER_1_SCORE_HOLE] - game_situation.board[PLAYER_2_SCORE_HOLE];
            } else {
                tmp[14] = game_situation.actor;
            }
            return Int32Array::from(&tmp[..]);
        }
    }
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
mod tests {
    // use super::*;

    #[test]
    fn test_case_illegal() {
        // assert_eq!(mancala_board(1, &[
        //     11, 21, 12, 13, 25,
        //     11, 21, 12, 22, 11,
        //     23, 12, 24, 13, 11,
        //     26, 12, 25, 12
        // ], 19), 200 + 3 * 2 - 48);
    }

    #[test]
    fn test_case_ended() {
        // assert_eq!(mancala_board(2, &[
        //     11, 21, 12, 13, 25,
        //     11, 21, 12, 22, 11,
        //     23, 12, 24, 13, 11,
        //     26, 12, 25, 11, 26
        // ], 20), 200 + 16);
        // assert_eq!(mancala_board(1, &[
        //     13, 11, 23, 26, 11,
        //     25, 12, 26, 21, 13,
        //     14, 22, 12, 21, 11,
        //     23, 24, 16
        // ], 18), 200 - 2);
        // assert_eq!(mancala_board(2, &[
        //     21, 15, 22, 13, 15,
        //     23, 14, 24, 14, 25,
        //     14, 21, 12, 23, 14,
        //     22, 15, 26, 13, 25,
        //     12, 23, 14, 21, 15,
        //     16, 13, 24, 25, 16,
        //     14, 23, 15, 16, 11,
        //     22
        // ], 36), 200 - 12);
    }

    #[test]
    fn test_case_not_ended() {
        // assert_eq!(mancala_board(1, &[
        //     11, 21, 12, 13, 25,
        //     11, 21, 12, 22, 11,
        //     23, 12, 24, 13, 11,
        //     26, 12, 25, 11
        // ], 19), 2);
        // assert_eq!(mancala_board(2, &[
        //     13, 11, 23, 26, 11,
        //     25, 12, 26, 21, 13,
        //     14, 22, 12, 21, 11,
        //     23, 24
        // ], 17), 1);
        // assert_eq!(mancala_board(1, &[
        //     21, 15, 22, 13, 15,
        //     23, 14, 24, 14, 25,
        //     14, 21, 12, 23, 14,
        //     22, 15, 26, 13, 25,
        //     12, 23, 14, 21, 15,
        //     16, 13, 24, 25, 16,
        //     14, 23, 15, 16, 11
        // ], 35), 2);
    }
}
