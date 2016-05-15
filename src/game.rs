use std::thread;
use std::time;
use sdl2::Sdl;
use sdl2::render::Renderer;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

/// Struct for maintaining internal game state
pub struct Game {
    title: String,
    width: u32,
    height: u32,
}

pub struct Position {
    x: i32,
    y: i32,
}

pub struct Velocity {
    x: i32,
    y: i32,
}

pub struct Actor {
    pos: Position,
    vel: Velocity,
}

impl Game {
    /// Create a new game with the default parameters
    pub fn new() -> Game {
        Game {
            title: "Endless Tactics".to_owned(),
            width: 800,
            height: 600,
        }
    }

    /// Run the game
    pub fn run(self, context: &mut Sdl, renderer: &mut Renderer) {
        let paddle_width: i32 = 16;
        let ball_diameter: i32 = 16;

        let mut ticks = 0;
        renderer.set_draw_color(Color::RGB(0, 0, 0));

        let mut texture = renderer.create_texture_streaming(PixelFormatEnum::RGB24, 256, 256)
                                  .unwrap();

        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
                   for y in 0..256 {
                       for x in 0..256 {
                           let offset = y * pitch + x * 3;
                           buffer[offset + 0] = 255;
                           buffer[offset + 1] = 255;
                           buffer[offset + 2] = 255;
                       }
                   }
               })
               .unwrap();

        let move_delta: i32 = 32;

        let mut player_one = Actor {
            pos: Position { x: 50, y: 100 },
            vel: Velocity { x: 0, y: 0 },
        };

        let mut player_two = Actor {
            pos: Position { x: 750 - paddle_width, y: 100 },
            vel: Velocity { x: 0, y: 0 },
        };

        let mut ball = Actor {
            pos: Position { x: 300, y: 300 },
            vel: Velocity { x: 10, y: 2 },
        };

        let mut event_pump = context.event_pump().unwrap();
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    Event::KeyDown { keycode, .. } => {
                        match keycode {
                            Some(Keycode::Escape) => break 'running,
                            Some(Keycode::W) => player_one.pos.y -= move_delta,
                            Some(Keycode::S) => player_one.pos.y += move_delta,
                            Some(Keycode::Up) => player_two.pos.y -= move_delta,
                            Some(Keycode::Down) => player_two.pos.y += move_delta,
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }

            ball.pos.x += ball.vel.x;
            ball.pos.y += ball.vel.y;

            if ball.pos.x < (player_one.pos.x + ball_diameter) {
                if (player_one.pos.y < ball.pos.y - ball_diameter) && (ball.pos.y < player_one.pos.y + 128) {
                    ball.vel.x *= -1;
                } else {
                    ball.pos = Position { x: 300, y: 300 };
                }
            }

            if (player_two.pos.x) < ball.pos.x + 16 {
                if (player_two.pos.y < ball.pos.y - ball_diameter) && (ball.pos.y < player_two.pos.y + 128) {
                    ball.vel.x *= -1;
                } else {
                    ball.pos = Position { x: 300, y: 300 };
                }
            }

            if (ball.pos.y < 0) || (ball.pos.y > 600 - ball_diameter) {
                ball.vel.y *= -1;
            }


            self.update_title(&mut ticks, renderer);
            renderer.clear();

            renderer.copy(&texture,
                          None,
                          Some(Rect::new(player_one.pos.x, player_one.pos.y, paddle_width, 128)));

            renderer.copy(&texture,
                          None,
                          Some(Rect::new(player_two.pos.x, player_two.pos.y, paddle_width, 128)));

            renderer.copy(&texture,
                          None,
                          Some(Rect::new(ball.pos.x, ball.pos.y, ball_diameter, ball_diameter)));

            renderer.present();

            thread::sleep(time::Duration::from_millis(35));
        }
    }

    /// Get a reference to the game title
    pub fn title<'a>(&'a self) -> &'a str {
        &self.title
    }

    /// Get the width of the game window
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get the height of the game window
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Update the title with position and size information
    fn update_title(&self, ticks: &mut u32, renderer: &mut Renderer) {
        let mut window = renderer.window_mut().unwrap();
        let position = window.position();
        let size = window.size();
        let title = format!("{} - pos({}x{}) - size({}x{}) - ticks({})",
                            self.title(),
                            position.0,
                            position.1,
                            size.0,
                            size.1,
                            ticks);
        window.set_title(&title).unwrap();
        *ticks += 1
    }
}
