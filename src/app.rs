//! Main entry point for the game. It manages the game loop.

use glium::Display;
use glium::glutin::{Event, VirtualKeyCode};
use std::time::{Duration, Instant};

use config::Config;
use graphics::Quad;

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
        use graphics::Quad;

        let mut commands: Vec<Command> = Vec::new();
        let mut events = self.display.poll_events();

        let mut quad: Quad = Quad::new(&self.display, (32, 32), (32, 32));

        GameLoop::new(self.config.frame_rate).run(|_| {
            process_events(&mut events, &mut commands);
            if !update_and_keep_running(&mut commands, &mut quad) { return false }
            render(&self.display, &quad);

            true
        });
    }
}

fn process_events<I: Iterator<Item = Event>>(events: &mut I, commands: &mut Vec<Command>) {
    use glium::glutin::ElementState;

    if let Some(event) = events.next() {
        match event {
            Event::KeyboardInput(ElementState::Released, _, Some(key)) => {
                if let Some(command) = get_keyboard_command(key) { commands.push(command) }
            },
            _ => { }
        }
    }
}

fn update_and_keep_running(commands: &mut Vec<Command>, quad: &mut Quad) -> bool {
    match commands.pop() {
        Some(Command::Quit) => return false,
        Some(Command::Move(direction)) => quad.translate(direction),
        _ => { }
    }

    true
}

fn render(window: &Display, quad: &Quad) {
    use glium::Surface;

    use graphics::Render;

    let mut target = window.draw();
    target.clear_color(0.1, 0.1, 0.1, 1.0);

    target.render(quad);

    target.finish().unwrap();
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

    pub fn run<F: FnMut(Duration) -> bool>(mut self, mut loop_operation: F) {
        loop {
            let current_instant = Instant::now();

            if let FrameThrottler::Run(duration) = self.throttle(current_instant) {
                if !loop_operation(duration) { break }

                self.previous_instant = current_instant;
                self.update_fps_display(current_instant);
            } else {
                continue
            }
        }
    }

    fn throttle(&self, current_instant: Instant) -> FrameThrottler {
        use std::thread;

        let delta = current_instant - self.previous_instant;

        if delta < self.frame_interval {
            thread::sleep(self.frame_interval - delta);
            return FrameThrottler::Skip;
        }

        FrameThrottler::Run(delta)
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

/// Represents the option of either skipping the frame on the current
/// iteration because it occurred too soon (relative to the target
/// frame rate) or running the frame and passing the elapsed time since
/// the last executed frame.
enum FrameThrottler {
    Skip,
    Run(Duration),
}
