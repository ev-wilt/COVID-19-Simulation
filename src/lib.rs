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
use nalgebra::{Vector2, Isometry2, Matrix};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Simulation {
    people: Vec<Person>,
    updated_people: Vec<Box<SerializablePerson>>,
    walls: [Wall; 4]
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(width: f64, height: f64) -> Self {

        // Generate people
        let mut people = Vec::new();
        for _ in 0..50 {
            let x = random() as f32 * (width as f32 - 10.0) + 5.0;
            let y = random() as f32 * (height as f32 - 10.0) + 5.0;
            people.push(Person::new(x, y));
        }
        people[0].set_status(Status::Sick);

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
            walls: planes
        }
    }

    pub fn get_updated_people(&self) -> JsValue {
        JsValue::from_serde(&self.updated_people).unwrap()
    }

    pub fn update(&mut self) -> Result<(), JsValue> {
        self.updated_people.clear();

        for i in 0..self.people.len() {
            let (people_l, people_r) = self.people.split_at_mut(i);
            let (person, people_r) = people_r.split_at_mut(1);
            let person = &mut person[0];
            let last_x = person.get_x();
            let last_y = person.get_y();
            let last_status = person.get_status();

            person.update();

            // Check wall collisions
            let person_position = person.get_position();
            for wall in self.walls.iter() {
                let contact = query::contact(&wall.get_position(), wall.get_plane(), &person_position, person.get_ball(), 1.0);
                if contact.is_some() {
                    let normal = contact.unwrap().normal;
                    person.set_velocity(person.get_velocity() - 2.0 * Matrix::dot(&person.get_velocity(), &normal) * *normal)
                }
            }
            
            // Check collisions with other people
            let other_people = [people_l, people_r].concat();
            for mut other_person in other_people {
                let contact = query::contact(&other_person.get_position(), other_person.get_ball(), &person_position, person.get_ball(), 1.0);
                if contact.is_some() {
                    if person.get_status() == Status::Sick && other_person.get_status() == Status::Healthy {
                        other_person.set_status(Status::Sick);
                    }
                    if person.get_status() == Status::Healthy && other_person.get_status() == Status::Sick {
                        person.set_status(Status::Sick);
                    }
                    let normal = contact.unwrap().normal;
                    person.set_velocity(person.get_velocity() - 2.0 * Matrix::dot(&person.get_velocity(), &normal) * *normal);
                    other_person.set_velocity(person.get_velocity() - 2.0 * Matrix::dot(&person.get_velocity(), &normal) * *normal);
                }
            }

            // Only update if the person has changed
            let has_changed = last_x != person.get_x() || last_y != person.get_y() || last_status != person.get_status();
            if has_changed {
                self.updated_people.push(Box::new(person.get_serializable_data()));
            }
        }

        Ok(())
    }

}