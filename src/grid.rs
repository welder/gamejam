
use std::path::{PathBuf, Path};
use std::borrow::ToOwned;

#[derive(Copy, Clone)]
pub enum GridView {
    Orthogonal,
    Isometric,
}

pub struct Grid {
    values: Vec<Vec<u32>>,
    tileset: PathBuf,
    view: GridView,
    tile_width: u32,
    tile_height: u32,
    width: u32,
    height: u32,
}

pub struct GridBuilder {
    values: Option<Vec<Vec<u32>>>,
    tileset: PathBuf,
    view: Option<GridView>,
    tile_width: Option<u32>,
    tile_height: Option<u32>,
    width: Option<u32>,
    height: Option<u32>,
}

impl GridBuilder {
    pub fn with_tileset(tileset: &str) -> GridBuilder {
        GridBuilder {
            values: None,
            tileset: PathBuf::from(tileset),
            view: None,
            tile_width: None,
            tile_height: None,
            width: None,
            height: None,
        }
    }

    pub fn view(&mut self, view: GridView) -> &mut Self {
        self.view = Some(view);
        self
    }

    pub fn tile_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.tile_width = Some(width);
        self.tile_height = Some(height);
        self
    }

    pub fn grid_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn values(&mut self, values: Vec<Vec<u32>>) -> &mut Self {
        self.values = Some(values);
        self
    }

    pub fn build(&self) -> Grid {
        Grid {
            values: self.values.unwrap(),
            tileset: self.tileset,
            view: self.view.unwrap(),
            tile_width: self.tile_width.unwrap(),
            tile_height: self.tile_height.unwrap(),
            width: self.width.unwrap(),
            height: self.height.unwrap(),
        }
    }
}
