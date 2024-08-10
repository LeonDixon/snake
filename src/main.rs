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

pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    fn new(x: f32, y: f32) -> Position {
        Position { x, y }
    }

    pub fn check_cordinates_are_equal(&self, comparison_cordinates: &Position) -> bool {
        return self.x == comparison_cordinates.x && self.y == comparison_cordinates.y;
    }
}

fn check_food_collision(snake: &Snake, food: &Position) -> bool {
    snake.head_x == food.x && snake.head_y == food.y
}

async fn run_snake_game() {
    let mut snake = Snake::new();

    let mut game_time = 0;
    let mut user_input = UserInput::new();
    let mut food = Position::new(200.0, 200.0);
    let mut score = 0;

    'game_loop: loop {
        clear_background(WHITE);
        let new_time = ((get_time() * 4.0) * 1.00).floor() as i32;

        user_input.get_user_input();

        if new_time > game_time {
            let tail_position = snake.get_tail_position();
            snake.slither(&user_input.get_direction());
            if snake.check_body_collision()
                || snake.head_x < 0.0
                || snake.head_y < 0.0
                || snake.head_x >= 400.0
                || snake.head_y >= 400.0
            {
                break 'game_loop;
            }
            if check_food_collision(&snake, &food) {
                snake.add_body_part(SnakeBodyPart::new(tail_position.x, tail_position.y));

                loop {
                    let random_x = ((rand::gen_range(0, 10)) * 40) as f32;
                    let random_y = ((rand::gen_range(0, 10)) * 40) as f32;
                    println!("{} {}", random_x, random_y);
                    if snake.check_collision_with_cordinates(random_x as f32, random_y as f32)
                        == false
                    {
                        food = Position::new(random_x, random_y);
                        score += 1;
                        break;
                    }
                }
            }
            game_time = new_time;
        }

        draw_rectangle(food.x, food.y, 40.0, 40.0, RED);

        draw_rectangle(snake.head_x, snake.head_y, 40.0, 40.0, GREEN);
        match user_input.current_direction.as_str() {
            "right" => {
                draw_rectangle(snake.head_x + 25.0, snake.head_y + 5.0, 10.0, 10.0, BLACK);
                draw_rectangle(snake.head_x + 30.0, snake.head_y + 25.0, 15.0, 5.0, RED);
            }
            _ => (),
        }

        for body_part in &snake.body {
            draw_rectangle(body_part.x, body_part.y, 40.0, 40.0, GREEN);
        }

        draw_text(
            format!("score: {score}").as_str(),
            20.0,
            20.0,
            20.0,
            DARKGRAY,
        );

        next_frame().await
    }
}

async fn main_menu() {
    loop {
        clear_background(WHITE);
        let play_button_x = ((window_conf().window_width / 2) - 50) as f32;
        let play_button_y = ((window_conf().window_height / 2) - 45) as f32;
        draw_rectangle(play_button_x, play_button_y, 100.0, 40.0, GOLD);
        let center = get_text_center("PLAY", Option::None, 40, 1.0, 0.0);
        let play_button_text_x = (window_conf().window_width / 2) as f32 - (center.x);
        let play_button_text_y = (window_conf().window_height / 2) as f32 + (center.y * 1.5);
        draw_text(
            "PLAY",
            play_button_text_x,
            play_button_text_y,
            40.0,
            DARKGRAY,
        );

        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            let x_diff = x - play_button_x;
            let y_diff = y - play_button_y;
            println!("{} {}", x_diff, y_diff);
            if x_diff > 0.0 && x_diff <= 100.0 && y_diff > 0.0 && y_diff <= 40.0 {
                break;
            }
        }
        next_frame().await
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state = "playings";
    loop {
        if game_state == "playing" {
            run_snake_game().await;
            game_state = "menu"
        } else {
            main_menu().await;
            game_state = "playing"
        }
    }
}
