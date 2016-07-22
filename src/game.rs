extern crate sdl2_ttf;

use std::thread;
use std::time;
use std::path;
use sdl2::Sdl;
use sdl2::render::Renderer;
use sdl2::render::Texture;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);
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
    texture: Texture,
    score: u32,
}

pub fn update_score_texture(font: &sdl2_ttf::Font, score: u32, renderer: &Renderer) -> Texture {
    let surface = font.render(&(score.to_string()))
                      .blended(Color::RGBA(255, 0, 0, 255))
                      .unwrap();
    renderer.create_texture_from_surface(&surface).unwrap()
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

        let paddle_width = 16;
        let ball_diameter = 16;

        let mut ticks = 0;
        renderer.set_draw_color(Color::RGB(0, 0, 0));

        let mut left_paddle_texture = renderer.create_texture_streaming(PixelFormatEnum::RGB24,
                                                                        256,
                                                                        256)
                                              .unwrap();

        left_paddle_texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
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

        let mut right_paddle_texture = renderer.create_texture_streaming(PixelFormatEnum::RGB24,
                                                                         256,
                                                                         256)
                                               .unwrap();

        right_paddle_texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
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

        let mut ball_texture = renderer.create_texture_streaming(PixelFormatEnum::RGB24, 256, 256)
                                       .unwrap();

        ball_texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
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

        let move_delta = 32;

        let mut player_one = Actor {
            pos: Position { x: 50, y: 100 },
            vel: Velocity { x: 0, y: 0 },
            texture: left_paddle_texture,
            score: 0,
        };

        let mut player_two = Actor {
            pos: Position {
                x: 750 - paddle_width,
                y: 100,
            },
            vel: Velocity { x: 0, y: 0 },
            texture: right_paddle_texture,
            score: 0,
        };

        let mut ball = Actor {
            pos: Position { x: 300, y: 300 },
            vel: Velocity { x: 10, y: 2 },
            texture: ball_texture,
            score: 0,
        };

        let ttf_context = sdl2_ttf::init().unwrap();
        let path_to_font = path::Path::new("resources/LiberationSans-Regular.ttf");
        let font = ttf_context.load_font(&path_to_font, 32).unwrap();

        let mut score_1_texture = update_score_texture(&font, player_one.score, renderer);

        let mut score_2_texture = update_score_texture(&font, player_two.score, renderer);

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
                            Some(Keycode::Space) => {
                                if ball.vel.x == 0 && ball.vel.y == 0 {
                                    ball.vel = Velocity { x: 10, y: 2 }
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }

            ball.pos.x += ball.vel.x;
            ball.pos.y += ball.vel.y;

            if ball.pos.x < (player_one.pos.x + ball_diameter) {
                if (player_one.pos.y < ball.pos.y - ball_diameter) &&
                   (ball.pos.y < player_one.pos.y + 128) {
                    ball.vel.x *= -1;
                } else {
                    ball.vel = Velocity { x: 0, y: 0 };
                    ball.pos = Position { x: 300, y: 300 };
                    player_two.score += 1;
                    score_2_texture = update_score_texture(&font, player_two.score, renderer);
                }
            }

            if (player_two.pos.x) < ball.pos.x + 16 {
                if (player_two.pos.y < ball.pos.y - ball_diameter) &&
                   (ball.pos.y < player_two.pos.y + 128) {
                    ball.vel.x *= -1;
                } else {
                    ball.vel = Velocity { x: 0, y: 0 };
                    ball.pos = Position { x: 300, y: 300 };
                    player_one.score += 1;
                    score_1_texture = update_score_texture(&font, player_one.score, renderer);
                }
            }

            if (ball.pos.y < 0) || (ball.pos.y > 600 - ball_diameter) {
                ball.vel.y *= -1;
            }


            self.update_title(&mut ticks, renderer);
            renderer.clear();

            renderer.copy(&player_one.texture,
                          None,
                          Some(Rect::new(player_one.pos.x,
                                         player_one.pos.y,
                                         paddle_width as u32,
                                         128)));

            renderer.copy(&player_two.texture,
                          None,
                          Some(Rect::new(player_two.pos.x,
                                         player_two.pos.y,
                                         paddle_width as u32,
                                         128)));

            renderer.copy(&ball.texture,
                          None,
                          Some(Rect::new(ball.pos.x,
                                         ball.pos.y,
                                         ball_diameter as u32,
                                         ball_diameter as u32)));

            renderer.copy(&mut score_1_texture,
                          None,
                          Some(Rect::new(player_one.pos.x, 10, 32, 64)));

            renderer.copy(&mut score_2_texture,
                          None,
                          Some(Rect::new(player_two.pos.x, 10, 32, 64)));

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
    fn update_title(&self, ticks: &mut i32, renderer: &mut Renderer) {
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
