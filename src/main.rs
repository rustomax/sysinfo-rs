use sysinfo::MINIMUM_CPU_UPDATE_INTERVAL;
use sysinfo::{Disks, System, Pid, Users};
use local_ip_address::{local_ip, list_afinet_netifas};
use serde_json::json;
use serde_json::Value;
use std::thread;
use std::time::Duration;
use argh::FromArgs;

fn generate_sysinfo(sys: &System) {
    std::thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
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
        "sysinfo_data_type": "system_info",
        "data": {
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
        }
    });

    println!("{}", payload.to_string());
}

fn generate_process_list(sys: &System) {

    for (_, process) in sys.processes() {
        let disk_usage = process.disk_usage();

        let prc = json!({
            "pid": process.pid().as_u32(),
            "parent": process.parent().unwrap_or(Pid::from_u32(0)).as_u32(),
            "name": process.name(),
            "exe_path": process.exe(),
            "mem_usage_bytes": process.memory(),
            "virt_mem_usage_bytes": process.virtual_memory(),
            "cpu_usage_percent": process.cpu_usage(),
            "status": format!("{}", process.status()),
            "start_time_seconds": process.start_time(),
            "run_time_seconds": process.run_time(),
            "disk_usage_read_bytes": disk_usage.read_bytes,
            "disk_usage_total_read_bytes": disk_usage.total_read_bytes,
            "disk_usage_written_bytes": disk_usage.written_bytes,
            "disk_usage_total_written_bytes": disk_usage.total_written_bytes,
            "user_id": format!("{:?}", process.user_id().unwrap()),
            "effective_user_id": format!("{:?}", process.effective_user_id().unwrap()),
            "group_id": format!("{:?}", process.group_id().unwrap())
        });

        let payload = json!({
            "sysinfo_data_type": "process_record",
            "data": prc
        });

        println!("{}", payload.to_string());
    }

}

fn generate_user_list() {
    let mut users: Vec<Value> = Vec::new();

    std::thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
    let users_arr: Users = Users::new_with_refreshed_list();
    for user in users_arr.list() {
        users.push(json!({
            "user": user.name(),
            "id": format!("{:?}", user.id()),
        }));
    }

    let payload = json!({
        "sysinfo_data_type": "user_list",
        "data": users
    });

    println!("{}", payload.to_string());
}

fn generate_group_list() {
    let mut groups: Vec<Value> = Vec::new();

    std::thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
    let users_arr: Users = Users::new_with_refreshed_list();
    for user in users_arr.list() {
        for group in user.groups() {
            let group_json = json!({
                "group": group.name(),
                "id": format!("{:?}", group.id()),
            });
            if !groups.contains(&group_json) {
                groups.push(group_json);
            }
        }
    }

    let payload = json!({
        "sysinfo_data_type": "group_list",
        "data": groups
    });

    println!("{}", payload.to_string());
}

#[derive(FromArgs)]
/**
version 0.1.3

Small helper program to get system info and print it to stdout in JSON format.
By default will daemonize itself and print basic system information,
such as OS version, system name, amount of memory, etc.
Use command line switches to fetch additional info.

Additional documentation and examples can be found at 
https://github.com/rustomax/sysinfo-rs
*/
struct Args {
    /// print list of running processes
    #[argh(switch, short = 'p')]
    processes: bool,

    /// print list of users on this system
    #[argh(switch, short = 'u')]
    users: bool,

    /// print list of groups on this system
    #[argh(switch, short = 'g')]
    groups: bool,

    /// do not daemonize; get values once and quit
    #[argh(switch, short = 'q')]
    quit: bool,

    /// how often to fetch updated info, in seconds (default 300s, minimum 15s)
    #[argh(option, short = 'i', default = "300")]
    interval: u64,
}
fn main() { 
	loop {
        let args: Args = argh::from_env();

        let mut sys = System::new_all();
        std::thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
        sys.refresh_all();    

        generate_sysinfo(&sys);

        if args.processes {
            generate_process_list(&sys);
        }
        if args.users {
            generate_user_list();
        }
        if args.groups {
            generate_group_list();
        }
        if args.quit {
            break;
        }

        // Don't refresh data more often than 15 seconds
        thread::sleep(Duration::from_secs(if args.interval < 15 {15} else {args.interval}));
	}
}