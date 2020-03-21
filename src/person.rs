use ncollide2d::shape::Ball;
use nalgebra::{Vector2, Isometry2};
use js_sys::Math::random;
use js_sys::Date;

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
    sick_time: Option<f64>
}


impl Person {
    pub fn new(x: f32, y: f32) -> Self {
        let radius = 4.0;
        let mut velocity_x: f32 = (random() as f32 - 0.25) / (0.75 - 0.25);
        let mut velocity_y: f32 = 1.0 - velocity_x;
        velocity_x = (-random() as i8) as f32 + velocity_x;
        velocity_y = (-random() as i8) as f32 + velocity_y;
        Self {
            serializable_data: SerializablePerson::new(x, y),
            velocity: Vector2::new(velocity_x, velocity_y),
            ball: Ball::new(radius),
            sick_time: None
        }
    }

    pub fn get_position(&self) -> Isometry2<f32> {
        Isometry2::new(Vector2::new(self.get_x(), self.get_y()), na::zero())
    }

    pub fn get_next_position(&self) -> Isometry2<f32> {
        Isometry2::new(Vector2::new(self.get_x(), self.get_y()) + self.velocity, na::zero())
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
            self.sick_time = Some(Date::now());
        }
        self.serializable_data.status = status;
    }

    pub fn get_status(&self) -> Status {
        self.serializable_data.status
    }

    pub fn update(&mut self) {
        // Update status
        if self.sick_time.is_some() && (self.sick_time.unwrap() + 7000.0) < Date::now() {
            self.sick_time = None;
            self.serializable_data.status = Status::Recovered;
        }

        // Update position
        self.set_x(self.get_x() + self.velocity.x);
        self.set_y(self.get_y() + self.velocity.y);
    }
}