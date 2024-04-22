use crate::command::builder::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageConfig<'a>(&'a str);

impl<'a> OptionFormatting<'a> for LanguageConfig<'a> {
    fn formatting(&self) -> OptionQ<'a> {
        OptionQ {
            prefix: "-",
            raw: "k",
            option_args_split_with: " ",
            args_split_with: "",
            args: vec![KVArgQ {
                key: self.0,
                kv_split_with: None,
                value: None,
            }],
        }
    }
}
