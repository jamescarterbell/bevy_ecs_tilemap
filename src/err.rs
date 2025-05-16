use crate::{map::TilemapSize, tiles::TilePos};

pub enum TileError {
    OutOfBounds { size: TilemapSize, target: TilePos },
}
