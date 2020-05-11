use crate::graphics::Graphics;
use ash::version::DeviceV1_0;
use ash::vk;
use std::error::Error;
use std::sync::Arc;

pub struct PipelineCache {
    graphics: Arc<Graphics>,
    handle: vk::PipelineCache,
}

impl PipelineCache {
    pub fn new(graphics: Arc<Graphics>) -> Result<Self, Box<dyn Error>> {
        let create_info = vk::PipelineCacheCreateInfo::builder().build();

        let handle = unsafe { graphics.device.create_pipeline_cache(&create_info, None)? };

        Ok(Self { graphics, handle })
    }
}

impl Drop for PipelineCache {
    fn drop(&mut self) {
        unsafe {
            self.graphics
                .device
                .destroy_pipeline_cache(self.handle, None)
        }
    }
}
