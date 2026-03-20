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
/// `WgpuMcBridge::new` accepts an `Arc<winit::window::Window>` and calls
/// `create_display` internally to build the wgpu `Display`.

pub mod block_state_provider;
pub mod math;
pub mod resource_provider;

pub use block_state_provider::LeafishBlockStateProvider;
pub use resource_provider::LeafishResourceProvider;

use std::sync::Arc;

use parking_lot::RwLock;
use wgpu_mc::mc::resource::ResourceProvider;
use wgpu_mc::wgpu::PresentMode;
use wgpu_mc::{Display, WmRenderer};

use crate::resources;

/// Bundles a `WmRenderer` together with ancillary state needed per frame.
pub struct WgpuMcBridge {
    pub renderer: WmRenderer,
}

impl WgpuMcBridge {
    /// Create the bridge from a winit window. Calls [`create_display`] internally.
    pub fn new(
        window: Arc<winit::window::Window>,
        vsync: bool,
        resources: Arc<RwLock<resources::Manager>>,
    ) -> Self {
        let display = create_display(window, vsync);
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

/// Build a wgpu-mc [`Display`] from a winit window.
pub fn create_display(window: Arc<winit::window::Window>, vsync: bool) -> Display {
    use futures::executor::block_on;
    use wgpu_mc::wgpu::{
        Backends, DeviceDescriptor, Features, Instance, InstanceDescriptor, Limits, MemoryHints,
        PowerPreference, RequestAdapterOptions, SurfaceConfiguration, TextureFormat, TextureUsages,
    };

    let instance = Instance::new(InstanceDescriptor {
        backends: Backends::PRIMARY,
        ..Default::default()
    });

    let surface = instance
        .create_surface(window.clone())
        .expect("wgpu surface");

    let adapter = block_on(instance.request_adapter(&RequestAdapterOptions {
        power_preference: PowerPreference::HighPerformance,
        force_fallback_adapter: false,
        compatible_surface: Some(&surface),
    }))
    .expect("no suitable wgpu adapter");

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
    .expect("wgpu device");

    let caps = surface.get_capabilities(&adapter);
    let present_mode = if vsync {
        PresentMode::AutoVsync
    } else if caps.present_modes.contains(&PresentMode::Immediate) {
        PresentMode::Immediate
    } else {
        caps.present_modes[0]
    };
    let size = window.inner_size();
    let config = SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: TextureFormat::Bgra8Unorm,
        width: size.width,
        height: size.height,
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
