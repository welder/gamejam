use sdl2::render::Texture;


#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x: x, y: y }
    }
}

pub struct Actor {
    position: Position,
    texture: Texture,
}

impl Actor {
    pub fn new(position: Position, texture: Texture) -> Actor {
        Actor {
            position: position,
            texture: texture,
        }
    }
}


pub struct ActorBuilder {
    position: Option<Position>,
    texture: Option<Texture>,
}

impl ActorBuilder {
    pub fn new() -> ActorBuilder {
        ActorBuilder {
            position: None,
            texture: None,
        }
    }

    pub fn position(&mut self, position: Position) -> &mut Self {
        self.position = Some(position);
        self
    }

    pub fn texture(&mut self, texture: Texture) -> &mut Self {
        self.texture = Some(texture);
        self
    }

    pub fn build(&self) -> Actor {
        Actor {
            position: self.position.unwrap(),
            texture: self.texture.unwrap().clone(),
        }
    }
}
