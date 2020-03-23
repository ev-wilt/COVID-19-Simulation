use ncollide2d::shape::Ball;
use nalgebra::{Vector2, Isometry2};

const SICKNESS_DURATION: f64 = 450.0;

#[derive(Serialize)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Copy)]
pub enum Status {
    Healthy,
    Sick,
    Recovered
}

#[derive(Serialize)]
#[derive(Clone)]
pub struct SerializablePerson {
    x: f32,
    y: f32,
    status: Status
}

impl SerializablePerson {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x: x,
            y: y,
            status: Status::Healthy,
        }
    }
}

#[derive(Clone)]
pub struct Person {
    serializable_data: SerializablePerson,
    velocity: Vector2<f32>,
    ball: Ball<f32>,
    sick_time: Option<f64>,
    is_distancing: bool
}


impl Person {
    pub fn new(x: f32, y: f32, is_distancing: bool, velocity: Vector2<f32>) -> Self {
        let radius = 3.75;
        Self {
            serializable_data: SerializablePerson::new(x, y),
            velocity: velocity,
            ball: Ball::new(radius),
            sick_time: None,
            is_distancing: is_distancing
        }
    }

    pub fn get_position(&self) -> Isometry2<f32> {
        Isometry2::new(Vector2::new(self.get_x(), self.get_y()), na::zero())
    }

    pub fn get_serializable_data(&self) -> SerializablePerson {
        self.serializable_data.clone()
    }

    pub fn get_x(&self) -> f32 {
        self.serializable_data.x
    }

    pub fn set_x(&mut self, x: f32) {
        self.serializable_data.x = x;
    }

    pub fn get_y(&self) -> f32 {
        self.serializable_data.y
    } 

    pub fn set_y(&mut self, y: f32) {
        self.serializable_data.y = y;
    }

    pub fn get_ball(&self) -> &Ball<f32> {
        &self.ball
    }

    pub fn get_velocity(&self) -> Vector2<f32> {
        self.velocity
    }

    pub fn set_velocity(&mut self, velocity: Vector2<f32>) {
        self.velocity = velocity;
    }

    pub fn set_status(&mut self, status: Status) {
        if status == Status::Sick {
            self.sick_time = Some(0.0);
        }
        self.serializable_data.status = status;
    }

    pub fn get_status(&self) -> Status {
        self.serializable_data.status
    }

    pub fn update(&mut self, delta: f64) {
        // Update status
        if self.sick_time.is_some() {
            self.sick_time = Some(self.sick_time.unwrap() + delta);
            if self.sick_time.unwrap() > SICKNESS_DURATION {
                self.sick_time = None;
                self.serializable_data.status = Status::Recovered;
            }
        }

        // Update position
        self.set_x(self.get_x() + self.velocity.x * delta as f32);
        self.set_y(self.get_y() + self.velocity.y * delta as f32);

    }
}