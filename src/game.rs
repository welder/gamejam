use sdl2::Sdl;
use sdl2::render::Renderer;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

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
            width: 640,
            height: 480,
        }
    }

    /// Run the game
    pub fn run(self, context: &mut Sdl, renderer: &mut Renderer) {
        renderer.set_draw_color(Color::RGB(255, 0, 0));
        renderer.clear();
        renderer.present();

        let mut event_pump = context.event_pump().unwrap();

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..}
                    | Event::KeyDown { keycode: Some(Keycode::Escape), ..} =>
                    {
                        break 'running
                    },
                    _ => {}
                }
            }
            // The rest of the game loop goes here...
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
}

