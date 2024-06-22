mod rubikscube;

fn main() {
    println!("Hello, world!");
    println!("This is a Rubik's cube solver");
    
    let mut cube = rubikscube::RubiksCube::new();
    cube.U();
    cube.repr();
}
