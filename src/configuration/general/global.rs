//! Set default value of driver's property prop to value
use crate::command::builder::*;
use serde::{Deserialize, Serialize};

/// Set default value of driver's property prop to value, e.g.:
///     qemu-system-x86_64 -global ide-hd.physical_block_size=4096 disk-image.img
/// In particular, you can use this to set driver properties for devices which are  created
/// automatically  by  the machine model. To create a device which is not created automati‐
/// cally and set properties on it, use -device.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig<'a> {
    #[serde(rename="driver")]
    driver: &'a str,

    #[serde(rename="property")]
    property: &'a str,

    #[serde(rename="value")]
    value: &'a str,
}

impl<'a> OptionFormatting<'a> for GlobalConfig<'a> {
    fn formatting(&self) -> OptionQ<'a> {
        todo!()
    }
}