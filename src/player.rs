#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
}
pub struct Player {
    pub position: Position,
    pub delta: Position,
    pub velocity: Velocity,
    pub angle: f64,
}

impl Player {
    pub const SIZE: f64 = 25.0;

    pub fn new(position: [f64; 2], velocity: [f64; 2]) -> Self {
        Self {
            position: Position {
                x: position[0],
                y: position[1],
            },
            delta: Position {
                x: position[0],
                y: position[1],
            },
            velocity: Velocity {
                x: velocity[0],
                y: velocity[1],
            },
            angle: 0.0,
        }
    }
}
