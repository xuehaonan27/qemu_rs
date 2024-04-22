//! Add a file descriptor to an fd set.
use crate::command::builder::*;
use serde::{Deserialize, Serialize};

/// Add a file descriptor to an fd set.
/// You can open an image using pre-opened file descriptors from an fd set:
///     qemu-system-x86_64 \
///     -add-fd fd=3,set=2,opaque="rdwr:/path/to/file" \
///     -add-fd fd=4,set=2,opaque="rdonly:/path/to/file" \
///     -drive file=/dev/fdset/2,index=0,media=disk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddFdConfig<'a> {
    // This option defines the file descriptor of which a duplicate is added to fd set.
    // The file descriptor cannot be stdin, stdout, or stderr.
    #[serde(rename = "fd")]
    fd: usize,

    // This option defines the ID of the fd set to add the file descriptor to.
    #[serde(rename = "set")]
    set: usize,

    // This option defines a free-form string that can be used to describe fd.
    #[serde(rename = "opaque", skip_serializing_if = "Option::is_none")]
    opaque: Option<&'a str>,
}

impl<'a> AddFdConfig<'a> {
    #[inline]
    fn fd(&self) -> Option<KVArgQ<'a>> {
        Some(KVArgQ {
            key: "fd",
            kv_split_with: Some("="),
            value: Some(self.fd.to_string()),
        })
    }

    #[inline]
    fn set(&self) -> Option<KVArgQ<'a>> {
        Some(KVArgQ {
            key: "set",
            kv_split_with: Some("="),
            value: Some(self.set.to_string()),
        })
    }

    #[inline]
    fn opaque(&self) -> Option<KVArgQ<'a>> {
        match self.opaque {
            None => None,
            Some(opaque) => Some(KVArgQ {
                key: "opaque",
                kv_split_with: Some("="),
                value: Some(opaque.to_string()),
            }),
        }
    }
}

impl<'a> OptionFormatting<'a> for AddFdConfig<'a> {
    fn formatting(&self) -> OptionQ<'a> {
        OptionQ {
            prefix: "-",
            raw: "add-fd",
            option_args_split_with: " ",
            args_split_with: ",",
            args: vec![self.fd(), self.set(), self.opaque()]
                .into_iter()
                .filter(|x| x.is_some())
                .map(|x| x.unwrap())
                .collect(),
        }
    }
}
