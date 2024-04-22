use crate::command::builder::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig<'a> {
    test: &'a str,
}

impl<'a> OptionFormatting<'a> for AudioConfig<'a> {
    fn formatting(&self) -> OptionQ<'a> {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDevConfig<'a> {
    test: &'a str,
}

impl<'a> OptionFormatting<'a> for AudioDevConfig<'a> {
    fn formatting(&self) -> OptionQ<'a> {
        todo!()
    }
}