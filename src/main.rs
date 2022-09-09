extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, ButtonEvent, ButtonState, EventLoop, Key};

pub struct Game {
    gl: GlGraphics,
    snake: Snake,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs) {
        let green: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(arg.viewport(), |_context, gl| {
            graphics::clear(green, gl);
        });

        self.snake.render(&mut self.gl, arg);
    }

    fn update(&mut self) {
        self.snake.update();
    }

    fn pressed(&mut self, arg: &Button) {
        let last_direction = self.snake.direction.clone();

        self.snake.direction = match arg {
            &Button::Keyboard(Key::Up) if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down) if last_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left) if last_direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right) if last_direction != Direction::Left => Direction::Right,
            _ => last_direction,
        }
    }
}

#[derive(Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let blue: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let square =
            graphics::rectangle::square((self.x * 20) as f64, (self.y * 20) as f64, 20_f64);

        gl.draw(args.viewport(), |context, gl| {
            let transform = context.transform;

            graphics::rectangle(blue, square, transform, gl);
        });
    }

    fn update(&mut self) {
        match self.direction {
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
        }
    }
}

fn main() {
    let opengl = OpenGL::V4_5;

    let mut window: Window = WindowSettings::new("Snake", [200, 200])
        .resizable(false)
        .graphics_api(opengl)
        .exit_on_esc(false)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            x: 0,
            y: 0,
            direction: Direction::Right,
        },
    };

    let mut events = Events::new(EventSettings::new()).ups(7);

    while let Some(event) = events.next(&mut window) {
        if let Some(args) = event.render_args() {
            game.render(&args);
        }

        if let Some(_args) = event.update_args() {
            game.update();
        }

        if let Some(args) = event.button_args() {
            if args.state == ButtonState::Press {
                game.pressed(&args.button);
            }
        }
    }
}
