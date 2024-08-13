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

pub fn solve_cube(cube: &mut RubiksCube) -> String {
    if cube.is_solved() {
        return "".to_string();
    }
    
    for rotation in 0..12 {
        rotate_cube(cube, rotation);
        if cube.is_solved() {
            return rotation_to_string(rotation);
        }

        // undo the move
        rotate_cube(cube, if rotation % 2 == 0 { rotation + 1 } else { rotation - 1 });
    }

    for step in 0..20 {
    }
    return "".to_string();
}