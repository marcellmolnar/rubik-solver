use super::common_functions::{rotation_to_string, rotate_cube, undo_move};

use crate::rubikscube::RubiksCube;

const MAX_SOLVE_DEPTH: usize = 15;

// enum to hold two modes
pub enum SolveMode {
    BruteForceRecursive,
    BruteForceSpreadOut,
}

// return string of the moves from vector of integers
fn moves_to_string(moves: &Vec<i32>) -> String {
    let mut result = "".to_string();
    for i in 0..moves.len() {
        result.push_str(&rotation_to_string(moves[i]));
        result.push_str(" ");
    }
    return result;
}

fn solve_cube_brute_force_recursive(cube: &mut RubiksCube) -> String {
    fn brute_force_one_step(cube: &mut RubiksCube, last_moves: &mut Vec<i32>) -> String {
        for rot in 0..12 {
            rotate_cube(cube, rot);
            last_moves.push(rot);
            if cube.is_solved() {
                return moves_to_string(last_moves);
            }
            if last_moves.len() < MAX_SOLVE_DEPTH {
                brute_force_one_step(cube, last_moves);
            }
            last_moves.pop();
            undo_move(cube, rot);
        }
        return "".to_string();
    }

    let solution = brute_force_one_step(cube, &mut vec![]);
    if solution != "" {
        return solution;
    }
    return "not solved".to_string();
}

fn solve_cube_brute_force_spread_out(cube: &mut RubiksCube) -> String {
    if cube.is_solved() {
        return "already solved".to_string();
    }
    
    // vector of moves initially containing a zero
    let mut moves: Vec<i32> = vec![0];
    let mut disable_remove = false;

    // while there are still moves in the vector
    while moves.len() > 0 {
        // get the last move from the vector
        let last_move = moves[moves.len() - 1];

        // rotate the cube by the last move
        rotate_cube(cube, last_move);

        //println!("{}", moves_to_string(&moves));
        // if the cube is solved, return the moves
        if cube.is_solved() {
            return moves_to_string(&moves);
        }

        let mut had_fallback: bool = false;

        if !disable_remove {
            // remove last until the last is 11
            while moves.len() > 0 && moves[moves.len() - 1] == 11 {
                undo_move(cube, moves[moves.len() - 1]);
                moves.pop();
                had_fallback = true;
            }
        }
        else {
            disable_remove = false;
        }

        let moves_len = moves.len();
        // if there was a fallback, increment the last move and continue
        if had_fallback {
            // if there are no moves left, break
            if moves.is_empty() {
                break;
            }
            
            undo_move(cube, moves[moves.len() - 1]);
            moves[moves_len - 1] += 1;
            if moves[moves_len - 1] == 11 && moves_len < MAX_SOLVE_DEPTH {
                disable_remove = true;
            }
            continue;
        }

        if moves.len() < MAX_SOLVE_DEPTH {
            moves.push(0);
        }
        else {
            undo_move(cube, last_move);
            moves[moves_len - 1] += 1;
            if moves[moves_len - 1] == 11 && moves_len < MAX_SOLVE_DEPTH {
                disable_remove = true;
            }
        }
    }

    return "not solved".to_string();
}

pub fn solve_cube(cube: &mut RubiksCube, mode: SolveMode) -> String {
    if cube.is_solved() {
        return "".to_string();
    }

    match mode {
        SolveMode::BruteForceRecursive =>
        return solve_cube_brute_force_recursive(cube),
    SolveMode::BruteForceSpreadOut =>
        return solve_cube_brute_force_spread_out(cube)
    }
}
