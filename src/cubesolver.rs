use super::common_functions::{rotation_to_string, rotate_cube, undo_move};

use crate::rubikscube::RubiksCube;

// return string of the moves from vector of integers
fn moves_to_string(moves: &Vec<i32>) -> String {
    let mut result = "".to_string();
    for i in 0..moves.len() {
        result.push_str(&rotation_to_string(moves[i]));
        result.push_str(" ");
    }
    return result;
}

fn brute_force_one_step(cube: &mut RubiksCube, last_moves: &mut Vec<i32>) -> String {
    for rot in 0..12 {
        rotate_cube(cube, rot);
        last_moves.push(rot);
        if cube.is_solved() {
            return moves_to_string(last_moves);
        }
        if last_moves.len() < 6 {
            brute_force_one_step(cube, last_moves);
        }
        last_moves.pop();
        undo_move(cube, rot);
    }
    return "".to_string();
}

fn solve_cube_brute_force_recursive(cube: &mut RubiksCube) -> String {
    let solution = brute_force_one_step(cube, &mut vec![]);
    if solution != "" {
        return solution;
    }
    return "not solved".to_string();
}

fn solve_cube_brute_force_embedded_loops(cube: &mut RubiksCube) -> String {
    for rot1 in 0..12 {
        rotate_cube(cube, rot1);
        if cube.is_solved() {
            return moves_to_string(&vec![rot1]);
        }
        for rot2 in 0..12 {
            rotate_cube(cube, rot2);
            if cube.is_solved() {
                return moves_to_string(&vec![rot1, rot2]);
            }
            for rot3 in 0..12 {
                rotate_cube(cube, rot3);
                if cube.is_solved() {
                    return moves_to_string(&vec![rot1, rot2, rot3]);
                }
                for rot4 in 0..12 {
                    rotate_cube(cube, rot4);
                    if cube.is_solved() {
                        return moves_to_string(&vec![rot1, rot2, rot3, rot4]);
                    }
                    for rot5 in 0..12 {
                        rotate_cube(cube, rot5);
                        if cube.is_solved() {
                            return moves_to_string(&vec![rot1, rot2, rot3, rot4, rot5]);
                        }
                        for rot6 in 0..12 {
                            rotate_cube(cube, rot6);
                            if cube.is_solved() {
                                return moves_to_string(&vec![rot1, rot2, rot3, rot4, rot5, rot6]);
                            }
                            undo_move(cube, rot6);
                        }
                        undo_move(cube, rot5);
                    }
                    undo_move(cube, rot4);
                }
                undo_move(cube, rot3);
            }
            undo_move(cube, rot2);
        }
        undo_move(cube, rot1);
    }

    return "not solved".to_string();
}

fn solve_cube_brute_force(cube: &mut RubiksCube) -> String {
    if cube.is_solved() {
        return "".to_string();
    }

    //return solve_cube_brute_force_recursive(cube);
    return solve_cube_brute_force_embedded_loops(cube);
}

pub fn solve_cube(cube: &mut RubiksCube) -> String {
    return solve_cube_brute_force(cube);
}