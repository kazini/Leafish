use std::collections::HashMap;
use std::sync::Arc;

use glam::IVec3;
use parking_lot::RwLock;
use wgpu_mc::mc::block::{BlockstateKey, ChunkBlockState};
use wgpu_mc::mc::chunk::{BlockStateProvider, LightLevel};
use wgpu_mc::mc::BlockManager;

use crate::world::{block, World};
use leafish_shared::Position;

/// Bridges Leafish's `World` to wgpu-mc's `BlockStateProvider` trait.
///
/// On first call we lazily build a block name → wgpu-mc block-index table.
/// The augment (variant index) is always 0 for now; full variant mapping
/// can be added in a follow-up once the basic pipeline is working.
pub struct LeafishBlockStateProvider {
    world: Arc<World>,
    /// "minecraft:stone" → `BlockstateKey.block` (index in BlockManager)
    block_index_cache: RwLock<HashMap<String, u16>>,
    block_manager: Arc<RwLock<BlockManager>>,
}

impl LeafishBlockStateProvider {
    pub fn new(world: Arc<World>, block_manager: Arc<RwLock<BlockManager>>) -> Self {
        Self {
            world,
            block_index_cache: RwLock::new(HashMap::new()),
            block_manager,
        }
    }

    fn resolve_key(&self, block: block::Block) -> ChunkBlockState {
        if matches!(block, block::Block::Air {}) {
            return ChunkBlockState::Air;
        }
        let (namespace, name) = block.get_model();
        let resource_name = format!("{namespace}:{name}");

        // Fast path: cache hit
        if let Some(&idx) = self.block_index_cache.read().get(&resource_name) {
            return ChunkBlockState::State(BlockstateKey { block: idx, augment: 0 });
        }

        // Slow path: look up in BlockManager and populate cache
        let index = self
            .block_manager
            .read()
            .blocks
            .get_index_of(resource_name.as_str())
            .map(|i| i as u16);

        match index {
            Some(idx) => {
                self.block_index_cache.write().insert(resource_name, idx);
                ChunkBlockState::State(BlockstateKey { block: idx, augment: 0 })
            }
            None => ChunkBlockState::Air, // unmapped block — render as air for now
        }
    }
}

impl BlockStateProvider for LeafishBlockStateProvider {
    fn get_state(&self, pos: IVec3) -> ChunkBlockState {
        let leafish_pos = Position::new(pos.x, pos.y, pos.z);
        let block = self.world.get_block(leafish_pos);
        self.resolve_key(block)
    }

    fn get_light_level(&self, pos: IVec3) -> LightLevel {
        let leafish_pos = Position::new(pos.x, pos.y, pos.z);
        let sky = self.world.get_sky_light(leafish_pos);
        let block = self.world.get_block_light(leafish_pos);
        LightLevel::from_sky_and_block(sky, block)
    }

    fn is_section_empty(&self, rel_pos: IVec3) -> bool {
        // rel_pos is a chunk-section coordinate (chunk_x, section_y, chunk_z)
        self.world
            .capture_snapshot(rel_pos.x, rel_pos.y, rel_pos.z)
            .is_none()
    }

    fn get_block_color(&self, pos: IVec3, tint_index: i32) -> u32 {
        let leafish_pos = Position::new(pos.x, pos.y, pos.z);
        let block = self.world.get_block(leafish_pos);
        match block.get_tint() {
            _ if tint_index < 0 => 0xFFFFFF,
            // Grass/foliage/water tint — use placeholder until biome colors are wired up
            _ => 0x91BD59,
        }
    }
}
