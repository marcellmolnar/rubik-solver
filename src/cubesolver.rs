use rand::Rng;
use crate::rubikscube::RubiksCube;

fn rotation_to_string(rotation: i32) -> String {
    match rotation {
        0 => "U".to_string(),
        1 => "U'".to_string(),
        2 => "D".to_string(),
        3 => "D'".to_string(),
        4 => "R".to_string(),
        5 => "R'".to_string(),
        6 => "L".to_string(),
        7 => "L'".to_string(),
        8 => "F".to_string(),
        9 => "F'".to_string(),
        10 => "B".to_string(),
        11 => "B'".to_string(),
        _ => panic!("Invalid move number"),
    }
}

fn rotate_cube(cube: &mut RubiksCube, rotation: i32) {
    match rotation {
        0 => cube.U(),
        1 => cube.Up(),
        2 => cube.D(),
        3 => cube.Dp(),
        4 => cube.R(),
        5 => cube.Rp(),
        6 => cube.L(),
        7 => cube.Lp(),
        8 => cube.F(),
        9 => cube.Fp(),
        10 => cube.B(),
        11 => cube.Bp(),
        _ => panic!("Invalid move number"),
    }
}

fn undo_move(cube: &mut RubiksCube, rotation: i32) {
    rotate_cube(cube, if rotation % 2 == 0 { rotation + 1 } else { rotation - 1 });
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

fn solve_cube_brute_force(cube: &mut RubiksCube) -> String {
    if cube.is_solved() {
        return "".to_string();
    }
    
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

pub fn solve_cube(cube: &mut RubiksCube) -> String {
    return solve_cube_brute_force(cube);
}