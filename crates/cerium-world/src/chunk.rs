use cerium_util::Position;

use crate::chunk_section::ChunkSection;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub chunk_x: i32,
    pub chunk_z: i32,
    min_y: i32,
    pub sections: Vec<ChunkSection>,
}

impl Chunk {
    pub fn new(chunk_x: i32, chunk_z: i32, min_y: i32) -> Self {
        let mut sections = vec![];
        for _ in 0..24 {
            sections.push(ChunkSection::new());
        }

        Self {
            chunk_x,
            chunk_z,
            min_y,
            sections,
        }
    }

    #[inline]
    fn to_relative(value: i32) -> usize {
        (value & 0x0F) as usize
    }

    #[inline]
    fn section_at(&self, y: i32) -> Option<&ChunkSection> {
        self.sections.get(((y - self.min_y) / 16) as usize)
    }

    #[inline]
    fn section_at_mut(&mut self, y: i32) -> Option<&mut ChunkSection> {
        self.sections.get_mut(((y - self.min_y) / 16) as usize)
    }

    pub fn get_block(&self, x: i32, y: i32, z: i32) -> u16 {
        let section = self.section_at(y);

        if let Some(section) = section {
            section.get_block(
                Self::to_relative(x),
                Self::to_relative(y),
                Self::to_relative(z),
            )
        } else {
            panic!("Chunk section out of bounds for y: {}", y);
        }
    }

    pub fn set_block(&mut self, x: i32, y: i32, z: i32, block: i32) {
        let section = self.section_at_mut(y);

        if let Some(section) = section {
            section.set_block(
                Self::to_relative(x),
                Self::to_relative(y),
                Self::to_relative(z),
                block,
            );
        } else {
            panic!("Chunk section out of bounds for y: {}", y);
        }
    }

    pub fn get_biome(&self, x: i32, y: i32, z: i32) -> u16 {
        let section = self.section_at(y);

        if let Some(section) = section {
            section.get_biome(
                Self::to_relative(x) / 4,
                Self::to_relative(y) / 4,
                Self::to_relative(z) / 4,
            )
        } else {
            panic!("Chunk section out of bounds for y: {}", y);
        }
    }

    pub fn set_biome(&mut self, x: i32, y: i32, z: i32, biome: i32) {
        let section = self.section_at_mut(y);

        if let Some(section) = section {
            section.set_biome(
                Self::to_relative(x) / 4,
                Self::to_relative(y) / 4,
                Self::to_relative(z) / 4,
                biome,
            );
        } else {
            panic!("Chunk section out of bounds for y: {}", y);
        }
    }

    pub fn to_chunk_pos(position: Position) -> (i32, i32) {
        ((position.x() / 16.0) as i32, (position.z() / 16.0) as i32)
    }

    /// This implementation comes from [Minestom](https://github.com/Minestom/Minestom/blob/7620f3320988e766cb8e34dd640b5a23911fa7e8/src/main/java/net/minestom/server/coordinate/ChunkRange.java#L48),
    /// which comes from [Krypton](https://github.com/KryptonMC/Krypton/blob/a9eff5463328f34072cdaf37aae3e77b14fcac93/server/src/main/kotlin/org/kryptonmc/krypton/util/math/Maths.kt#L62),
    /// which comes from a kotlin port [Esophose](https://github.com/Esophose),
    /// which originally comes from a [StackOverflow answer](https://stackoverflow.com/questions/398299/looping-in-a-spiral).
    pub fn chunks_in_range(chunk: (i32, i32), range: i32) -> Vec<(i32, i32)> {
        let (cx, cz) = chunk;

        // Send in spiral around the center chunk
        // Note: its not really required to start at the center anymore since the chunk queue is sorted by distance,
        //       however we still should send a circle so this method is still fine, and good for any other case a
        //       spiral might be needed.
        let mut chunks = vec![(cx, cz)];

        for id in 1..(range * 2 + 1) * (range * 2 + 1) {
            let index = id - 1;
            // compute radius (inverse arithmetic sum of 8 + 16 + 24 + ...)
            let radius = ((((index + 1) as f64).sqrt() - 1.0) / 2.0).floor() as i32 + 1;
            // compute total point on radius -1 (arithmetic sum of 8 + 16 + 24 + ...)
            let p = 8 * radius * (radius - 1) / 2;
            // points by face
            let en = radius * 2;
            // compute de position and shift it so the first is (-r, -r) but (-r + 1, -r)
            // so the square can connect
            let a = (1 + index - p) % (radius * 8);

            match a / (radius * 2) {
                // find the face (0 = top, 1 = right, 2 = bottom, 3 = left)
                0 => chunks.push((a - radius + cx, -radius + cz)),
                1 => chunks.push((radius + cx, a % en - radius + cz)),
                2 => chunks.push((radius - a % en + cx, radius + cz)),
                3 => chunks.push((-radius + cx, radius - a % en + cz)),
                _ => {}
            }
        }

        chunks
    }

    /// Calulates difference between chunks
    pub async fn difference<F, Fut>(lhs: (i32, i32), rhs: (i32, i32), range: i32, callback: F)
    where
        F: Fn(i32, i32) -> Fut,
        Fut: Future<Output = ()>,
    {
        let start_x = lhs.0 - range;
        let end_x = lhs.0 + range;
        let start_z = lhs.1 - range;
        let end_z = lhs.1 + range;

        for x in start_x..=end_x {
            for z in start_z..=end_z {
                if (x - rhs.0).abs() > range || (z - rhs.1).abs() > range {
                    callback(x, z).await;
                }
            }
        }
    }
}
