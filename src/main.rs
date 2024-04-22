use std::{error::Error, fs::File, io::Read, path::Path};

use qemu_rs::{
    command::builder::{CommandBuild, CommandFormatting},
    configuration::general::*,
};

fn main() {
    println!("Hello, world!");

    let config = config::Config {
        qemu: "qemu-system-x86_64",
        machine_config: Some(machine::MachineConfig {
            m_type: "pc-i440fx-jammy",
            accel: Some(vec!["kvm", "xen", "hax"]),
            vmport: None,
            dump_guest_core: None,
            mem_merge: Some("on"),
            aes_key_wrap: None,
            dea_key_wrap: None,
            nvdimm: None,
            memory_encryption: None,
            hmat: None,
            memory_backend: None,
            sgx_epc_0_memdev: None,
            sgx_epc_0_node: None,
        }),
        cpu_config: None,
        accel_config: Some(accel::AccelConfig {
            name: "kvm",
            igd_passthru: Some("off"),
            kernel_irqchip: Some("on"),
            kvm_shadow_mem: None,
            split_wx: None,
            tb_size: Some(2),
            thread: Some("multi"),
            dirty_ring_size: None,
        }),
        smp_config: Some(smp::SmpConfig {
            cpus: Some(2),
            maxcpus: Some(4),
            drawers: None,
            books: None,
            sockets: None,
            dies: None,
            clusters: None,
            cores: None,
            threads: None,
        }),
        numa_config: None,
        add_fd_config: None,
        set_config: None,
        global_config: None,
        boot_config: None,
        m_config: None,
        mem_path_config: None,
        mem_prealloc_config: None,
        language_config: None,
        audio_config: None,
        audiodev_config: None,
        device_config: None,
        name_config: None,
        uuid_config: None,
    };

    println!("{}", config.formatting().to_string());

    let mut s = String::new();
    let c = read_from_file("./config.json", &mut s).unwrap();
    println!("{}", c.formatting().to_string());

    assert_eq!(config.formatting().to_string(), c.formatting().to_string());
}

fn read_from_file<'a, P: AsRef<Path>>(
    path: P,
    string: &'a mut String,
) -> Result<config::Config<'a>, Box<dyn Error>> {
    let mut file = File::open(path)?;
    file.read_to_string(string)?;
    let c: config::Config = serde_json::from_str(string)?;
    Ok(c)
}
