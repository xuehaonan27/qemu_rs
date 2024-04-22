//! According to man page of qemu.

/// Build the command into a raw string.
pub trait CommandBuild {
    fn to_string(&self) -> String;
}

pub trait OptionFormatting<'a> {
    fn formatting(&self) -> OptionQ<'a>;
}

pub trait CommandFormatting<'a> {
    fn formatting(&self) -> CommandQ<'a>;
}

pub struct CommandQ<'a> {
    // Raw string of this command.
    pub raw: &'a str,
    pub option_split_with: &'a str,
    pub options: Vec<OptionQ<'a>>,
}

impl<'a> CommandBuild for CommandQ<'a> {
    fn to_string(&self) -> String {
        self.options.iter().fold(self.raw.to_string(), |s, option| {
            format!("{}{}{}", s, self.option_split_with, option.to_string())
        })
    }
}

pub struct OptionQ<'a> {
    pub prefix: &'a str,
    // Raw string of this option.
    pub raw: &'a str,
    pub option_args_split_with: &'a str,
    pub args_split_with: &'a str,
    pub args: Vec<KVArgQ<'a>>,
}

impl<'a> CommandBuild for OptionQ<'a> {
    fn to_string(&self) -> String {
        let mut b: bool = false;
        self.args.iter().fold(
            format!("{}{}", self.prefix, self.raw.to_string()),
            |s, arg| {
                format!(
                    "{}{}{}",
                    s,
                    if b {
                        self.args_split_with
                    } else {
                        b = true;
                        self.option_args_split_with
                    },
                    arg.to_string()
                )
            },
        )
    }
}

pub struct KVArgQ<'a> {
    pub key: &'a str,
    pub kv_split_with: Option<&'a str>,
    pub value: Option<String>,
}

impl<'a> CommandBuild for KVArgQ<'a> {
    fn to_string(&self) -> String {
        match &self.value {
            Some(s) => format!("{}{}{}", self.key, self.kv_split_with.unwrap_or(""), s),
            None => format!("{}", self.key),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::command::builder::CommandBuild;

    use super::{CommandQ, KVArgQ, OptionQ};

    #[test]
    fn test_command_build_sound() {
        let cmd = CommandQ {
            raw: "qemu-system-x86_64",
            option_split_with: " ",
            options: vec![
                OptionQ {
                    prefix: "-",
                    raw: "name",
                    option_args_split_with: " ",
                    args_split_with: "",
                    args: vec![KVArgQ {
                        key: "\"testmachine\"",
                        kv_split_with: None,
                        value: None,
                    }],
                },
                OptionQ {
                    prefix: "-",
                    raw: "machine",
                    option_args_split_with: " ",
                    args_split_with: "",
                    args: vec![KVArgQ {
                        key: "accel",
                        kv_split_with: Some("="),
                        value: Some("kvm".to_string()),
                    }],
                },
                OptionQ {
                    prefix: "-",
                    raw: "M",
                    option_args_split_with: " ",
                    args_split_with: "",
                    args: vec![KVArgQ {
                        key: "pc",
                        kv_split_with: None,
                        value: None,
                    }],
                },
                OptionQ {
                    prefix: "-",
                    raw: "m",
                    option_args_split_with: " ",
                    args_split_with: "",
                    args: vec![KVArgQ {
                        key: "768",
                        kv_split_with: None,
                        value: None,
                    }],
                },
                OptionQ {
                    prefix: "-",
                    raw: "smp",
                    option_args_split_with: " ",
                    args_split_with: "",
                    args: vec![KVArgQ {
                        key: "2",
                        kv_split_with: None,
                        value: None,
                    }],
                },
                OptionQ {
                    prefix: "-",
                    raw: "boot",
                    option_args_split_with: " ",
                    args_split_with: "",
                    args: vec![KVArgQ {
                        key: "d",
                        kv_split_with: None,
                        value: None,
                    }],
                },
                OptionQ {
                    prefix: "-",
                    raw: "drive",
                    option_args_split_with: " ",
                    args_split_with: ",",
                    args: vec![
                        KVArgQ {
                            key: "file",
                            kv_split_with: Some("="),
                            value: Some("/images/sles/hda".to_string()),
                        },
                        KVArgQ {
                            key: "if",
                            kv_split_with: Some("="),
                            value: Some("virtio".to_string()),
                        },
                        KVArgQ {
                            key: "index",
                            kv_split_with: Some("="),
                            value: Some("0".to_string()),
                        },
                        KVArgQ {
                            key: "media",
                            kv_split_with: Some("="),
                            value: Some("disk".to_string()),
                        },
                        KVArgQ {
                            key: "format",
                            kv_split_with: Some("="),
                            value: Some("raw".to_string()),
                        },
                    ],
                },
                OptionQ {
                    prefix: "-",
                    raw: "drive",
                    option_args_split_with: " ",
                    args_split_with: ",",
                    args: vec![
                        KVArgQ {
                            key: "file",
                            kv_split_with: Some("="),
                            value: Some("/isos/image.iso".to_string()),
                        },
                        KVArgQ {
                            key: "index",
                            kv_split_with: Some("="),
                            value: Some("1".to_string()),
                        },
                        KVArgQ {
                            key: "media",
                            kv_split_with: Some("="),
                            value: Some("cdrom".to_string()),
                        },
                    ],
                },
                OptionQ {
                    prefix: "-",
                    raw: "net",
                    option_args_split_with: " ",
                    args_split_with: ",",
                    args: vec![
                        KVArgQ {
                            key: "nic",
                            kv_split_with: None,
                            value: None,
                        },
                        KVArgQ {
                            key: "model",
                            kv_split_with: Some("="),
                            value: Some("virtio".to_string()),
                        },
                        KVArgQ {
                            key: "macaddr",
                            kv_split_with: Some("="),
                            value: Some("52:54:00:05:11:11".to_string()),
                        },
                    ],
                },
                OptionQ {
                    prefix: "-",
                    raw: "net",
                    option_args_split_with: " ",
                    args_split_with: ",",
                    args: vec![KVArgQ {
                        key: "user",
                        kv_split_with: None,
                        value: None,
                    }],
                },
                OptionQ {
                    prefix: "-",
                    raw: "vga",
                    option_args_split_with: " ",
                    args_split_with: "",
                    args: vec![KVArgQ {
                        key: "cirrus",
                        kv_split_with: None,
                        value: None,
                    }],
                },
                OptionQ {
                    prefix: "-",
                    raw: "balloon",
                    option_args_split_with: " ",
                    args_split_with: "",
                    args: vec![KVArgQ {
                        key: "virtio",
                        kv_split_with: None,
                        value: None,
                    }],
                },
            ],
        };
        assert_eq!(cmd.to_string(),
        "qemu-system-x86_64 -name \"testmachine\" -machine accel=kvm -M pc -m 768 -smp 2 -boot d -drive file=/images/sles/hda,if=virtio,index=0,media=disk,format=raw -drive file=/isos/image.iso,index=1,media=cdrom -net nic,model=virtio,macaddr=52:54:00:05:11:11 -net user -vga cirrus -balloon virtio")
    }
}
