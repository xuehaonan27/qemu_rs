//! This will depend on the hypervisor you run. Note that the default is TCG, which is 
//! purely emulated, so you must specify an accelerator type to take advantage of hardware virtualization.

use crate::command::builder::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccelConfig<'a> {
    // This is used to enable an accelerator. Depending on  the  target
    // architecture,  kvm,  xen,  hax,  hvf,  nvmm,  whpx or tcg can be
    // available. By default, tcg is used. If there is  more  than  one
    // accelerator  specified, the next one is used if the previous one
    // fails to initialize.
    pub name: &'a str,

    // igd-passthru=on|off
    // When Xen is in use, this option  controls  whether  Intel
    // integrated  graphics devices can be passed through to the
    // guest (default=off)
    #[serde(rename = "igd-passthru", skip_serializing_if = "Option::is_none")]
    pub igd_passthru: Option<&'a str>,

    // kernel-irqchip=on|off|split
    // Controls KVM in-kernel irqchip support.  The  default  is
    // full  acceleration  of the interrupt controllers. On x86,
    // split irqchip reduces the kernel  attack  surface,  at  a
    // performance  cost  for  non-MSI interrupts. Disabling the
    // in-kernel irqchip completely is  not  recommended  except
    // for debugging purposes.
    #[serde(rename = "kernel-irqchip", skip_serializing_if = "Option::is_none")]
    pub kernel_irqchip: Option<&'a str>,

    // kvm-shadow-mem=size
    // Defines the size of the KVM shadow MMU.
    #[serde(rename = "kvm-shadow-mem", skip_serializing_if = "Option::is_none")]
    pub kvm_shadow_mem: Option<usize>,

    // split-wx=on|off
    // Controls  the  use  of split w^x mapping for the TCG code
    // generation buffer. Some operating systems require this to
    // be  enabled,  and in such a case this will default on. On
    // other operating systems, this will default off,  but  one
    // may enable this for testing or debugging.
    #[serde(rename = "split-wx", skip_serializing_if = "Option::is_none")]
    pub split_wx: Option<&'a str>,

    // tb-size=n
    // Controls  the  size (in MiB) of the TCG translation block
    // cache.
    #[serde(rename = "tb-size", skip_serializing_if = "Option::is_none")]
    pub tb_size: Option<usize>,

    // thread=single|multi
    // Controls  number  of  TCG  threads.  When  the   TCG   is
    // multi-threaded  there  will be one thread per vCPU there‐
    // fore taking advantage of additional host cores.  The  de‐
    // fault   is  to  enable  multi-threading  where  both  the
    // back-end and front-ends support it  and  no  incompatible
    // TCG features have been enabled (e.g.  icount/replay).
    #[serde(rename = "thread", skip_serializing_if = "Option::is_none")]
    pub thread: Option<&'a str>,

    // dirty-ring-size=n
    // When the KVM accelerator is used, it controls the size of
    // the per-vCPU dirty page ring buffer  (number  of  entries
    // for  each  vCPU).  It  should be a value that is power of
    // two, and it should be 1024 or bigger (but still less than
    // the  maximum value that the kernel supports).  4096 could
    // be a good initial value if you have no idea which is  the
    // best.   Set  this  value to 0 to disable the feature.  By
    // default, this feature  is  disabled  (dirty-ring-size=0).
    // When  enabled,  KVM  will instead record dirty pages in a
    // bitmap.
    #[serde(rename = "dirty-ring-size", skip_serializing_if = "Option::is_none")]
    pub dirty_ring_size: Option<usize>,
}

impl<'a> AccelConfig<'a> {
    #[inline]
    fn name(&self) -> Option<KVArgQ<'a>> {
        Some(KVArgQ {
            key: &self.name,
            kv_split_with: None,
            value: None,
        })
    }

    #[inline]
    fn igd_passthru(&self) -> Option<KVArgQ<'a>> {
        match self.igd_passthru {
            None => None,
            Some(igd_passthru) => Some(KVArgQ {
                key: "igd-passthru",
                kv_split_with: Some("="),
                value: Some(igd_passthru.to_string()),
            }),
        }
    }

    #[inline]
    fn kernel_irqchip(&self) -> Option<KVArgQ<'a>> {
        match self.kernel_irqchip {
            None => None,
            Some(kernel_irqchip) => Some(KVArgQ {
                key: "kernel-irqchip",
                kv_split_with: Some("="),
                value: Some(kernel_irqchip.to_string()),
            }),
        }
    }

    #[inline]
    fn kvm_shadow_mem(&self) -> Option<KVArgQ<'a>> {
        match self.kvm_shadow_mem {
            None => None,
            Some(kvm_shadow_mem) => Some(KVArgQ {
                key: "kvm-shadow-mem",
                kv_split_with: Some("="),
                value: Some(kvm_shadow_mem.to_string()),
            }),
        }
    }

    #[inline]
    fn split_wx(&self) -> Option<KVArgQ<'a>> {
        match self.split_wx {
            None => None,
            Some(split_wx) => Some(KVArgQ {
                key: "split-wx",
                kv_split_with: Some("="),
                value: Some(split_wx.to_string()),
            }),
        }
    }

    #[inline]
    fn tb_size(&self) -> Option<KVArgQ<'a>> {
        match self.tb_size {
            None => None,
            Some(tb_size) => Some(KVArgQ {
                key: "tb-size",
                kv_split_with: Some("="),
                value: Some(tb_size.to_string()),
            }),
        }
    }

    #[inline]
    fn thread(&self) -> Option<KVArgQ<'a>> {
        match self.thread {
            None => None,
            Some(thread) => Some(KVArgQ {
                key: "thread",
                kv_split_with: Some("="),
                value: Some(thread.to_string()),
            }),
        }
    }

    #[inline]
    fn dirty_ring_size(&self) -> Option<KVArgQ<'a>> {
        match self.dirty_ring_size {
            None => None,
            Some(dirty_ring_size) => Some(KVArgQ {
                key: "dirty-ring-size",
                kv_split_with: Some("="),
                value: Some(dirty_ring_size.to_string()),
            }),
        }
    }
}

impl<'a> OptionFormatting<'a> for AccelConfig<'a> {
    fn formatting(&self) -> OptionQ<'a> {
        OptionQ {
            prefix: "-",
            raw: "accel",
            option_args_split_with: " ",
            args_split_with: ",",
            args: vec![
                self.name(),
                self.igd_passthru(),
                self.kernel_irqchip(),
                self.kvm_shadow_mem(),
                self.split_wx(),
                self.tb_size(),
                self.thread(),
                self.dirty_ring_size(),
            ]
            .into_iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect(),
        }
    }
}
