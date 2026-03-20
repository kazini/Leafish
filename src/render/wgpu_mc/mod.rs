/// wgpu-mc integration bridge for Leafish.
///
/// This module provides adapter types that translate between Leafish's data
/// model and the wgpu-mc API:
///
/// - [`LeafishResourceProvider`]   → implements `wgpu_mc::mc::resource::ResourceProvider`
/// - [`LeafishBlockStateProvider`] → implements `wgpu_mc::mc::chunk::BlockStateProvider`
/// - [`WgpuMcBridge`]              → owns the `WmRenderer` and drives frame rendering
///
/// # Display construction
///
/// `WgpuMcBridge::new` accepts a pre-constructed [`wgpu_mc::Display`].
/// Building the Display requires a wgpu `Surface`, which in turn needs a
/// raw window handle.  The caller is responsible for that step; a helper
/// [`create_display`] is provided for convenience.
///
/// Once Leafish upgrades to `winit 0.30` / `raw-window-handle 0.6` (matching
/// wgpu-mc's deps), `create_display` can be called directly with the window.

pub mod block_state_provider;
pub mod math;
pub mod resource_provider;

pub use block_state_provider::LeafishBlockStateProvider;
pub use resource_provider::LeafishResourceProvider;

use std::sync::Arc;

use parking_lot::RwLock;
use wgpu_mc::mc::resource::ResourceProvider;
use wgpu_mc::wgpu::PresentMode;
use wgpu_mc::{wgpu, Display, WmRenderer};

use crate::resources;

/// Bundles a `WmRenderer` together with ancillary state needed per frame.
pub struct WgpuMcBridge {
    pub renderer: WmRenderer,
}

impl WgpuMcBridge {
    /// Create the bridge from a fully initialised `Display`.
    ///
    /// Call [`create_display`] to build the `Display`, or construct it yourself
    /// when integrating into an existing wgpu setup.
    pub fn new(display: Display, resources: Arc<RwLock<resources::Manager>>) -> Self {
        let resource_provider: Arc<dyn ResourceProvider> =
            Arc::new(LeafishResourceProvider::new(resources));

        let renderer = WmRenderer::new(display, resource_provider);
        renderer.init();

        Self { renderer }
    }

    /// Resize the wgpu surface when the window is resized.
    pub fn resize(&self, width: u32, height: u32) {
        let mut config = self.renderer.gpu.config.write();
        config.width = width;
        config.height = height;
        self.renderer
            .gpu
            .surface
            .configure(&self.renderer.gpu.device, &config);
    }
}

/// Build a wgpu-mc [`Display`] — example skeleton, not yet wired to Leafish's window.
///
/// TODO: Call this once Leafish upgrades to `winit 0.30` + `raw-window-handle 0.6`.
/// Until then construct `Display` manually or provide a pre-built Surface.
pub fn display_setup_example(
    instance: wgpu::Instance,
    surface: wgpu::Surface<'static>,
    width: u32,
    height: u32,
    vsync: bool,
) -> Display {
    use futures::executor::block_on;
    use wgpu_mc::wgpu::{
        DeviceDescriptor, Features, Limits, MemoryHints, PowerPreference, RequestAdapterOptions,
        SurfaceConfiguration, TextureFormat, TextureUsages,
    };

    let adapter = block_on(instance.request_adapter(&RequestAdapterOptions {
        power_preference: PowerPreference::HighPerformance,
        force_fallback_adapter: false,
        compatible_surface: Some(&surface),
    }))
    .expect("no suitable wgpu adapter found");

    let (device, queue) = block_on(adapter.request_device(
        &DeviceDescriptor {
            label: None,
            required_features: Features::default()
                | Features::DEPTH_CLIP_CONTROL
                | Features::PUSH_CONSTANTS
                | Features::MULTI_DRAW_INDIRECT,
            required_limits: Limits {
                max_push_constant_size: 128,
                max_bind_groups: 8,
                max_storage_buffers_per_shader_stage: 10000,
                ..Default::default()
            },
            memory_hints: MemoryHints::Performance,
        },
        None,
    ))
    .expect("wgpu device creation failed");

    let caps = surface.get_capabilities(&adapter);
    let present_mode = if vsync {
        PresentMode::AutoVsync
    } else if caps.present_modes.contains(&PresentMode::Immediate) {
        PresentMode::Immediate
    } else {
        caps.present_modes[0]
    };
    let config = SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: TextureFormat::Bgra8Unorm,
        width,
        height,
        present_mode,
        desired_maximum_frame_latency: 2,
        alpha_mode: caps.alpha_modes[0],
        view_formats: vec![],
    };

    surface.configure(&device, &config);

    Display {
        instance,
        adapter,
        surface,
        device,
        queue,
        config: RwLock::new(config),
    }
}
