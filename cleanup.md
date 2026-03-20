# wgpu-mc Integration Cleanup Plan

## Goal
Replace Leafish's OpenGL/glow renderer with wgpu-mc for modern GPU rendering.
The Leafish source already has a TODO noting this: *"renderer should be replaced with wgpu-mc soon-ish"*.

## Key Mismatches to Resolve

### winit version
- Leafish: `0.29` with `ApplicationHandler` style (old `run()` API)
- wgpu-mc: `0.30` with `ApplicationHandler` trait
- **Action**: Update Leafish winit to 0.30, adapt event loop accordingly.

### Math library
- Leafish: `cgmath 0.17` (Point3, Matrix4, Vector3)
- wgpu-mc: `glam 0.29` (Vec3, Mat4, IVec3)
- **Action**: Keep both; add thin conversion helpers in bridge.

### Block state IDs
- Leafish: Minecraft protocol vanilla numeric IDs (usize)
- wgpu-mc: `BlockstateKey { block: u16, augment: u16 }` indexed into BlockManager
- **Action**: Build a runtime mapping table during resource loading.

### Resource paths
- Leafish: `manager.open(plugin, name)` e.g. `("minecraft", "textures/block/stone.png")`
- wgpu-mc: `ResourcePath("minecraft:textures/block/stone.png")`
- **Action**: Split `namespace:path` on `:` to get (plugin, name).

## Files to Create
- `src/render/wgpu_mc/mod.rs`        — bridge entry point and Display setup
- `src/render/wgpu_mc/resource_provider.rs` — `ResourceProvider` impl
- `src/render/wgpu_mc/block_state_provider.rs` — `BlockStateProvider` impl
- `src/render/wgpu_mc/math.rs`       — cgmath ↔ glam conversions

## Files to Modify
- `Cargo.toml` — add `wgpu-mc` path dep (feature-gated), update winit to 0.30
- `src/render/mod.rs` — add `pub mod wgpu_mc;`

## Not Yet (Phase 2)
- Swap out OpenGL render calls for wgpu-mc draw calls
- Migrate texture atlas to wgpu-mc Atlas
- Entity model bridge
- Full winit 0.30 event loop migration
