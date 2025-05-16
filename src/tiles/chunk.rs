use bevy::{
    ecs::{
        entity::{EntityMapper, MapEntities},
        reflect::ReflectMapEntities,
    },
    prelude::*,
};

use crate::{err::TileError, map::TilemapSize};

use super::TilePos;

/// Marker relation between chunks and maps
pub struct Chunk;

/// Used to store tile entities for fast look up.
/// Tile entities are stored in a grid. The grid is always filled with None.
#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
#[reflect(where T: Reflect)]
pub struct ChunkStorage<T> {
    tiles: Vec<Option<T>>,
    pub size: TilemapSize,
}

impl<T> Default for ChunkStorage<T> {
    fn default() -> Self {
        ChunkStorage {
            tiles: vec![],
            size: TilemapSize { x: 0, y: 0 },
        }
    }
}

impl<T: MapEntities> MapEntities for ChunkStorage<T> {
    fn map_entities<E: EntityMapper>(&mut self, entity_mapper: &mut E) {
        for tile in self.tiles.iter_mut().flatten() {
            tile.map_entities(entity_mapper);
        }
    }
}

impl<T> ChunkStorage<T> {
    /// Creates a new tile storage that is empty.
    pub fn empty(size: TilemapSize) -> Self {
        let mut tiles = Vec::with_capacity(size.count());
        for _ in 0..size.count() {
            tiles.push(None);
        }
        Self { tiles, size }
    }

    /// Gets a tile for the given tile position, if an is associated with that tile position.
    ///
    /// Panics if the given `tile_pos` doesn't lie within the extents of the underlying tile map.
    pub fn get(&self, tile_pos: &TilePos) -> Option<&T> {
        self.tiles[tile_pos.to_index(&self.size)].as_ref()
    }

    /// Gets a tile entity for the given tile position, if:
    /// 1) the tile position lies within the underlying tile map's extents *and*
    /// 2) there is an entity associated with that tile position;
    ///
    /// otherwise it returns `None`.
    pub fn try_get(&self, tile_pos: &TilePos) -> Result<Option<&T>, TileError> {
        if tile_pos.within_map_bounds(&self.size) {
            Ok(self.tiles[tile_pos.to_index(&self.size)].as_ref())
        } else {
            Err(TileError::OutOfBounds {
                size: self.size,
                target: *tile_pos,
            })
        }
    }

    /// Gets a tile for the given tile position, if an is associated with that tile position.
    ///
    /// Panics if the given `tile_pos` doesn't lie within the extents of the underlying tile map.
    pub fn get_mut(&mut self, tile_pos: &TilePos) -> Option<&mut T> {
        self.tiles[tile_pos.to_index(&self.size)].as_mut()
    }

    /// Gets a tile entity for the given tile position, if:
    /// 1) the tile position lies within the underlying tile map's extents *and*
    /// 2) there is an entity associated with that tile position;
    ///
    /// otherwise it returns `None`.
    pub fn try_get_mut(&mut self, tile_pos: &TilePos) -> Result<Option<&mut T>, TileError> {
        if tile_pos.within_map_bounds(&self.size) {
            Ok(self.tiles[tile_pos.to_index(&self.size)].as_mut())
        } else {
            Err(TileError::OutOfBounds {
                size: self.size,
                target: *tile_pos,
            })
        }
    }

    /// Sets a tile entity for the given tile position.
    ///
    /// If there is an entity already at that position, the original will be returned.
    ///
    /// Panics if the given `tile_pos` doesn't lie within the extents of the underlying tile map.
    pub fn set(&mut self, tile_pos: &TilePos, tile: T) -> Option<T> {
        self.tiles[tile_pos.to_index(&self.size)].replace(tile)
    }

    /// Sets a tile entity for the given tile position, if the tile position lies within the
    /// underlying tile map's extents.
    ///
    /// If there is an entity already at that position, the original will be returned.
    pub fn try_set(&mut self, tile_pos: &TilePos, tile: T) -> Result<Option<T>, TileError> {
        if tile_pos.within_map_bounds(&self.size) {
            Ok(self.tiles[tile_pos.to_index(&self.size)].replace(tile))
        } else {
            Err(TileError::OutOfBounds {
                size: self.size,
                target: *tile_pos,
            })
        }
    }

    /// Returns an iterator with all of the positions in the grid.
    pub fn iter(&self) -> impl Iterator<Item = &Option<T>> {
        self.tiles.iter()
    }

    /// Returns mutable iterator with all of the positions in the grid.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Option<T>> {
        self.tiles.iter_mut()
    }

    /// Removes any stored `T` at the given tile position, leaving `None` in its place and
    /// returning the `T`.
    ///
    /// Panics if the given `tile_pos` doesn't lie within the extents of the underlying tile map.
    pub fn remove(&mut self, tile_pos: &TilePos) -> Option<T> {
        self.tiles[tile_pos.to_index(&self.size)].take()
    }

    /// Remove any stored `T` at the given tile position, leaving `None` in its place and
    /// returning the `T`.
    ///
    /// Checks that the given `tile_pos` lies within the extents of the underlying map.
    pub fn try_remove(&mut self, tile_pos: &TilePos) -> Option<T> {
        self.tiles.get_mut(tile_pos.to_index(&self.size))?.take()
    }

    /// Removes all stored `T`s, leaving `None` in their place and
    /// returning them in an iterator.
    ///
    /// Example:
    /// ```
    /// # use bevy::prelude::Commands;
    /// # use bevy_ecs_tilemap::prelude::{TilemapSize, TileStorage};
    /// # fn example(mut commands: Commands) {
    /// # let mut storage = TileStorage::empty(TilemapSize { x: 16, y: 16 });
    /// for entity in storage.drain() {
    ///   commands.entity(entity).despawn();
    /// }
    /// # }
    /// ```
    pub fn drain(&mut self) -> impl Iterator<Item = T> {
        self.tiles.iter_mut().filter_map(|opt| opt.take())
    }
}
