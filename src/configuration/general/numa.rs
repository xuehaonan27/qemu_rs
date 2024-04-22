use crate::command::builder::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumaConfig<'a> {
    test: &'a str,
}

impl<'a> OptionFormatting<'a> for NumaConfig<'a> {
    fn formatting(&self) -> OptionQ<'a> {
        todo!()
    }
}