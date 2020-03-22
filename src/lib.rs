mod person;
mod wall;

#[macro_use]
extern crate serde_derive;
extern crate wasm_bindgen;
extern crate js_sys;
extern crate ncollide2d;
extern crate nalgebra as na;

use wasm_bindgen::prelude::*;
use person::*;
use wall::*;
use js_sys::Math::random;
use ncollide2d::query;
use nalgebra::Matrix;
use nalgebra::Vector2;

const TOTAL_PEOPLE: u16 = 50;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Simulation {
    people: Vec<Person>,
    updated_people: Vec<Box<SerializablePerson>>,
    walls: [Wall; 4],
    sick_total: u16,
    healthy_total: u16,
    recovered_total: u16,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(width: f64, height: f64, sim_type: &str, percentage: f64) -> Self {
        
        // Generate people
        let mut people = Vec::new();
        for i in 0..TOTAL_PEOPLE {
            let x = random() as f32 * (width as f32 - 10.0) + 5.0;
            let y = random() as f32 * (height as f32 - 10.0) + 5.0;
            let mut velocity_x: f32 = (random() as f32 - 0.25) / (0.75 - 0.25);
            let mut velocity_y: f32 = 1.0 - velocity_x;
            velocity_x = (-random() as i8) as f32 + velocity_x;
            velocity_y = (-random() as i8) as f32 + velocity_y;

            if sim_type == "freeForAll" {
                people.push(Person::new(x, y, false, Vector2::new(velocity_x, velocity_y)));
            }
            else if sim_type == "distancing" {
                let distancing_total = (TOTAL_PEOPLE as f64 * (percentage / 100.0)) as u16;
                if i < distancing_total {
                    people.push(Person::new(x, y, true, Vector2::new(0.0, 0.0)));
                }
                else {
                    people.push(Person::new(x, y, false, Vector2::new(velocity_x, velocity_y)));
                }
            }
            else {
                log("unsupported sim type");
            }
        }
        people[(TOTAL_PEOPLE - 1) as usize].set_status(Status::Sick);

        let mut updated_people: Vec<Box<SerializablePerson>> = Vec::new();
        people
            .clone()
            .into_iter()
            .for_each(|person| updated_people.push(Box::new(person.get_serializable_data())));


        let planes = [
            Wall::new(Direction::Top, width as f32, height as f32),
            Wall::new(Direction::Bottom, width as f32, height as f32),
            Wall::new(Direction::Left, width as f32, height as f32),
            Wall::new(Direction::Right, width as f32, height as f32)
        ];

        Self {
            people: people,
            updated_people: updated_people,
            walls: planes,
            sick_total: 1,
            healthy_total: TOTAL_PEOPLE - 1,
            recovered_total: 0
        }
    }

    pub fn get_updated_people(&self) -> JsValue {
        JsValue::from_serde(&self.updated_people).unwrap()
    }

    pub fn get_sick_total(&self) -> JsValue {
        JsValue::from_f64(self.sick_total as f64)
    }

    pub fn get_healthy_total(&self) -> JsValue {
        JsValue::from_f64(self.healthy_total as f64)
    }

    pub fn get_recovered_total(&self) -> JsValue {
        JsValue::from_f64(self.recovered_total as f64)
    }

    pub fn update(&mut self) -> Result<(), JsValue> {
        self.updated_people.clear();
        self.recovered_total = 0;
        self.healthy_total = 0;
        self.sick_total = 0;

        for i in 0..self.people.len() {
            let (people_l, people_r) = self.people.split_at_mut(i);
            let (person, people_r) = people_r.split_at_mut(1);
            let person = &mut person[0];
            person.update();

            // Update totals
            match person.get_status() {
                Status::Healthy => self.healthy_total += 1,
                Status::Sick => self.sick_total += 1,
                Status::Recovered => self.recovered_total += 1
            }

            // Check wall collisions
            let person_position = person.get_position();
            for wall in self.walls.iter() {
                let contact = query::contact(&wall.get_position(), wall.get_plane(), &person_position, person.get_ball(), 1.0);
                if contact.is_some() {
                    let normal = contact.unwrap().normal;
                    if Matrix::dot(&person.get_velocity(), &normal) < 0.0 {
                        person.set_velocity(person.get_velocity() - 2.0 * Matrix::dot(&person.get_velocity(), &normal) * *normal);
                    }
                }
            }
            
            // Check collisions with other people
            let other_people = [people_l, people_r].concat();
            for mut other_person in other_people {
                let contact = query::contact(&other_person.get_position(), other_person.get_ball(), &person_position, person.get_ball(), 0.0);
                if contact.is_some() {
                    if person.get_status() == Status::Sick && other_person.get_status() == Status::Healthy {
                        other_person.set_status(Status::Sick);
                    }
                    if person.get_status() == Status::Healthy && other_person.get_status() == Status::Sick {
                        person.set_status(Status::Sick);
                    }
                    let normal = contact.unwrap().normal;
                    if Matrix::dot(&person.get_velocity(), &normal) < 0.0 {
                        person.set_velocity(person.get_velocity() - 2.0 * Matrix::dot(&person.get_velocity(), &normal) * *normal);
                        other_person.set_velocity(person.get_velocity() - 2.0 * Matrix::dot(&person.get_velocity(), &-normal) * *-normal);    
                    }
                }
            }

            // Only update if the person has changed
            self.updated_people.push(Box::new(person.get_serializable_data()));
        }

        Ok(())
    }

}