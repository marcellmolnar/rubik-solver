#[derive(Copy, Clone)]
#[derive(Debug)]
enum Color {
    White,
    Yellow,
    Red,
    Orange,
    Blue,
    Green,
}

struct Layer {
    colors: [Color; 9]
}

pub struct RubiksCube {
    top: Layer,
    bottom: Layer,
    left: Layer,
    right: Layer,
    front: Layer,
    back: Layer
}

impl RubiksCube {
    pub fn new() -> Self {
        Self{top: Layer{colors:[Color::Yellow; 9]},
        bottom: Layer{colors:[Color::White; 9]},
        left: Layer{colors:[Color::Blue; 9]},
        right: Layer{colors:[Color::Green; 9]},
        front: Layer{colors:[Color::Red; 9]},
        back: Layer{colors:[Color::Orange; 9]}}
    }

    pub fn repr(&self) {
        println!("{:?}", self.top.colors);
        println!("{:?}", self.bottom.colors);
        println!("{:?}", self.right.colors);
        println!("{:?}", self.left.colors);
        println!("{:?}", self.front.colors);
        println!("{:?}", self.back.colors);
    }
}
