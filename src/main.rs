use sysinfo::{Disks, System};
use local_ip_address::{local_ip, list_afinet_netifas};
use serde_json::json;
use serde_json::Value;
use std::thread;
use std::time::Duration;

fn get_stats() -> Value {
    let mut sys = System::new_all();
    sys.refresh_all();

    let disk_arr = Disks::new_with_refreshed_list();
    let mut disks: Vec<Value> = Vec::new();

    for disk in &disk_arr {
        disks.push(json!({
            "disk_name": disk.name().to_str(),
            "disk_kind": format!("{}", disk.kind()),
            "disk_filesystem": disk.file_system().to_str(),
            "disk_mountpoint": disk.mount_point().to_str(),
            "disk_total_space": disk.total_space(),
        }));
    }

    let interface_arr = list_afinet_netifas().unwrap();
    let mut interfaces: Vec<Value> = Vec::new();

    for (name, ip) in interface_arr.iter() {
        interfaces.push(json!({
            "interface_name": name,
            "interface_ip": ip
        }));
    }

    let payload = json!({
        "system": {
            "hostname": System::host_name(),
            "os_name": System::name(),
            "os_kernel_version": System::kernel_version(),
            "os_version": System::os_version()
        },
        "memory": {
            "mem_total": sys.total_memory(),
            "swap_total": sys.total_swap()
        },
        "cpu": {
            "num_cpus": sys.cpus().len(),
            "cpu_name": sys.cpus()[0].name(),
            "cpu_brand": sys.cpus()[0].brand(),
            "cpu_frequency": sys.cpus()[0].frequency()
        },
        "disks": disks,
        "network": {
            "primary_ip": local_ip().unwrap(),
        },
        "interfaces": interfaces
    });

    return payload;
}


fn main() {
	loop {
        println!("{}", get_stats().to_string());
        //println!("{}", serde_json::to_string_pretty(&get_stats()).unwrap());
        thread::sleep(Duration::from_secs(60));
	}
}
