use crate::command::builder::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceConfig<'a> {
    test: &'a str,
}

impl<'a> OptionFormatting<'a> for DeviceConfig<'a> {
    fn formatting(&self) -> OptionQ<'a> {
        todo!()
    }
}