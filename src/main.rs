mod rubikscube;

fn main() {
    println!("Hello, world!");
    println!("This is a Rubik's cube solver");
    
    let cube = rubikscube::RubiksCube::new();
    cube.repr();
}
