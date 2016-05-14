use std::collections::HashSet;
use sdl2::Sdl;
use sdl2::render::Renderer;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use actor::*;

/// Struct for maintaining internal game state
pub struct Game {
    title: String,
    width: u32,
    height: u32,
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
        // Make the renderer default to black.
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        let mut ticks = 0;

        // Initialize variables.
        let mut ticks = 0;

        // Create the texture for the player.
        let mut format = PixelFormatEnum::RGB24;
        let mut texture = renderer.create_texture_streaming(format, 256, 256)
                                  .unwrap();
        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
                   for y in 0..256 {
                       for x in 0..256 {
                           let offset = y * pitch + x * 3;
                           buffer[offset + 0] = x as u8;
                           buffer[offset + 1] = y as u8;
                           buffer[offset + 2] = 0;
                       }
                   }
               })
               .unwrap();

        let actor = ActorBuilder::new()
                        .position(Position::new(100, 100))
                        .texture(texture)
                        .build();

        let mut event_pump = context.event_pump().unwrap();
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    Event::KeyDown { keycode: Some(keycode), .. } => {
                        match keycode {
                            Keycode::Escape => break 'running,
                            // Keycode::Up => player.pos.y -= move_delta,
                            // Keycode::Down => player.pos.y += move_delta,
                            // Keycode::Left => player.pos.x -= move_delta,
                            // Keycode::Right => player.pos.x += move_delta,
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }

            self.update_title(&mut ticks, renderer);
            renderer.clear();
            renderer.present();
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
