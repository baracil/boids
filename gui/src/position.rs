use raylib::math::Vector2;
use crate::size::Size;

#[derive(Copy, Clone)]
pub struct Position {
    x:Coordinate,
    y:Coordinate,
}

#[derive(Copy, Clone)]
pub enum Coordinate {
    Absolute {
        value:f32
    },
    Relative {
        percent:f32
    }
}

impl Coordinate {
    pub fn compute_absolute(&self,available_space:f32) -> f32 {
        match self {
            Coordinate::Absolute { value } => *value,
            Coordinate::Relative { percent } => percent*available_space*0.01
        }
    }
}

impl Position {

    pub fn new(x:Coordinate,y:Coordinate) -> Self {
        Self{x,y}
    }

    pub fn compute_absolute(&self, available_space:&Size) -> Vector2 {
        let x= self.x.compute_absolute(available_space.width());
        let y= self.y.compute_absolute(available_space.height());
        Vector2{x,y}
    }

    pub fn with_x(&self, x:Coordinate) -> Self {
        Self{x,y:self.y}
    }

    pub fn with_y(&self, y:Coordinate) -> Self {
        Self{x:self.x,y}
    }

    pub fn set(&mut self, pos:&Position) -> &mut Position {
        self.x = pos.x;
        self.y = pos.y;
        self
    }

    pub fn set_ex(&mut self, x:&Coordinate, y:&Coordinate) -> &mut Position {
        self.x = x.clone();
        self.y = y.clone();
        self
    }

    pub fn set_x(&mut self, x:Coordinate) -> &mut Position {
        self.x = x;
        self
    }

    pub fn set_y(&mut self, y:Coordinate) -> &mut Position {
        self.y = y;
        self
    }
}