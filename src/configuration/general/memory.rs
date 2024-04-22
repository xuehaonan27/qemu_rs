use crate::command::builder::*;
use serde::{Deserialize, Serialize};

/// Overall memory configuration.
/// Note: Some architectures might enforce a specific granularity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MConfig {
    // size: initial amount of guest memory (in MiB).
    #[serde(rename = "size")]
    pub(crate) size: usize,

    // slots: number of hotplug slots.
    // (default: none)
    #[serde(rename = "slots", skip_serializing_if = "Option::is_none")]
    pub(crate) slots: Option<usize>,

    // maxmem: maximum amount of guest memory.
    // (default: none)
    #[serde(rename = "maxmem", skip_serializing_if = "Option::is_none")]
    pub(crate) maxmem: Option<usize>,
}

impl<'a> MConfig {
    #[inline]
    fn size(&self) -> Option<KVArgQ<'a>> {
        Some(KVArgQ {
            key: "size",
            kv_split_with: Some("="),
            value: Some(self.size.to_string()),
        })
    }

    #[inline]
    fn slots(&self) -> Option<KVArgQ<'a>> {
        match self.slots {
            None => None,
            Some(slots) => Some(KVArgQ {
                key: "slots",
                kv_split_with: Some("="),
                value: Some(slots.to_string()),
            }),
        }
    }

    #[inline]
    fn maxmem(&self) -> Option<KVArgQ<'a>> {
        match self.maxmem {
            None => None,
            Some(maxmem) => Some(KVArgQ {
                key: "maxmem",
                kv_split_with: Some("="),
                value: Some(maxmem.to_string()),
            }),
        }
    }
}

impl<'a> OptionFormatting<'a> for MConfig {
    fn formatting(&self) -> OptionQ<'a> {
        OptionQ {
            prefix: "-",
            raw: "m",
            option_args_split_with: " ",
            args_split_with: ",",
            args: vec![self.size(), self.slots(), self.maxmem()]
                .into_iter()
                .filter(|x| x.is_some())
                .map(|x| x.unwrap())
                .collect(),
        }
    }
}

/// Provide backing storage for guest RAM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemPathConfig<'a>(&'a str);

impl<'a> OptionFormatting<'a> for MemPathConfig<'a> {
    fn formatting(&self) -> OptionQ<'a> {
        OptionQ {
            prefix: "-",
            raw: "mem-path",
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

/// Preallocate guest memory (use with -mem-path)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemPreallocConfig(usize);

impl<'a> OptionFormatting<'a> for MemPreallocConfig {
    fn formatting(&self) -> OptionQ<'a> {
        OptionQ {
            prefix: "-",
            raw: "mem-prealloc",
            option_args_split_with: " ",
            args_split_with: "",
            args: vec![KVArgQ {
                key: "",
                kv_split_with: None,
                value: Some(self.0.to_string()),
            }],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::{error::Error, fs::File, io::Read, path::Path};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct Temp<'a> {
        #[serde(rename = "m")]
        m_config: MConfig,

        #[serde(rename = "mem-path", borrow)]
        mem_path_config: MemPathConfig<'a>,

        #[serde(rename = "mem-prealloc")]
        mem_prealloc_config: MemPreallocConfig,
    }

    impl<'a> CommandFormatting<'a> for Temp<'a> {
        fn formatting(&self) -> CommandQ<'a> {
            CommandQ {
                raw: "",
                option_split_with: " ",
                options: vec![
                    self.m_config.formatting(),
                    self.mem_path_config.formatting(),
                    self.mem_prealloc_config.formatting(),
                ],
            }
        }
    }

    fn read_from_file<'a, P: AsRef<Path>>(
        path: P,
        string: &'a mut String,
    ) -> Result<Temp<'a>, Box<dyn Error>> {
        let mut file = File::open(path)?;
        file.read_to_string(string)?;
        let c: Temp<'a> = serde_json::from_str(string)?;
        Ok(c)
    }

    #[test]
    fn test_m() {
        let m_config = MConfig {
            size: 512,
            slots: Some(2),
            maxmem: Some(1024),
        };
        assert_eq!(
            m_config.formatting().to_string(),
            "-m size=512,slots=2,maxmem=1024"
        )
    }

    #[test]
    fn test_mem_path() {
        let mem_path_config = MemPathConfig("/mem_backend_storage/mem.1");
        assert_eq!(
            mem_path_config.formatting().to_string(),
            "-mem-path /mem_backend_storage/mem.1"
        )
    }

    #[test]
    fn test_mem_prealloc() {
        let mem_prealloc_config = MemPreallocConfig(128);
        assert_eq!(
            mem_prealloc_config.formatting().to_string(),
            "-mem-prealloc 128"
        )
    }

    #[test]
    fn test_read_json() {
        let mut aux = String::new();
        let memory_1 = read_from_file("./test_json/memory.json", &mut aux).unwrap();
        let memory_2 = Temp {
            m_config: MConfig {
                size: 512,
                slots: Some(2),
                maxmem: Some(1024),
            },
            mem_path_config: MemPathConfig("/mem_backend_storage/mem.1"),
            mem_prealloc_config: MemPreallocConfig(128),
        };

        assert_eq!(memory_1.formatting().to_string(), memory_2.formatting().to_string())
    }
}
