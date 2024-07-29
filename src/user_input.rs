use std::collections::HashSet;

use macroquad::input::{get_keys_pressed, KeyCode};

enum Direction {
    Right,
    Left,
    Up,
    Down,
}

// impl Direction {
//     fn keycode_to_direction()
// }

pub struct UserInput {
    current_direction: String,
}

impl UserInput {
    pub fn new() -> UserInput {
        UserInput {
            current_direction: String::from("right"),
        }
    }

    // pub fn check_direction_change_is_valid(current ) {

    // }

    pub fn get_direction(&mut self) -> &String {
        let keys_pressed: HashSet<KeyCode> = get_keys_pressed();
        let mut keys_iterator = keys_pressed.iter();
        let movement_key = keys_iterator.find(|key| {
            **key == KeyCode::Right
                || **key == KeyCode::Left
                || **key == KeyCode::Down
                || **key == KeyCode::Up
        });

        if movement_key.is_some() {
            self.current_direction = self.transform_key_pressed_to_direction(movement_key.unwrap());
        }

        return &self.current_direction;
    }

    fn transform_key_pressed_to_direction(&self, key_pressed: &KeyCode) -> String {
        match key_pressed {
            KeyCode::Right => return String::from("right"),
            KeyCode::Left => return String::from("left"),
            KeyCode::Up => return String::from("up"),
            KeyCode::Down => return String::from("down"),
            _ => panic!("unsupported key"),
        };
    }
}
