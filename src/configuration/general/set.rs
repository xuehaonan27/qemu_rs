//! Set parameter arg for item id of type group
use crate::command::builder::*;
use serde::{Deserialize, Serialize};

/// Set parameter arg for item id of type group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetConfig<'a> {
    #[serde(rename = "group.id.arg")]
    group_id_arg: &'a str,
}

impl<'a> OptionFormatting<'a> for SetConfig<'a> {
    fn formatting(&self) -> OptionQ<'a> {
        OptionQ {
            prefix: "-",
            raw: "set",
            option_args_split_with: " ",
            args_split_with: "",
            args: vec![KVArgQ {
                key: "group.id.arg",
                kv_split_with: Some("="),
                value: Some(self.group_id_arg.to_string()),
            }],
        }
    }
}
