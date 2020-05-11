use crate::graphics::Graphics;
use ash::version::DeviceV1_0;
use ash::vk;
use std::error::Error;
use std::sync::Arc;

pub struct Semaphore {
    graphics: Arc<Graphics>,
    handle: vk::Semaphore,
}

impl Semaphore {
    pub fn new(graphics: Arc<Graphics>) -> Result<Self, Box<dyn Error>> {
        let create_info = vk::SemaphoreCreateInfo::builder().build();

        let handle = unsafe { graphics.device.create_semaphore(&create_info, None)? };

        Ok(Self { graphics, handle })
    }

    pub fn handle(&self) -> vk::Semaphore {
        self.handle
    }
}

impl Drop for Semaphore {
    fn drop(&mut self) {
        unsafe { self.graphics.device.destroy_semaphore(self.handle, None) }
    }
}

pub struct TimelineSemaphore(Semaphore);

impl TimelineSemaphore {
    pub fn new(graphics: Arc<Graphics>, initial_value: u64) -> Result<Self, Box<dyn Error>> {
        let mut semaphore_type_create_info = vk::SemaphoreTypeCreateInfo::builder()
            .semaphore_type(vk::SemaphoreType::TIMELINE)
            .initial_value(initial_value);

        let semaphore_create_info = vk::SemaphoreCreateInfo::builder()
            .push_next(&mut semaphore_type_create_info)
            .build();

        let handle = unsafe {
            graphics
                .device
                .create_semaphore(&semaphore_create_info, None)?
        };

        Ok(Self {
            0: Semaphore { graphics, handle },
        })
    }

    pub fn handle(&self) -> vk::Semaphore {
        self.0.handle()
    }
}
