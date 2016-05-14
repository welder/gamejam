extern crate sdl2;

mod game;
use game::Game;

fn main() {
    let game         = Game::new();
    let mut context  = sdl2::init().unwrap();
    let mut renderer = context.video()
                              .unwrap()
                              .window(game.title(), game.width(), game.height())
                              .position_centered()
                              .resizable()
                              .build()
                              .unwrap()
                              .renderer()
                              .present_vsync()
                              .build()
                              .unwrap();

    game.run(&mut context, &mut renderer);
}
