use sysinfo::{Disks, System, Pid};
use local_ip_address::{local_ip, list_afinet_netifas};
use serde_json::json;
use serde_json::Value;
use std::thread;
use std::time::Duration;
use clap::{Arg, Command};

#[allow(dead_code)]
fn generate_sysinfo() {
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

    println!("{}", payload.to_string());
}

#[allow(dead_code)]
fn generate_process_list() {
    let sys = System::new_all();

    for (_, process) in sys.processes() {
        let disk_usage = process.disk_usage();

        let prc = json!({
            "pid": process.pid().as_u32(),
            "parent": process.parent().unwrap_or(Pid::from_u32(0)).as_u32(),
            "name": process.name(),
            "exe_path": process.exe(),
            "mem_usage_bytes": process.memory(),
            "virt_mem_usage_bytes": process.virtual_memory(),
            "cpu_usage": process.cpu_usage(),
            "status": format!("{}", process.status()),
            "start_time_sec": process.start_time(),
            "run_time_sec": process.run_time(),
            "disk_usage_read_bytes": disk_usage.read_bytes,
            "disk_usage_total_read_bytes": disk_usage.total_read_bytes,
            "disk_usage_written_bytes": disk_usage.written_bytes,
            "disk_usage_total_written_bytes": disk_usage.total_written_bytes,
            "process_user_id": format!("{:?}", process.user_id().unwrap()),
            "process_effective_user_id": format!("{:?}", process.effective_user_id().unwrap()),
            "process_group_id": format!("{:?}", process.group_id().unwrap())
        });

        let payload = json!({
            "sysinfo_data_type": "process_record",
            "process": prc
        });

        println!("{}", payload.to_string());
    }

}

#[allow(dead_code)]
fn generate_user_list() {

    let users: Vec<Value> = Vec::new();

    let payload = json!({
        "sysinfo_data_type": "user_list",
        "users": users
    });

    println!("{}", payload.to_string());
}

fn main() {

    let matches = Command::new("sysinfo-rs")
    .version("0.1.0")
    .author("Max Skybin <max@observeinc.com>")
    .about("Small helper to get system info and print it to stdout in JSON format")
    .arg(Arg::new("processes")
            .short('p')
            .long("processes")
            .required(false)
            .help("Print list of running processes"))
    .arg(Arg::new("users")
            .short('u')
            .long("users")
            .required(false)
            .help("Print list of users on this system"))
    .get_matches();

    let processes = matches.get_one::<bool>("processes").unwrap_or(&false);
    let users = matches.get_one::<bool>("users").unwrap_or(&false);

	loop {
        generate_sysinfo();
        if *processes {
            generate_process_list();
        }
        if *users {
            generate_user_list();
        }
        
        thread::sleep(Duration::from_secs(300));
	}
}