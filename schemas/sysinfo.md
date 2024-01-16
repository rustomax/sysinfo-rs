# Data Schema Example: System Information

> Note: schemas are subject to change.

```json
{
  "data": {
    "cpu": {
      "cpu_brand": "Genuine Intel(R) CPU 0000 @ 2.60GHz",
      "cpu_frequency": 3110,
      "cpu_name": "cpu0",
      "num_cpus": 2
    },
    "disks": [
      {
        "disk_filesystem": "ext4",
        "disk_kind": "HDD",
        "disk_mountpoint": "/",
        "disk_name": "/dev/vda2",
        "disk_total_space": 105086115840
      }
    ],
    "interfaces": [
      {
        "interface_ip": "127.0.0.1",
        "interface_name": "lo"
      },
      {
        "interface_ip": "192.168.1.224",
        "interface_name": "enp1s0"
      },
      {
        "interface_ip": "::1",
        "interface_name": "lo"
      },
      {
        "interface_ip": "2607:fea8:695d:3c00:5054:ff:fed9:7792",
        "interface_name": "enp1s0"
      },
      {
        "interface_ip": "2607:fea8:695d:3c00::8911",
        "interface_name": "enp1s0"
      },
      {
        "interface_ip": "fe80::5054:ff:fed9:7792",
        "interface_name": "enp1s0"
      }
    ],
    "memory": {
      "mem_total": 8323616768,
      "swap_total": 4294963200
    },
    "network": {
      "primary_ip": "192.168.1.224"
    },
    "system": {
      "hostname": "europa",
      "os_kernel_version": "5.15.0-91-generic",
      "os_name": "Ubuntu",
      "os_version": "22.04"
    }
  },
  "sysinfo_data_type": "system_info"
}
```