use sdl2::render::Texture;


#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position {
            x: x,
            y: y
        }
    }
}

pub struct Actor {
    position: Position,
    texture: Texture,
    move_delta: i32,
}

impl Actor {
    pub fn at_position(position: Position) -> Actor {
        Actor {
            position: position
        }
    }

    pub fn render(&self, renderer: &mut Renderer) {
        renderer.copy(self.texture,
                      None,
                      Some(Rect::new(self.position.x, self.position.y, 128, 128)));
    }
}


        let mut texture = renderer.create_texture_streaming(PixelFormatEnum::RGB24, 256, 256)
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
