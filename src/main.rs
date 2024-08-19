use std::time::{Duration, Instant};

mod rubikscube;
mod cubesolver;
mod common_functions;

fn test_solve_time(cube: &mut rubikscube::RubiksCube, scarmble_moves: i32) {
    let iters: u128 = 50;
    let mut sum_duration: u128 = 0;
    for _i in 0..iters {
        let scramble: String = cube.scramble(scarmble_moves);
        //println!("Scramble: {}", scramble);

        let now: Instant = Instant::now();
        cubesolver::solve_cube(cube);
        let end: Duration = now.elapsed();
        sum_duration += end.as_millis();
    }
    println!("avg solve time for {} moves is {} ms", scarmble_moves, sum_duration/iters);
}

fn main() {
    println!("Hello, world!");
    println!("This is a Rubik's cube solver");

    let mut cube: rubikscube::RubiksCube = rubikscube::RubiksCube::new();

    for scarmble_moves in 1..9 {
        //test_solve_time(&mut cube, scarmble_moves);
    }
    //test_solve_time(&mut cube, 10);

    
    let scramble: String = cube.scramble(6);
    println!("Scramble: {}", scramble);

    let now: Instant = Instant::now();
    let sol = cubesolver::solve_cube(&mut cube);
    println!("Solution: {}", sol);
    let end: Duration = now.elapsed();
    println!("Solve time: {} ms", end.as_millis());
}
