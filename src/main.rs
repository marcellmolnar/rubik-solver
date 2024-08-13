mod rubikscube;
mod cubesolver;

fn main() {
    println!("Hello, world!");
    println!("This is a Rubik's cube solver");
    
    let mut cube = rubikscube::RubiksCube::new();
    cube.L();

    let soluton = cubesolver::solve_cube(&mut cube);
    println!("Solution: {}", soluton);

    // scramble cube
    let moves = 20;
    let scramble = cube.scramble(moves);
    println!("Scramble: {}", scramble);
}
