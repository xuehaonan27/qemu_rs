use crate::command::builder::*;
use serde::{Deserialize, Serialize};

/// Name of the vm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NameConfig<'a> {
    // string1 sets the window title.
    #[serde(rename = "window-title")]
    window_title: &'a str,

    // string2 the process name.
    #[serde(rename = "process", skip_serializing_if = "Option::is_none")]
    process: Option<&'a str>,

    // debug-threads=on|off
    // When debug-threads is enabled, individual threads are given a separate name
    // NOTE: The thread names are for debugging and not a stable API.
    #[serde(rename = "debug-threads", skip_serializing_if = "Option::is_none")]
    debug_threads: Option<&'a str>,
}

impl<'a> NameConfig<'a> {
    #[inline]
    fn window_title(&self) -> Option<KVArgQ<'a>> {
        Some(KVArgQ {
            key: &self.window_title,
            kv_split_with: None,
            value: None,
        })
    }

    #[inline]
    fn process(&self) -> Option<KVArgQ<'a>> {
        match self.process {
            None => None,
            Some(process) => Some(KVArgQ {
                key: "process",
                kv_split_with: Some("="),
                value: Some(process.to_string()),
            }),
        }
    }

    #[inline]
    fn debug_threads(&self) -> Option<KVArgQ<'a>> {
        match self.debug_threads {
            None => None,
            Some(debug_threads) => Some(KVArgQ {
                key: "debug-threads",
                kv_split_with: Some("="),
                value: Some(debug_threads.to_string()),
            }),
        }
    }
}

impl<'a> OptionFormatting<'a> for NameConfig<'a> {
    fn formatting(&self) -> OptionQ<'a> {
        OptionQ {
            prefix: "-",
            raw: "name",
            option_args_split_with: " ",
            args_split_with: ",",
            args: vec![self.window_title(), self.process(), self.debug_threads()]
                .into_iter()
                .filter(|x| x.is_some())
                .map(|x| x.unwrap())
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UuidConfig<'a>(&'a str);

impl<'a> OptionFormatting<'a> for UuidConfig<'a> {
    fn formatting(&self) -> OptionQ<'a> {
        OptionQ {
            prefix: "-",
            raw: "uuid",
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_name() {
        let name_config = NameConfig {
            window_title: "qemu-testmachine",
            process: Some("qemu-simulation-process1"),
            debug_threads: Some("off"),
        };

        assert_eq!(
            name_config.formatting().to_string(),
            "-name qemu-testmachine,process=qemu-simulation-process1,debug-threads=off"
        )
    }

    #[test]
    fn test_uuid() {
        let uuid_config = UuidConfig("12345678-1234-1234-1234-123456789abc");

        assert_eq!(
            uuid_config.formatting().to_string(),
            "-uuid 12345678-1234-1234-1234-123456789abc"
        )
    }
}
