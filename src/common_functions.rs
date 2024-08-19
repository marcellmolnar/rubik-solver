use crate::rubikscube::RubiksCube;

pub fn rotation_to_string(rotation: i32) -> String {
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
        12 => "-".to_string(),
        _ => panic!("Invalid move number"),
    }
}

pub fn rotate_cube(cube: &mut RubiksCube, rotation: i32) {
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
        12 => (),
        _ => panic!("Invalid move number"),
    }
}

pub fn undo_move(cube: &mut RubiksCube, rotation: i32) {
    if rotation == 12 {
        return;
    }
    rotate_cube(cube, if rotation % 2 == 0 { rotation + 1 } else { rotation - 1 });
}
