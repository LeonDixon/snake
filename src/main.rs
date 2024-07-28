use macroquad::prelude::*;
use snake::{Snake, SnakeBodyPart};
use user_input::UserInput;

mod snake;
mod user_input;

fn window_conf() -> Conf {
    Conf {
        window_title: "Snake".to_owned(),
        window_height: 400,
        window_width: 400,
        ..Default::default()
    }
}

struct Food {
    pub x_position: f32,
    pub y_position: f32,
}

impl Food {
    fn new(x: f32, y: f32) -> Food {
        Food {
            x_position: x,
            y_position: y,
        }
    }
}

fn check_food_collision(snake: &Snake, food: &Food) -> bool {
    snake.head_x_position == food.x_position && snake.head_y_position == food.y_position
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut snake = Snake::new();

    let mut game_time = 0;
    // let mut food_on_screen = false;
    let mut user_input = UserInput::new();
    let mut food = Food::new(200.0, 200.0);

    loop {
        clear_background(WHITE);

        let new_time = (get_time() * 1.00).floor() as i32;
        user_input = user_input.get_direction();

        if new_time > game_time {
            let tail_position = snake.get_tail_position();
            snake.slither(&user_input.last_key_pressed);
            if check_food_collision(&snake, &food) {
                snake.add_body_part(SnakeBodyPart::new(tail_position.x, tail_position.y))
            }
            game_time = new_time;
        }

        draw_rectangle(
            snake.head_x_position,
            snake.head_y_position,
            40.0,
            40.0,
            GREEN,
        );

        for body_part in &snake.body {
            draw_rectangle(
                body_part.x_position,
                body_part.y_position,
                40.0,
                40.0,
                GREEN,
            );
        }

        draw_rectangle(food.x_position, food.y_position, 40.0, 40.0, RED);

        next_frame().await
    }
}
