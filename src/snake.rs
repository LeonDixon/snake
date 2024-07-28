use macroquad::input::KeyCode;

pub struct SnakeBodyPart {
    pub x_position: f32,
    pub y_position: f32,
}

impl SnakeBodyPart {
    pub fn new(x: f32, y: f32) -> SnakeBodyPart {
        SnakeBodyPart {
            x_position: x,
            y_position: y,
        }
    }

    fn update_position(&mut self, x: f32, y: f32) {
        self.x_position = x;
        self.y_position = y;
    }
}

pub struct Snake {
    pub head_x_position: f32,
    pub head_y_position: f32,
    pub body: Vec<SnakeBodyPart>,
}

pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            head_x_position: 40.0,
            head_y_position: 0.0,
            body: vec![SnakeBodyPart::new(0.0, 0.0)],
        }
    }

    pub fn add_body_part(&mut self, body_part: SnakeBodyPart) {
        self.body.push(body_part);
    }

    pub fn slither(&mut self, direction: &KeyCode) {
        let mut previous_parts_position = Position {
            x: self.head_x_position.clone(),
            y: self.head_y_position.clone(),
        };

        let body_parts_iterator = self.body.iter_mut();
        for body_part in body_parts_iterator {
            let old_x = body_part.x_position.clone();
            let old_y = body_part.y_position.clone();
            body_part.update_position(previous_parts_position.x, previous_parts_position.y);
            previous_parts_position.x = old_x;
            previous_parts_position.y = old_y;
        }

        match direction {
            KeyCode::Right => self.update_x_position(40.0),
            KeyCode::Left => self.update_x_position(-40.0),
            KeyCode::Up => self.update_y_position(-40.0),
            KeyCode::Down => self.update_y_position(40.0),
            _ => panic!("error"),
        }
    }

    pub fn get_tail_position(&self) -> Position {
        let tail = self.body.last().unwrap();
        Position {
            x: tail.x_position.clone(),
            y: tail.y_position.clone(),
        }
    }

    // pub fn check_collision(&self) -> bool {
    //     let mut collision = false;
    //     for body_part in &self.body {
    //         if self.head_x_position == body_part.x_position
    //             && self.head_y_position == body_part.y_position
    //         {
    //             collision = true;
    //         }
    //     }
    //     collision
    // }

    fn update_x_position(&mut self, x: f32) {
        self.head_x_position += x;
    }

    fn update_y_position(&mut self, y: f32) {
        self.head_y_position += y;
    }
}

// fn process_input(mut snake: Snake, last_key_pressed: KeyCode) {
//     match last_key_pressed {
//         miniquad::KeyCode::Right => snake.update_x_position(40.0),
//         miniquad::KeyCode::Left => snake.update_x_position(-40.0),
//         miniquad::KeyCode::Up => snake.update_y_position(-40.0),
//         miniquad::KeyCode::Down => snake.update_y_position(40.0),
//         _ => panic!("error"),
//     }
// }
