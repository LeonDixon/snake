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

    fn check_direction_change_is_valid(&self, direction: &String) -> bool {
        match direction.as_str() {
            "right" => return &self.current_direction != "left",
            "left" => return &self.current_direction != "right",
            "up" => return &self.current_direction != "down",
            "down" => return &self.current_direction != "up",
            _ => panic!("unsupported direction"),
        }
    }

    pub fn get_direction(&mut self) -> &String {
        let keys_pressed: HashSet<KeyCode> = get_keys_pressed();
        let mut keys_iterator = keys_pressed.iter();
        let movement_key = keys_iterator.find(|key| {
            **key == KeyCode::Right
                || **key == KeyCode::Left
                || **key == KeyCode::Down
                || **key == KeyCode::Up
        });

        //TODO need to add a pending direction to hold the non commited direction

        if movement_key.is_some() {
            let new_direction = self.transform_key_pressed_to_direction(movement_key.unwrap());
            if self.check_direction_change_is_valid(&new_direction) {
                self.current_direction = new_direction
            }
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
