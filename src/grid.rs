use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use tiled::parse;
use tiled::Map;

pub fn open_tiles() -> Map {
    let path = Path::new("assets/tiles.tmx");
    let file = File::open(path).unwrap();
    let file = BufReader::new(file);
    parse(file).unwrap()
}

