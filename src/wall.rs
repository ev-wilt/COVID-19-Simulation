use ncollide2d::shape::Plane;
use nalgebra::{Vector2, Isometry2};

pub enum Direction {
    Top,
    Bottom,
    Left,
    Right
}

pub struct Wall {
    plane: Plane<f32>,
    position: Isometry2<f32>,
    direction: Direction
}

impl Wall {
    pub fn new(direction: Direction, width: f32, height: f32) -> Wall {
        let position;
        let normal;
        match &direction {
            Direction::Top => { 
                position = Isometry2::new(Vector2::new(0.0, 0.0), na::zero());
                normal = Vector2::y_axis();
            },
            Direction::Bottom => { 
                position = Isometry2::new(Vector2::new(0.0, height), na::zero());
                normal = -Vector2::y_axis();
            },
            Direction::Left => { 
                position = Isometry2::new(Vector2::new(0.0, 0.0), na::zero());
                normal = Vector2::x_axis();
            },
            Direction::Right => { 
                position = Isometry2::new(Vector2::new(width, 0.0), na::zero());
                normal = -Vector2::x_axis();
            }
        }
        Self {
            plane: Plane::new(normal),
            position: position,
            direction: direction
        }
    }

    pub fn get_position(&self) -> Isometry2<f32> {
        self.position
    }

    pub fn get_plane(&self) -> &Plane<f32> {
        &self.plane
    }
}