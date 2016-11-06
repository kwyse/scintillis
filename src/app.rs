//! Main entry point for the game. It manages the game loop.

use glium::Display;
use glium::glutin::VirtualKeyCode;
use std::time::{Duration, Instant};

use config::Config;

#[derive(Debug, Clone, Copy)]
enum Command {
    Quit,
    Move(Direction),
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct App {
    config: Config,
    display: Display,
}

impl App {
    pub fn from_config(config: Config) -> Self {
        use glium::DisplayBuild;
        use glium::glutin::WindowBuilder;

        let display = WindowBuilder::new()
            .with_dimensions(config.window_width, config.window_height)
            .with_title(env!("CARGO_PKG_NAME"))
            .build_glium()
            .expect("Attempting to build Glium window");

        App {
            config: config,
            display: display,
        }
    }

    pub fn run(self) {
        use glium::glutin::{ElementState, Event};
        use glium::Surface;

        use graphics::Render;
        use graphics::Quad;

        let mut commands: Vec<Command> = Vec::new();
        let mut events = self.display.poll_events();
        let mut running = true;

        let mut quad: Quad = Quad::new(&self.display, (32i32, 32i32), (32i32, 32i32));

        GameLoop::new(self.config.frame_rate).run(|| {
            if let Some(event) = events.next() {
                match event {
                    Event::KeyboardInput(ElementState::Released, _, Some(key)) => {
                        if let Some(command) = get_keyboard_command(key) { commands.push(command) }
                    },
                    _ => { }
                }
            }

            match commands.pop() {
                Some(Command::Quit) => running = false,
                Some(Command::Move(direction)) => quad.translate(direction),
                _ => { }
            }

            let mut target = self.display.draw();
            target.clear_color(0.1, 0.1, 0.1, 1.0);

            target.render(&quad);

            target.finish().unwrap();

            running
        });
    }
}

fn get_keyboard_command(key: VirtualKeyCode) -> Option<Command> {
    use glium::glutin::VirtualKeyCode::*;

    match key {
        Escape => Some(Command::Quit),
        Up => Some(Command::Move(Direction::Up)),
        Down => Some(Command::Move(Direction::Down)),
        Left => Some(Command::Move(Direction::Left)),
        Right => Some(Command::Move(Direction::Right)),
        _ => None
    }
}

struct GameLoop {
    frame_interval: Duration,
    frame_count: u8,
    previous_instant: Instant,
    previous_second: Instant,
}

impl GameLoop {
    pub fn new(target_fps: f32) -> Self {
        GameLoop {
            frame_interval: Duration::from_millis(1_000 / target_fps as u64),
            frame_count: 0,
            previous_instant: Instant::now(),
            previous_second: Instant::now(),
        }
    }

    pub fn run<F: FnMut() -> bool>(mut self, mut loop_operation: F) {
        loop {
            let current_instant = Instant::now();
            if self.frame_too_soon(current_instant) { continue }

            if !loop_operation() { break }

            self.previous_instant = current_instant;
            self.update_fps_display(current_instant);
        }
    }

    fn frame_too_soon(&self, current_instant: Instant) -> bool {
        use std::thread;

        let delta = current_instant - self.previous_instant;

        if delta < self.frame_interval {
            thread::sleep(self.frame_interval - delta);
            return true;
        }

        false
    }

    fn update_fps_display(&mut self, current_instant: Instant) {
        self.frame_count += 1;

        if current_instant - self.previous_second >= Duration::from_secs(1) {
            println!("FPS: {}", self.frame_count);
            self.previous_second = current_instant;
            self.frame_count = 0;
        }
    }
}
