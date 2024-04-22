//! Type and number/topology of vCPUs, Most accelerators offer a host cpu option which
//! simply passes through your host CPU configuration without filtering out any features.
use crate::command::builder::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuConfig<'a> {
    test: &'a str,
}

impl<'a> OptionFormatting<'a> for CpuConfig<'a> {
    fn formatting(&self) -> OptionQ<'a> {
        todo!()
    }
}