use crate::graphics::Graphics;
use ash::version::DeviceV1_0;
use ash::vk;
use std::error::Error;
use std::sync::Arc;
use vk_mem as vma;
use vk_mem::ffi::vmaCreateImage;

pub struct Image {
    graphics: Arc<Graphics>,
    handle: vk::Image,
    allocation: vma::Allocation,
    allocation_info: vma::AllocationInfo,
    image_type: vk::ImageType,
    extent: vk::Extent3D,
    format: vk::Format,
    usage: vk::ImageUsageFlags,
    samples: vk::SampleCountFlags,
    tiling: vk::ImageTiling,
    subresource: vk::ImageSubresource,
    mip_levels: u32,
    layers: u32,
}

impl Image {
    pub fn new(
        graphics: Arc<Graphics>,
        extent: vk::Extent3D,
        format: vk::Format,
        usage: vk::ImageUsageFlags,
        memory_usage: vma::MemoryUsage,
        samples: vk::SampleCountFlags,
        tiling: vk::ImageTiling,
        mip_levels: u32,
        layers: u32,
        create_flags: vk::ImageCreateFlags,
    ) -> Result<Self, Box<dyn Error>> {
        assert!(mip_levels > 0, "Image should have at least one mip level.");
        assert!(layers > 0, "Image should have at least one layer.");

        let subresource = vk::ImageSubresource::builder()
            .mip_level(mip_levels)
            .array_layer(layers)
            .build();

        let image_create_info = vk::ImageCreateInfo::builder()
            .flags(create_flags)
            .image_type(Self::determine_image_type(extent)?)
            .format(format)
            .extent(extent)
            .mip_levels(mip_levels)
            .array_layers(layers)
            .samples(samples)
            .tiling(tiling)
            .usage(usage)
            .build();

        let mut allocation_create_info = vma::AllocationCreateInfo {
            usage: memory_usage,
            ..Default::default()
        };

        if usage.contains(vk::ImageUsageFlags::TRANSIENT_ATTACHMENT) {
            allocation_create_info.preferred_flags = vk::MemoryPropertyFlags::LAZILY_ALLOCATED;
        }

        let (handle, allocation, allocation_info) = graphics
            .allocator()
            .create_image(&image_create_info, &allocation_create_info)?;

        Ok(Self {
            graphics,
            handle,
            allocation,
            allocation_info,
            image_type: image_create_info.image_type,
            extent,
            format,
            usage,
            samples,
            tiling,
            subresource,
            mip_levels,
            layers,
        })
    }

    pub fn handle(&self) -> vk::Image {
        self.handle
    }

    pub fn format(&self) -> vk::Format {
        self.format
    }

    pub fn extent(&self) -> vk::Extent3D {
        self.extent
    }

    pub fn usage(&self) -> vk::ImageUsageFlags {
        self.usage
    }

    pub fn samples(&self) -> vk::SampleCountFlags {
        self.samples
    }

    pub fn tiling(&self) -> vk::ImageTiling {
        self.tiling
    }

    pub fn subresource(&self) -> vk::ImageSubresource {
        self.subresource
    }

    pub fn layers(&self) -> u32 {
        self.layers
    }

    fn determine_image_type(extent: vk::Extent3D) -> Result<vk::ImageType, Box<dyn Error>> {
        let mut dimensions = 0u32;

        if extent.width >= 1 {
            dimensions += 1
        }

        if extent.height >= 1 {
            dimensions += 1
        }

        if extent.depth > 1 {
            dimensions += 1
        }

        match dimensions {
            1 => Ok(vk::ImageType::TYPE_1D),
            2 => Ok(vk::ImageType::TYPE_2D),
            3 => Ok(vk::ImageType::TYPE_3D),
            _ => Err("Unsupported image type".into()),
        }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        self.graphics
            .allocator
            .destroy_image(self.handle, &self.allocation)
            .expect("Failed to deallocate vulkan image")
    }
}

pub struct ImageView {
    graphics: Arc<Graphics>,
    image: vk::Image,
    handle: vk::ImageView,
    range: vk::ImageSubresourceRange,
}

impl ImageView {
    pub fn new(
        graphics: Arc<Graphics>,
        image: &Image,
        format: vk::Format,
        view_type: vk::ImageViewType,
        aspect_mask: vk::ImageAspectFlags,
        components: vk::ComponentMapping,
        base_mip_level: u32,
        level_count: u32,
        base_array_layer: u32,
        layer_count: u32,
    ) -> Result<Self, Box<dyn Error>> {
        let subresource = vk::ImageSubresourceRange::builder()
            .aspect_mask(aspect_mask)
            .base_mip_level(base_mip_level)
            .level_count(level_count)
            .base_array_layer(base_array_layer)
            .layer_count(layer_count)
            .build();

        let create_info = vk::ImageViewCreateInfo::builder()
            .image(image.handle())
            .view_type(view_type)
            .format(format)
            .components(components)
            .subresource_range(subresource)
            .build();

        let handle = unsafe { graphics.device.create_image_view(&create_info, None)? };

        Ok(Self {
            graphics,
            image: image.handle(),
            handle,
            range: subresource,
        })
    }
}

impl Drop for ImageView {
    fn drop(&mut self) {
        unsafe { self.graphics.device.destroy_image_view(self.handle, None) }
    }
}
