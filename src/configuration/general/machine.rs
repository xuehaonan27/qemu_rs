//! Define the machine type, amount of memory etc.
use crate::command::builder::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineConfig<'a> {
    // Select the emulated machine by name.
    #[serde(rename = "type")]
    pub m_type: &'a str,

    // accel=accels1[:accels2[:...]]
    // (default: tcg)
    // 
    // This is used to enable an accelerator. Depending  on  the
    // target  architecture,  kvm,  xen, hax, hvf, nvmm, whpx or
    // tcg can be available.  By default, tcg is used. If  there
    // is  more  than one accelerator specified, the next one is
    // used if the previous one fails to initialize.
    #[serde(rename = "accel", skip_serializing_if = "Option::is_none")]
    pub accel: Option<Vec<&'a str>>,

    // vmport=on|off|auto
    // (default: auto)
    // 
    // Enables emulation of VMWare IO  port,  for  vmmouse  etc.
    // auto  says  to  select  the value based on accel. For ac‐
    // cel=xen the default is off otherwise the default is on.
    #[serde(rename = "vmport", skip_serializing_if = "Option::is_none")]
    pub vmport: Option<&'a str>,

    // dump-guest-core=on|off
    // (default=on)
    // 
    // Include guest memory in a core dump. The default is on.
    #[serde(rename = "dump-guest-core", skip_serializing_if = "Option::is_none")]
    pub dump_guest_core: Option<&'a str>,

    // mem-merge=on|off
    // (default: on)
    //
    // Enables or disables memory merge support.  This  feature,
    // when  supported by the host, de-duplicates identical mem‐
    // ory pages among VMs instances (enabled by default).
    #[serde(rename = "mem-merge", skip_serializing_if = "Option::is_none")]
    pub mem_merge: Option<&'a str>,

    // aes-key-wrap=on|off
    // (default=on)
    // 
    // Enables or disables AES key wrapping support on  s390-ccw
    // hosts.   This  feature controls whether AES wrapping keys
    // will be created to allow execution of  AES  cryptographic
    // functions. The default is on.
    #[serde(rename = "aes-key-wrap", skip_serializing_if = "Option::is_none")]
    pub aes_key_wrap: Option<&'a str>,

    // dea-key-wrap=on|off
    // (default=on)
    // 
    // Enables  or disables DEA key wrapping support on s390-ccw
    // hosts.  This feature controls whether DEA  wrapping  keys
    // will  be  created to allow execution of DEA cryptographic
    // functions. The default is on.
    //
    // ory pages among VMs instances (enabled by default).
    #[serde(rename = "dea-key-wrap", skip_serializing_if = "Option::is_none")]
    pub dea_key_wrap: Option<&'a str>,

    // nvdimm=on|off
    // (default=off)
    //
    // Enables or disables NVDIMM support. The default is off.
    #[serde(rename = "nvdimm", skip_serializing_if = "Option::is_none")]
    pub nvdimm: Option<&'a str>,

    // memory-encryption=
    // (default=none)
    //
    // Memory encryption object to use. The default is none.
    #[serde(rename = "memory-encryption", skip_serializing_if = "Option::is_none")]
    pub memory_encryption: Option<&'a str>,

    // hmat=on|off
    // (default=off)
    //
    // Enables or disables ACPI Heterogeneous  Memory  Attribute
    // Table (HMAT) support. The default is off.
    #[serde(rename = "hmat", skip_serializing_if = "Option::is_none")]
    pub hmat: Option<&'a str>,

    // memory-backend='id'
    // (default=none)
    // 
    // Allows to use a memory backend as main RAM.
    // For example:
    //     -object memory-backend-file,id=pc.ram,size=512M,mem-path=/hugetlbfs,prealloc=on,share=on
    //     -machine memory-backend=pc.ram
    //     -m 512M
    #[serde(rename = "memory-backend", skip_serializing_if = "Option::is_none")]
    pub memory_backend: Option<&'a str>,

    // memid
    #[serde(rename = "sgx-epc.0.memdev", skip_serializing_if="Option::is_none")]
    pub sgx_epc_0_memdev: Option<usize>,

    // numaid
    #[serde(rename = "sgx-epc.0.node", skip_serializing_if="Option::is_none")]
    pub sgx_epc_0_node: Option<usize>
}

impl<'a> MachineConfig<'a> {
    #[inline]
    fn m_type(&self) -> Option<KVArgQ<'a>> {
        Some(KVArgQ {
            key: "type",
            kv_split_with: Some("="),
            value: Some(self.m_type.to_string()),
        })
    }

    #[inline]
    fn accel(&self) -> Option<KVArgQ<'a>> {
        match &self.accel {
            None => None,
            Some(accel) => Some(KVArgQ {
                key: "accel",
                kv_split_with: Some("="),
                value: Some(accel.join(":")),
            }),
        }
    }

    #[inline]
    fn vmport(&self) -> Option<KVArgQ<'a>> {
        match self.vmport {
            None => None,
            Some(vmport) => Some(KVArgQ {
                key: "vmport",
                kv_split_with: Some("="),
                value: Some(vmport.to_string()),
            }),
        }
    }

    #[inline]
    fn dump_guest_core(&self) -> Option<KVArgQ<'a>> {
        match self.dump_guest_core {
            None => None,
            Some(dump_guest_core) => Some(KVArgQ {
                key: "dump-guest-core",
                kv_split_with: Some("="),
                value: Some(dump_guest_core.to_string()),
            }),
        }
    }

    #[inline]
    fn mem_merge(&self) -> Option<KVArgQ<'a>> {
        match self.mem_merge {
            None => None,
            Some(mem_merge) => Some(KVArgQ {
                key: "mem-merge",
                kv_split_with: Some("="),
                value: Some(mem_merge.to_string()),
            }),
        }
    }

    #[inline]
    fn aes_key_wrap(&self) -> Option<KVArgQ<'a>> {
        match self.aes_key_wrap {
            None => None,
            Some(aes_key_wrap) => Some(KVArgQ {
                key: "aes-key-wrap",
                kv_split_with: Some("="),
                value: Some(aes_key_wrap.to_string()),
            }),
        }
    }

    #[inline]
    fn dea_key_wrap(&self) -> Option<KVArgQ<'a>> {
        match self.dea_key_wrap {
            None => None,
            Some(dea_key_wrap) => Some(KVArgQ {
                key: "dea-key-wrap",
                kv_split_with: Some("="),
                value: Some(dea_key_wrap.to_string()),
            }),
        }
    }

    #[inline]
    fn nvdimm(&self) -> Option<KVArgQ<'a>> {
        match self.nvdimm {
            None => None,
            Some(nvdimm) => Some(KVArgQ {
                key: "nvdimm",
                kv_split_with: Some("="),
                value: Some(nvdimm.to_string()),
            }),
        }
    }

    #[inline]
    fn memory_encryption(&self) -> Option<KVArgQ<'a>> {
        match self.memory_encryption {
            None => None,
            Some(memory_encryption) => Some(KVArgQ {
                key: "memory-encryption",
                kv_split_with: Some("="),
                value: Some(memory_encryption.to_string()),
            }),
        }
    }

    #[inline]
    fn hmat(&self) -> Option<KVArgQ<'a>> {
        match self.hmat {
            None => None,
            Some(hmat) => Some(KVArgQ {
                key: "hmat",
                kv_split_with: Some("="),
                value: Some(hmat.to_string()),
            }),
        }
    }

    #[inline]
    fn memory_backend(&self) -> Option<KVArgQ<'a>> {
        match self.memory_backend {
            None => None,
            Some(memory_backend) => Some(KVArgQ {
                key: "memory-backend",
                kv_split_with: Some("="),
                value: Some(memory_backend.to_string()),
            }),
        }
    }

    #[inline]
    fn sgx_epc_0_memdev(&self) -> Option<KVArgQ<'a>> {
        match self.sgx_epc_0_memdev {
            None => None,
            Some(sgx_epc_0_memdev) => Some(KVArgQ {
                key: "sgx-epc.0.memdev",
                kv_split_with: Some("="),
                value: Some(sgx_epc_0_memdev.to_string()),
            }),
        }
    }

    #[inline]
    fn sgx_epc_0_node(&self) -> Option<KVArgQ<'a>> {
        match self.sgx_epc_0_node {
            None => None,
            Some(sgx_epc_0_node) => Some(KVArgQ {
                key: "sgx-epc.0.node",
                kv_split_with: Some("="),
                value: Some(sgx_epc_0_node.to_string()),
            }),
        }
    }
}

impl<'a> OptionFormatting<'a> for MachineConfig<'a> {
    fn formatting(&self) -> OptionQ<'a> {
        OptionQ {
            prefix: "-",
            raw: "machine",
            option_args_split_with: " ",
            args_split_with: ",",
            args: vec![
                self.m_type(),
                self.accel(),
                self.vmport(),
                self.dump_guest_core(),
                self.mem_merge(),
                self.aes_key_wrap(),
                self.dea_key_wrap(),
                self.nvdimm(),
                self.memory_encryption(),
                self.hmat(),
                self.memory_backend(),
                self.sgx_epc_0_memdev(),
                self.sgx_epc_0_node(),
            ]
            .into_iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect(),
        }
    }
}
