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
            width: 800,
            height: 600,
        }
    }

    /// Run the game
    pub fn run(self, context: &mut Sdl, renderer: &mut Renderer) {
        let mut ticks = 0;
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

            self.update_title(&mut ticks, renderer);
            renderer.set_draw_color(Color::RGB(0, 0, 0));
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

