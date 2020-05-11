pub mod api;

use crate::application::settings::Settings;
use crate::graphics::api::semaphore::TimelineSemaphore;
use crate::graphics::api::surface::Surface;
use ash::extensions::ext::DebugReport;
use ash::extensions::khr;
use ash::extensions::khr::{Swapchain, XlibSurface};
use ash::version::{DeviceV1_2, InstanceV1_0};
use ash::{version::EntryV1_0, vk, Device, Entry, Instance};
use std::borrow::Borrow;
use std::error::Error;
use std::ffi::{CStr, CString};
use vk_mem as vma;
use winit::window::Window;

const ENGINE_VERSION_MAJOR: &'static str = env!("CARGO_PKG_VERSION_MAJOR");
const ENGINE_VERSION_MINOR: &'static str = env!("CARGO_PKG_VERSION_MINOR");
const ENGINE_VERSION_PATCH: &'static str = env!("CARGO_PKG_VERSION_PATCH");

pub struct Graphics {
    instance: Instance,
    device: Device,
    entry: Entry,
    allocator: vma::Allocator,
}

impl Graphics {
    pub(crate) fn new(settings: &Settings, window: &Window) -> Result<Self, Box<dyn Error>> {
        let entry = Entry::new()?;

        let instance = Self::create_instance(&entry, settings, None)?;

        let physical_devices =
            unsafe { instance.enumerate_physical_devices() }.expect("Physical device error");

        let surface = Surface::new(&entry, &instance, &window)?;
        let (physical_device, queue_family_index) = physical_devices
            .iter()
            .map(|physical_device| {
                unsafe { instance.get_physical_device_queue_family_properties(*physical_device) }
                    .iter()
                    .enumerate()
                    .filter_map(|(index, ref info)| {
                        let surface_support = unsafe {
                            surface
                                .loader()
                                .get_physical_device_surface_support(
                                    *physical_device,
                                    index as u32,
                                    surface.handle(),
                                )
                                .unwrap()
                        };

                        let supports_graphic_and_surface =
                            info.queue_flags.contains(vk::QueueFlags::GRAPHICS) && surface_support;

                        if supports_graphic_and_surface {
                            Some((*physical_device, index as u32))
                        } else {
                            None
                        }
                    })
                    .next()
            })
            .filter_map(|v| v)
            .next()
            .expect("Could not find suitable device.");

        let device_extension_names_raw = [Swapchain::name().as_ptr()];
        let features = vk::PhysicalDeviceFeatures {
            shader_clip_distance: 1,
            ..Default::default()
        };
        let priorities = [1.0];

        let queue_info = [vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(queue_family_index)
            .queue_priorities(&priorities)
            .build()];

        let device_create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queue_info)
            .enabled_extension_names(&device_extension_names_raw)
            .enabled_features(&features);

        let device: Device =
            unsafe { instance.create_device(physical_device, &device_create_info, None) }.unwrap();

        let allocator_create_info = vk_mem::AllocatorCreateInfo {
            physical_device,
            device: device.clone(),
            instance: instance.clone(),
            flags: Default::default(),
            preferred_large_heap_block_size: 0,
            frame_in_use_count: 0,
            heap_size_limits: None,
        };

        let allocator = vma::Allocator::new(&allocator_create_info)?;

        Ok(Self {
            instance,
            device,
            entry,
            allocator,
        })
    }

    pub fn allocator(&self) -> &vma::Allocator {
        &self.allocator
    }

    fn create_instance(
        entry: &Entry,
        settings: &Settings,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) -> Result<Instance, Box<dyn Error>> {
        let app_name = CString::new(settings.application.name.as_bytes())?;
        let engine_name = CString::new("Crius Engine")?;

        let app_info = vk::ApplicationInfo::builder()
            .application_name(&app_name)
            .application_version(vk::make_version(
                settings.application.version.major,
                settings.application.version.minor,
                settings.application.version.patch,
            ))
            .engine_name(&engine_name)
            .engine_version(vk::make_version(
                ENGINE_VERSION_MAJOR.parse()?,
                ENGINE_VERSION_MINOR.parse()?,
                ENGINE_VERSION_PATCH.parse()?,
            ))
            .api_version(vk::make_version(1, 2, 0));

        let mut extensions = settings.graphics.extensions.clone().unwrap_or(vec![]);
        extensions.extend(Surface::extensions());

        let raw_ptr_extensions = extensions.iter().map(|e| e.as_ptr()).collect::<Vec<_>>();

        let mut layers = settings.graphics.layers.clone().unwrap_or(vec![]);

        if cfg!(debug_assertions) {
            layers.push(CString::new("VK_LAYER_KHRONOS_validation")?)
        }

        let raw_ptr_layers = layers
            .iter()
            .map(|layer| layer.as_ptr())
            .collect::<Vec<_>>();

        let instance_create_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(&raw_ptr_extensions)
            .enabled_layer_names(&raw_ptr_layers);

        let instance =
            unsafe { entry.create_instance(&instance_create_info, allocation_callbacks)? };

        println!("Vulkan instance created.");
        match entry.try_enumerate_instance_version()? {
            Some(version) => {
                let major = vk::version_major(version);
                let minor = vk::version_minor(version);
                let patch = vk::version_patch(version);

                println!("Version: {}.{}.{}", major, minor, patch)
            }
            None => println!("Version: 1.0.0"),
        }

        Ok(instance)
    }

    pub fn wait_semaphores(
        &self,
        timeline_semaphores: &[TimelineSemaphore],
        wait_values: &[u64],
        timeout: u64,
    ) -> Result<(), Box<dyn Error>> {
        let semaphore_wait_info = vk::SemaphoreWaitInfo::builder()
            .semaphores(
                &timeline_semaphores
                    .iter()
                    .map(|semaphore| semaphore.handle())
                    .collect::<Vec<_>>(),
            )
            .values(wait_values)
            .build();

        Ok(unsafe {
            self.device
                .wait_semaphores(self.device.handle(), &semaphore_wait_info, timeout)?
        })
    }

    // fn select_physical_device() -> Result<vk::PhysicalDevice, Box<dyn Error>> {}
}
