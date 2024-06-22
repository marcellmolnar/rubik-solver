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

#[derive(Clone)]
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

    fn rotInLayer(layer: &mut Layer, indices: [usize; 4]) {
        let tmp = layer.colors[indices[0]].clone();
        layer.colors[indices[0]] = layer.colors[indices[1]];
        layer.colors[indices[1]] = layer.colors[indices[2]];
        layer.colors[indices[2]] = layer.colors[indices[3]];
        layer.colors[indices[3]] = tmp;
    }

    fn rotLayer(layer: &mut Layer, clockwise: bool = true)
    {
        Self::rotInLayer(layer, if clockwise [0,6,8,2] else [0,2,8,6]);
        Self::rotInLayer(layer, if clockwise [1,3,7,5] else [1,5,7,3]);
    }

    fn rotInterLayer(layers: [&mut Layer; 4], indices: [usize; 3]) {
        for i in indices {
            let tmp = layers[0].colors[i];
            layers[0].colors[i] = layers[1].colors[i];
            layers[1].colors[i] = layers[2].colors[i];
            layers[2].colors[i] = layers[3].colors[i];
            layers[3].colors[i] = tmp;
        }
    }

    pub fn U(&mut self) {
        Self::rotLayer(&mut self.top);
        Self::rotInterLayer([&mut self.front, &mut self.right, &mut self.back, &mut self.left], [0,1,2]);
    }
    
    pub fn Up(&mut self) {
        Self::rotLayer(&mut self.top, false);
        Self::rotInterLayer([&mut self.front, &mut self.left, &mut self.back, &mut self.right], [0,1,2]);
    }

    pub fn repr(&self) {
        println!("top: {:?}", self.top.colors);
        println!("bottom: {:?}", self.bottom.colors);
        println!("front: {:?}", self.front.colors);
        println!("left: {:?}", self.left.colors);
        println!("back: {:?}", self.back.colors);
        println!("right: {:?}", self.right.colors);
    }
}
