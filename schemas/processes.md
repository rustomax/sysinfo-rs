# Data Schema Example: Process List

> Note: schemas are subject to change.

For each process running on a system, an individual record will be printed, i.e.:

```json
{
  "data": {
    "cpu_usage_percent": 0,
    "disk_usage_read_bytes": 0,
    "disk_usage_total_read_bytes": 0,
    "disk_usage_total_written_bytes": 0,
    "disk_usage_written_bytes": 0,
    "effective_user_id": "Uid(0)",
    "exe_path": null,
    "group_id": "Gid(0)",
    "mem_usage_bytes": 0,
    "name": "raid5wq",
    "parent": 2,
    "pid": 270,
    "run_time_seconds": 2596015,
    "start_time_seconds": 1702782607,
    "status": "Idle",
    "user_id": "Uid(0)",
    "virt_mem_usage_bytes": 0
  },
  "sysinfo_data_type": "process_record"
}
```
