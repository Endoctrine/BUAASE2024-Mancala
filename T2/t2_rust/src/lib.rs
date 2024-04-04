use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn mancala_result(flag: i32, seq: &[i32], size: i32) -> i32 {
    let mut game_situation = GameSituation::new(flag);
    for i in 0..size {
        match game_situation.act(seq[i as usize]) {
            ILLEGAL => {
                return ILLEGAL + i;
            }
            _ => {}
        }
        // println!("step {i} action {} :", seq[i as usize]);
        // game_situation.log();
    }

    if game_situation.ended {
        if flag == 1 {
            ENDED + game_situation.board[PLAYER_1_SCORE_HOLE] -
                game_situation.board[PLAYER_2_SCORE_HOLE]
        } else {
            ENDED + game_situation.board[PLAYER_2_SCORE_HOLE] -
                game_situation.board[PLAYER_1_SCORE_HOLE]
        }
    } else {
        if flag == 1 {
            NOT_ENDED + game_situation.board[PLAYER_1_SCORE_HOLE]
        } else {
            NOT_ENDED + game_situation.board[PLAYER_2_SCORE_HOLE]
        }
    }
}

const HOLE_NUMBER: usize = 14;

struct GameSituation {
    actor: i32,
    board: [i32; HOLE_NUMBER],
    ended: bool,
}

const PLAYER_1_SCORE_HOLE: usize = 6;
const PLAYER_2_SCORE_HOLE: usize = 13;


const ENDED: i32 = 15000;
const NOT_ENDED: i32 = 20000;
const ILLEGAL: i32 = 30000;

impl GameSituation {
    fn new(first_actor: i32) -> GameSituation {
        let mut situation = GameSituation {
            actor: first_actor,
            board: [4; HOLE_NUMBER],
            ended: false,
        };

        situation.board[PLAYER_1_SCORE_HOLE] = 0;
        situation.board[PLAYER_2_SCORE_HOLE] = 0;

        situation
    }

    fn act(&mut self, action: i32) -> i32 {
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

    // fn log(&self) {
    //     for i in 0..7 {
    //         print!("|{}\t", self.board[13 - i]);
    //     }
    //     println!("|");
    //     print!("\t");
    //     for i in 0..7 {
    //         print!("|{}\t", self.board[i]);
    //     }
    //     println!("|");
    //     println!();
    // }

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
    use super::*;

    #[test]
    fn test_case_illegal() {
        assert_eq!(mancala_result(1, &[11, 12], 2), ILLEGAL + 1);
        assert_eq!(mancala_result(1, &[
            11, 22, 12, 13, 21,
            14, 22, 16, 23, 15,
            23, 14, 22
        ], 13), ILLEGAL + 12);
        assert_eq!(mancala_result(1, &[
            11, 22, 12, 13, 21,
            14, 22, 16, 23, 15,
            23, 14, 21, 13, 24,
            16, 15, 25, 16, 15
        ], 20), ILLEGAL + 16);
        assert_eq!(mancala_result(1, &[
            11, 21, 12, 13, 25,
            11, 21, 12, 22, 11,
            23, 12, 24, 13, 11,
            26, 12, 25, 11, 26,
            11
        ], 21), ILLEGAL + 20);
    }

    #[test]
    fn test_case_ended() {
        assert_eq!(mancala_result(1, &[
            11, 21, 12, 13, 25,
            11, 21, 12, 22, 11,
            23, 12, 24, 13, 11,
            26, 12, 25, 11, 26
        ], 20), ENDED + 16);
    }

    #[test]
    fn test_case_not_ended() {
        assert_eq!(mancala_result(1, &[11, 22], 2), NOT_ENDED + 0);
        assert_eq!(mancala_result(1, &[11, 22, 12, 13], 4), NOT_ENDED + 2);
        assert_eq!(mancala_result(1, &[
            11, 22, 12, 13, 21, 14,
            22, 16, 23, 15, 23, 14
        ], 12), NOT_ENDED + 8);
        assert_eq!(mancala_result(1, &[
            11, 21, 12, 13, 25
        ], 5), NOT_ENDED + 2);
    }

    // #[test]
    // fn test_manually() {
    //     let seq = [
    //         11, 21, 12, 13, 25,
    //         11, 21, 12, 22, 11,
    //         23, 12, 24, 13, 11,
    //         26, 12, 25, 11, 26
    //     ];
    //     println!("{}", mancala_result(1, &seq, seq.len() as i32));
    // }
}
