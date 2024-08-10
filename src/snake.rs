use crate::Position;

pub struct SnakeBodyPart {
    pub x: f32,
    pub y: f32,
}

impl SnakeBodyPart {
    pub fn new(x: f32, y: f32) -> SnakeBodyPart {
        SnakeBodyPart { x, y }
    }

    fn update_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}

pub struct Snake {
    pub head_x: f32,
    pub head_y: f32,
    pub body: Vec<SnakeBodyPart>,
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            head_x: 40.0,
            head_y: 0.0,
            body: vec![SnakeBodyPart::new(0.0, 0.0)],
        }
    }

    pub fn add_body_part(&mut self, body_part: SnakeBodyPart) {
        self.body.push(body_part);
    }

    pub fn slither(&mut self, direction: &str) {
        let mut previous_parts_position = Position {
            x: self.head_x.clone(),
            y: self.head_y.clone(),
        };

        let body_parts_iterator = self.body.iter_mut();
        for body_part in body_parts_iterator {
            let old_x = body_part.x.clone();
            let old_y = body_part.y.clone();
            body_part.update_position(previous_parts_position.x, previous_parts_position.y);
            previous_parts_position.x = old_x;
            previous_parts_position.y = old_y;
        }

        match direction {
            "right" => self.update_x_position(40.0),
            "left" => self.update_x_position(-40.0),
            "up" => self.update_y_position(-40.0),
            "down" => self.update_y_position(40.0),
            _ => panic!("error"),
        }
    }

    pub fn get_tail_position(&self) -> Position {
        let tail = self.body.last().unwrap();
        Position {
            x: tail.x.clone(),
            y: tail.y.clone(),
        }
    }

    pub fn check_body_collision(&self) -> bool {
        let mut collision = false;
        for body_part in &self.body {
            if self.head_x == body_part.x && self.head_y == body_part.y {
                collision = true;
            }
        }
        collision
    }

    pub fn check_collision_with_cordinates(&self, x: f32, y: f32) -> bool {
        if self.head_x == x && self.head_y == y {
            return true;
        }

        for body_part in &self.body {
            if x == body_part.x && y == body_part.y {
                return true;
            }
        }

        return false;
    }

    fn update_x_position(&mut self, x: f32) {
        self.head_x += x;
    }

    fn update_y_position(&mut self, y: f32) {
        self.head_y += y;
    }
}
