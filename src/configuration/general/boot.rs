//! 
use crate::command::builder::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootConfig<'a> {
    #[serde(rename="menu", skip_serializing_if="Option::is_none")]
    menu: Option<&'a str>,
}

impl<'a> OptionFormatting<'a> for BootConfig<'a> {
    fn formatting(&self) -> OptionQ<'a> {
        todo!()
    }
}