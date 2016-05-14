use sdl2::Sdl;
use sdl2::render::Renderer;

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
    pub fn run(self, _: Sdl, _: Renderer) {
        unimplemented!();
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

