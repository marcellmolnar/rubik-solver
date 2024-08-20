use std::time::{Duration, Instant};
use rand::Rng;

mod rubikscube;
mod cubesolver;
mod common_functions;
mod cubesolver_genetic_alg;

fn test_solve_time(cube: &mut rubikscube::RubiksCube, max_scarmble_moves: i32) {
    fn test_solve_time_moves(cube: &mut rubikscube::RubiksCube, scarmble_moves: i32) {
        let iters: u128 = 50;
        let mut sum_duration: u128 = 0;
        for _i in 0..iters {
            let _scramble: String = cube.scramble(scarmble_moves);
            //println!("Scramble: {}", _scramble);

            let now: Instant = Instant::now();
            cubesolver::solve_cube(cube, cubesolver::SolveMode::BruteForceSpreadOut);
            let end: Duration = now.elapsed();
            sum_duration += end.as_millis();
        }
        println!("avg solve time for {} moves is {} ms", scarmble_moves, sum_duration/iters);
    }

    for scarmble_moves in 1..max_scarmble_moves {
        test_solve_time_moves(cube, scarmble_moves);
    }
}

fn brute_force_solve() {
    let mut cube: rubikscube::RubiksCube = rubikscube::RubiksCube::new();
    test_solve_time(&mut cube, 10);
    
    /*
    for _i in 0..100 {
        if _i % 10 == 0 {
            println!("Iteration: {}", _i);
        }
        cube.reset();
        let scramble: String = cube.scramble(15);

        let sol = cubesolver::solve_cube(&mut cube);
        if sol == "not solved" {
            println!("Scramble: {}", scramble);
        }
    } 
    */
}

fn genetic_alg_solve() {
    let mut cube: rubikscube::RubiksCube = rubikscube::RubiksCube::new();
    let _scramble: String = cube.scramble(15);
    cubesolver_genetic_alg::solve_cube(&mut cube);
}

fn main() {
    println!("Hello, world!");
    println!("This is a Rubik's cube solver");

    //brute_force_solve();
    genetic_alg_solve();    
}
