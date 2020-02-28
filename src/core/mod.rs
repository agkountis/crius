pub mod application;
pub mod ecs;
pub mod event;
pub mod scene;
pub mod window;

use crate::core::window::WindowMode;
use nalgebra::Vector2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

#[derive(Debug)]
pub struct Settings {
    pub name: String,
    pub version: Version,
    pub window_size: Vector2<u32>,
    pub window_mode: WindowMode,
    pub vsync: bool,
}
