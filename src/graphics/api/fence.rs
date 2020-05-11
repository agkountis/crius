use crate::graphics::Graphics;
use ash::version::DeviceV1_0;
use ash::vk;
use std::error::Error;
use std::sync::Arc;

pub struct Fence {
    graphics: Arc<Graphics>,
    handle: vk::Fence,
}

impl Fence {
    pub fn new(graphics: Arc<Graphics>) -> Result<Self, Box<dyn Error>> {
        let create_info = vk::FenceCreateInfo::builder().build();

        let handle = unsafe { graphics.device.create_fence(&create_info, None)? };

        Ok(Self { graphics, handle })
    }
}

impl Drop for Fence {
    fn drop(&mut self) {
        unsafe { self.graphics.device.destroy_fence(self.handle, None) }
    }
}
