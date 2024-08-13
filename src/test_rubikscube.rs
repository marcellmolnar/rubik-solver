mod RubiksCube;

pub fn test_cube() {
    let mut cube = rubikscube::RubiksCube::new();

    cube.U();
    cube.Up();
    println!("Is solved: {}", cube.is_solved());

    cube.D();
    cube.Dp();
    println!("Is solved: {}", cube.is_solved());

    cube.R();
    cube.Rp();
    println!("Is solved: {}", cube.is_solved());

    cube.L();
    cube.Lp();
    println!("Is solved: {}", cube.is_solved());

    cube.F();
    cube.Fp();
    println!("Is solved: {}", cube.is_solved());

    cube.B();
    cube.Bp();
    println!("Is solved: {}", cube.is_solved());
}