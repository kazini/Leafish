use std::io::Read;
use std::sync::Arc;

use parking_lot::RwLock;
use wgpu_mc::mc::resource::{ResourcePath, ResourceProvider};

use crate::resources;

/// Bridges Leafish's `resources::Manager` to wgpu-mc's `ResourceProvider` trait.
///
/// wgpu-mc uses `ResourcePath` of the form `"namespace:path/to/file"`.
/// Leafish's Manager expects `(plugin, name)` where plugin == namespace.
pub struct LeafishResourceProvider {
    pub resources: Arc<RwLock<resources::Manager>>,
}

impl LeafishResourceProvider {
    pub fn new(resources: Arc<RwLock<resources::Manager>>) -> Self {
        Self { resources }
    }

    /// Split `"namespace:path"` into `("namespace", "path")`.
    /// Falls back to `("minecraft", id)` when there is no colon.
    fn split_resource_path(id: &ResourcePath) -> (&str, &str) {
        match id.0.find(':') {
            Some(colon) => (&id.0[..colon], &id.0[colon + 1..]),
            None => ("minecraft", id.0.as_str()),
        }
    }
}

impl ResourceProvider for LeafishResourceProvider {
    fn get_bytes(&self, id: &ResourcePath) -> Option<Vec<u8>> {
        let (plugin, name) = Self::split_resource_path(id);
        let mut reader = self.resources.read().open(plugin, name)?;
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).ok()?;
        Some(buf)
    }
}
