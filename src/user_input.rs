use std::collections::HashSet;

use macroquad::{
    input::{get_keys_pressed, KeyCode},
    miniquad,
};

pub struct UserInput {
    pub last_key_pressed: KeyCode,
}

impl UserInput {
    pub fn new() -> UserInput {
        UserInput {
            last_key_pressed: miniquad::KeyCode::Right,
        }
    }

    pub fn get_direction(mut self) -> UserInput {
        let keys_pressed: HashSet<KeyCode> = get_keys_pressed();
        let mut keys_iterator = keys_pressed.iter();
        let movement_key = keys_iterator.find(|key| {
            **key == miniquad::KeyCode::Right
                || **key == miniquad::KeyCode::Left
                || **key == miniquad::KeyCode::Down
                || **key == miniquad::KeyCode::Up
        });

        self.last_key_pressed = movement_key.unwrap_or(&self.last_key_pressed).clone();

        self
    }
}
