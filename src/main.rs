mod rubikscube;

fn main() {
    println!("Hello, world!");
    println!("This is a Rubik's cube solver");
    
    let mut cube = rubikscube::RubiksCube::new();
    cube.U();
    cube.Up();
    cube.repr();

    // print if it is solved
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

    // print cube state score
    println!("Score: {}", cube.state_score());

    // scramble cube
    let moves = 20;
    let scramble = cube.scramble(moves);
    println!("Scramble: {}", scramble);
}
