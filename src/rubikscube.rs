use rand::Rng;

#[derive(Copy, Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
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
    colors: [Color; 9], // in scanline order
    mid_color: Color
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
        Self{top: Layer{colors:[Color::Yellow; 9], mid_color: Color::Yellow},
        bottom: Layer{colors:[Color::White; 9], mid_color: Color::White},
        left: Layer{colors:[Color::Blue; 9], mid_color: Color::Blue},
        right: Layer{colors:[Color::Green; 9], mid_color: Color::Green},
        front: Layer{colors:[Color::Red; 9], mid_color: Color::Red},
        back: Layer{colors:[Color::Orange; 9], mid_color: Color::Orange}}
    }

    fn rot_in_layer(layer: &mut Layer, indices: [usize; 4]) {
        let tmp = layer.colors[indices[0]].clone();
        layer.colors[indices[0]] = layer.colors[indices[1]];
        layer.colors[indices[1]] = layer.colors[indices[2]];
        layer.colors[indices[2]] = layer.colors[indices[3]];
        layer.colors[indices[3]] = tmp;
    }

    fn rot_layer(layer: &mut Layer, clockwise: bool)
    {
        Self::rot_in_layer(layer, if clockwise {[0,6,8,2]} else {[0,2,8,6]});
        Self::rot_in_layer(layer, if clockwise {[1,3,7,5]} else {[1,5,7,3]});
    }

    fn rot_inter_layer(layers: [&mut Layer; 4], indices: [usize; 3]) {
        for i in indices {
            let tmp = layers[0].colors[i];
            layers[0].colors[i] = layers[1].colors[i];
            layers[1].colors[i] = layers[2].colors[i];
            layers[2].colors[i] = layers[3].colors[i];
            layers[3].colors[i] = tmp;
        }
    }

    pub fn U(&mut self) {
        Self::rot_layer(&mut self.top, true);
        Self::rot_inter_layer([&mut self.front, &mut self.right, &mut self.back, &mut self.left], [0,1,2]);
    }
    
    pub fn Up(&mut self) {
        Self::rot_layer(&mut self.top, false);
        Self::rot_inter_layer([&mut self.front, &mut self.left, &mut self.back, &mut self.right], [0,1,2]);
    }

    pub fn D(&mut self) {
        Self::rot_layer(&mut self.bottom, true);
        Self::rot_inter_layer([&mut self.front, &mut self.left, &mut self.back, &mut self.right], [6,7,8]);
    }
    
    pub fn Dp(&mut self) {
        Self::rot_layer(&mut self.bottom, false);
        Self::rot_inter_layer([&mut self.front, &mut self.right, &mut self.back, &mut self.left], [6,7,8]);
    }

    pub fn R(&mut self) {
        Self::rot_layer(&mut self.right, true);
        Self::rot_inter_layer([&mut self.top, &mut self.front, &mut self.bottom, &mut self.back], [2,5,8]);
    }
    
    pub fn Rp(&mut self) {
        Self::rot_layer(&mut self.right, false);
        Self::rot_inter_layer([&mut self.top, &mut self.back, &mut self.bottom, &mut self.front], [2,5,8]);
    }

    pub fn L(&mut self) {
        Self::rot_layer(&mut self.left, true);
        Self::rot_inter_layer([&mut self.top, &mut self.back, &mut self.bottom, &mut self.front], [0,3,6]);
    }
    
    pub fn Lp(&mut self) {
        Self::rot_layer(&mut self.left, false);
        Self::rot_inter_layer([&mut self.top, &mut self.front, &mut self.bottom, &mut self.back], [0,3,6]);
    }

    // because of the representation of the cube, F and B are special cases
    pub fn F(&mut self) {
        Self::rot_layer(&mut self.front, true);
        let tmp: [Color; 3] = [self.top.colors[6], self.top.colors[7], self.top.colors[8]];
        self.top.colors[6] = self.left.colors[8];
        self.top.colors[7] = self.left.colors[5];
        self.top.colors[8] = self.left.colors[2];
        self.left.colors[8] = self.bottom.colors[2];
        self.left.colors[5] = self.bottom.colors[1];
        self.left.colors[2] = self.bottom.colors[0];
        self.bottom.colors[2] = self.right.colors[0];
        self.bottom.colors[1] = self.right.colors[3];
        self.bottom.colors[0] = self.right.colors[6];
        self.right.colors[0] = tmp[0];
        self.right.colors[3] = tmp[1];
        self.right.colors[6] = tmp[2];
    }

    pub fn Fp(&mut self) {
        Self::rot_layer(&mut self.front, false);
        let tmp: [Color; 3] = [self.top.colors[6], self.top.colors[7], self.top.colors[8]];
        self.top.colors[6] = self.right.colors[0];
        self.top.colors[7] = self.right.colors[3];
        self.top.colors[8] = self.right.colors[6];
        self.right.colors[0] = self.bottom.colors[2];
        self.right.colors[3] = self.bottom.colors[1];
        self.right.colors[6] = self.bottom.colors[0];
        self.bottom.colors[2] = self.left.colors[8];
        self.bottom.colors[1] = self.left.colors[5];
        self.bottom.colors[0] = self.left.colors[2];
        self.left.colors[8] = tmp[0];
        self.left.colors[5] = tmp[1];
        self.left.colors[2] = tmp[2];
    }

    pub fn B(&mut self) {
        Self::rot_layer(&mut self.back, true);
        let tmp: [Color; 3] = [self.top.colors[0], self.top.colors[1], self.top.colors[2]];
        self.top.colors[0] = self.right.colors[2];
        self.top.colors[1] = self.right.colors[5];
        self.top.colors[2] = self.right.colors[8];
        self.right.colors[2] = self.bottom.colors[8];
        self.right.colors[5] = self.bottom.colors[7];
        self.right.colors[8] = self.bottom.colors[6];
        self.bottom.colors[8] = self.left.colors[6];
        self.bottom.colors[7] = self.left.colors[3];
        self.bottom.colors[6] = self.left.colors[0];
        self.left.colors[6] = tmp[0];
        self.left.colors[3] = tmp[1];
        self.left.colors[0] = tmp[2];
    }

    pub fn Bp(&mut self) {
        Self::rot_layer(&mut self.back, false);
        let tmp: [Color; 3] = [self.top.colors[0], self.top.colors[1], self.top.colors[2]];
        self.top.colors[0] = self.left.colors[6];
        self.top.colors[1] = self.left.colors[3];
        self.top.colors[2] = self.left.colors[0];
        self.left.colors[6] = self.bottom.colors[8];
        self.left.colors[3] = self.bottom.colors[7];
        self.left.colors[0] = self.bottom.colors[6];
        self.bottom.colors[8] = self.right.colors[2];
        self.bottom.colors[7] = self.right.colors[5];
        self.bottom.colors[6] = self.right.colors[8];
        self.right.colors[2] = tmp[0];
        self.right.colors[5] = tmp[1];
        self.right.colors[8] = tmp[2];
    }

    pub fn state_score(&self) -> i32 {
        // 3 point for every correct color on each face
        // 1 point for every adjecent pair if they are not on the correct face

        let mut score : i32 = 0;
        for i in 0..9 {
            // skip the middle color
            if i == 4 {
                continue;
            }
            if self.top.colors[i] == Color::Yellow {
                score += 3;
            }
            if self.bottom.colors[i] == Color::White {
                score += 3;
            }
            if self.left.colors[i] == Color::Blue {
                score += 3;
            }
            if self.right.colors[i] == Color::Green {
                score += 3;
            }
            if self.front.colors[i] == Color::Red {
                score += 3;
            }
            if self.back.colors[i] == Color::Orange {
                score += 3;
            }
        }
        
        for [bottom_layer, top_layer] in [[&self.front, &self.top]]
        {
            if bottom_layer.colors[0] == bottom_layer.colors[1] && bottom_layer.colors[0] != bottom_layer.mid_color &&
            top_layer.colors[6] == top_layer.colors[7] && top_layer.colors[2] != top_layer.mid_color {
                score += 1;
            }
        }

        return score;
    }

    pub fn is_solved(&self) -> bool {
        for i in 0..9 {
            if self.top.colors[i] != Color::Yellow {
                return false;
            }
            if self.bottom.colors[i] != Color::White {
                return false;
            }
            if self.left.colors[i] != Color::Blue {
                return false;
            }
            if self.right.colors[i] != Color::Green {
                return false;
            }
            if self.front.colors[i] != Color::Red {
                return false;
            }
            if self.back.colors[i] != Color::Orange {
                return false;
            }
        }
        return true;
    }

    pub fn scramble(&mut self, moves: i32) -> String {
        let mut rng = rand::thread_rng();
        // string to strore the moves
        let mut moves_str = String::new();
        for _ in 0..moves {
            let r = rng.gen_range(0..12);
            match r {
                0 => {
                    self.U();
                    moves_str.push_str("U ");
                },
                1 => {
                    self.Up();
                    moves_str.push_str("U' ");
                },
                2 => {
                    self.D();
                    moves_str.push_str("D ");
                },
                3 => {
                    self.Dp();
                    moves_str.push_str("D' ");
                },
                4 => {
                    self.R();
                    moves_str.push_str("R ");
                },
                5 => {
                    self.Rp();
                    moves_str.push_str("R' ");
                },
                6 => {
                    self.L();
                    moves_str.push_str("L ");
                },
                7 => {
                    self.Lp();
                    moves_str.push_str("L' ");
                },
                8 => {
                    self.F();
                    moves_str.push_str("F ");
                },
                9 => {
                    self.Fp();
                    moves_str.push_str("F' ");
                },
                10 => {
                    self.B();
                    moves_str.push_str("B ");
                },
                11 => {
                    self.Bp();
                    moves_str.push_str("B' ");
                },
                _ => {}
            }
        }
        return moves_str;
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
