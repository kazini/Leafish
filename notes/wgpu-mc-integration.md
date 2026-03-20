# wgpu-mc Integration Notes

## Status
Phase 1 (adapters) complete and compiling.  Enable with `--features wgpu-mc`.

## What Was Built

### `src/render/wgpu_mc/resource_provider.rs`
`LeafishResourceProvider` implements `wgpu_mc::mc::resource::ResourceProvider`.
- Wraps `Arc<RwLock<resources::Manager>>`
- Splits `"namespace:path"` on `:` → calls `manager.open(namespace, path)`

### `src/render/wgpu_mc/block_state_provider.rs`
`LeafishBlockStateProvider` implements `wgpu_mc::mc::chunk::BlockStateProvider`.
- Uses `block.get_model()` to get `("minecraft", "stone")` → `"minecraft:stone"`
- Looks up the index in `BlockManager.blocks` (IndexMap) for `BlockstateKey.block`
- `augment` is always 0 for now (variant mapping is a TODO)
- Light levels come from `World::get_sky_light` / `get_block_light`
- Block colour returns a placeholder green; wire up biome colours later

### `src/render/wgpu_mc/math.rs`
Thin helpers to convert between cgmath (Leafish) and glam (wgpu-mc):
- `cgmath_vec3_to_glam`, `cgmath_point3_to_glam_vec3/ivec3`
- `leafish_pos_to_ivec3`, `ivec3_to_leafish_pos`
- `cgmath_mat4_to_glam`

### `src/render/wgpu_mc/mod.rs`
`WgpuMcBridge` owns a `WmRenderer`.
- `::new(display, resources)` → creates provider, calls `WmRenderer::new` + `init()`
- `::resize(w, h)` → reconfigures the wgpu surface
- `display_setup_example` shows how to build a `Display` from an existing `wgpu::Surface`
  (for when winit/rwh are upgraded)

## Blocking Issue: winit Version Mismatch

| | Leafish | wgpu-mc |
|--|--|--|
| winit | 0.29 | 0.30 |
| raw-window-handle | 0.5 | 0.6 |

These coexist in Cargo (resolver = 2, different semver) but Leafish cannot
pass its `winit::window::Window` to wgpu-mc's surface creation because the
trait impls changed.

**Fix**: upgrade Leafish winit 0.29 → 0.30 and update the event loop to use
the `ApplicationHandler` trait pattern (as in `wgpu-mc-demo`).

## Phase 2 Checklist

- [ ] Upgrade `winit` to 0.30 + `raw-window-handle` to 0.6 in Leafish
- [ ] Migrate event loop in `main.rs` to `ApplicationHandler`
- [ ] Call `display_setup_example` (or inline it) to build the `Display`
- [ ] Wire `BlockStateProvider::get_block_color` to real biome data
- [ ] Map block variant strings (from `get_model_variant()`) to augment indices
- [ ] Route chunk dirty-flag rebuilds to `wm.chunk_update_queue` sender
- [ ] Replace `render::Renderer` OpenGL draw calls with wgpu-mc `RenderGraph`
- [ ] Migrate entity models

## Dependency Notes
- wgpu-mc path dep: `/opt/3d/minecraft/wgpu-mc/rust/wgpu-mc`
- futures 0.3 added for `block_on` in display setup
- glam 0.29 added (matches wgpu-mc) for math helpers
