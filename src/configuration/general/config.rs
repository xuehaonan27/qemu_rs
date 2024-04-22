use crate::command::builder::*;
use serde::{Deserialize, Serialize};

use super::{
    accel::AccelConfig, add_fd::AddFdConfig, audio::{AudioConfig, AudioDevConfig}, boot::BootConfig, cpus::x86_64::CpuConfig, device::DeviceConfig, global::GlobalConfig, language::LanguageConfig, machine::MachineConfig, memory::{MConfig, MemPathConfig, MemPreallocConfig}, name::{NameConfig, UuidConfig}, numa::NumaConfig, set::SetConfig, smp::SmpConfig
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config<'a> {
    #[serde(rename = "qemu", borrow)]
    pub qemu: &'a str,

    #[serde(rename = "machine", borrow)]
    pub machine_config: Option<MachineConfig<'a>>,

    #[serde(rename = "cpu", borrow)]
    pub cpu_config: Option<CpuConfig<'a>>,

    #[serde(rename = "accel", borrow)]
    pub accel_config: Option<AccelConfig<'a>>,

    #[serde(rename = "smp")]
    pub smp_config: Option<SmpConfig>,

    #[serde(rename = "numa", borrow)]
    pub numa_config: Option<NumaConfig<'a>>,

    #[serde(rename = "add-fd", borrow)]
    pub add_fd_config: Option<AddFdConfig<'a>>,

    #[serde(rename="set", borrow)]
    pub set_config: Option<SetConfig<'a>>,

    #[serde(rename="global", borrow)]
    pub global_config: Option<GlobalConfig<'a>>,

    #[serde(rename="boot", borrow)]
    pub boot_config: Option<BootConfig<'a>>,

    #[serde(rename="m")]
    pub m_config: Option<MConfig>,

    #[serde(rename="mem-path", borrow)]
    pub mem_path_config: Option<MemPathConfig<'a>>,

    #[serde(rename="mem-prealloc")]
    pub mem_prealloc_config: Option<MemPreallocConfig>,

    #[serde(rename="k", borrow)]
    pub language_config: Option<LanguageConfig<'a>>,

    #[serde(rename="audio", borrow)]
    pub audio_config: Option<AudioConfig<'a>>,

    #[serde(rename="audiodev", borrow)]
    pub audiodev_config: Option<AudioDevConfig<'a>>,

    #[serde(rename="device", borrow)]
    pub device_config: Option<DeviceConfig<'a>>,

    #[serde(rename="name", borrow)]
    pub name_config: Option<NameConfig<'a>>,

    #[serde(rename="uuid", borrow)]
    pub uuid_config: Option<UuidConfig<'a>>,
}

impl<'a> Config<'a> {
    #[inline]
    fn f<T: OptionFormatting<'a>>(x: &Option<T>) -> Option<OptionQ<'a>> {
        match x {
            None => None,
            Some(t) => Some(t.formatting()),
        }
    }
}

impl<'a> CommandFormatting<'a> for Config<'a> {
    fn formatting(&self) -> CommandQ<'a> {
        CommandQ {
            raw: &self.qemu,
            option_split_with: " ",
            options: vec![
                Self::f(&self.machine_config),
                Self::f(&self.cpu_config),
                Self::f(&self.accel_config),
                Self::f(&self.smp_config),
                Self::f(&self.numa_config),
                Self::f(&self.add_fd_config),
                Self::f(&self.set_config),
                Self::f(&self.global_config),
                Self::f(&self.boot_config),
                Self::f(&self.m_config),
                Self::f(&self.mem_path_config),
                Self::f(&self.mem_prealloc_config),
                Self::f(&self.language_config),
            ]
            .into_iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect(),
        }
    }
}
