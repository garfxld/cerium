use std::collections::HashMap;

use crate::{registry::dimension_type::DimensionType, world::chunk::Chunk};

pub struct World {
    dimension_type: DimensionType,
    chunks: HashMap<(i32, i32), Chunk>,
}

impl World {
    pub fn new(dimension_type: DimensionType) -> Self {
        Self {
            dimension_type,
            chunks: HashMap::new(),
        }
    }

    pub fn get_chunk(&mut self, chunk_x: i32, chunk_z: i32) -> Option<&mut Chunk> {
        self.chunks.get_mut(&(chunk_x, chunk_z))
    }

    pub fn load_chunk(&mut self, chunk_x: i32, chunk_z: i32) {
        self.chunks.insert(
            (chunk_x, chunk_z),
            Chunk::new(chunk_x, chunk_z, self.dimension_type.min_y),
        );
    }

}

#[cfg(test)]
mod tests {

    #[test]
    fn test_chunk_index() {}
}
