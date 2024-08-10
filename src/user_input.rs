use std::collections::HashSet;

use macroquad::input::{get_keys_down, KeyCode};

pub struct UserInput {
    pub current_direction: String,
    queued_direction: Option<String>,
}

impl UserInput {
    pub fn new() -> UserInput {
        UserInput {
            current_direction: String::from("right"),
            queued_direction: None,
        }
    }

    fn check_direction_change_is_valid(&self) -> bool {
        if self.queued_direction.is_none() {
            return false;
        }

        match self.queued_direction.as_ref().unwrap().as_str() {
            "right" => return &self.current_direction != "left",
            "left" => return &self.current_direction != "right",
            "up" => return &self.current_direction != "down",
            "down" => return &self.current_direction != "up",
            _ => false,
        }
    }

    pub fn get_user_input(&mut self) {
        let keys_pressed: HashSet<KeyCode> = get_keys_down();
        let mut keys_iterator = keys_pressed.iter();
        let movement_key = keys_iterator.find(|key| {
            **key == KeyCode::Right
                || **key == KeyCode::D
                || **key == KeyCode::Left
                || **key == KeyCode::A
                || **key == KeyCode::Down
                || **key == KeyCode::S
                || **key == KeyCode::Up
                || **key == KeyCode::W
        });

        if movement_key.is_some() {
            let new_direction = self.transform_key_pressed_to_direction(movement_key.unwrap());
            self.queued_direction = Some(new_direction);
        }
    }

    pub fn get_direction(&mut self) -> &String {
        if self.check_direction_change_is_valid() {
            self.current_direction = self.queued_direction.as_deref().unwrap().to_string();
        }

        self.queued_direction = None;
        &self.current_direction
    }

    fn transform_key_pressed_to_direction(&self, key_pressed: &KeyCode) -> String {
        match key_pressed {
            KeyCode::Right | KeyCode::D => return String::from("right"),
            KeyCode::Left | KeyCode::A => return String::from("left"),
            KeyCode::Up | KeyCode::W => return String::from("up"),
            KeyCode::Down | KeyCode::S => return String::from("down"),
            _ => panic!("unsupported key"),
        };
    }
}
