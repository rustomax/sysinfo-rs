# sysinfo-rs

Small helper program to get system info and print it to stdout in JSON format. By default will daemonize itself and print basic system information, such as OS version, system name, amount of memory, etc. Use command line switches to fetch additional info. 

Intended to be run via systemd or another wrapper with output being consumed by a forwarding agent, such as [fluentbit](https://fluentbit.io/) or [vector](https://vector.dev/) and an observability platform such as [Observe](https://observeinc.com) to create dashboards, datasets and alerts, i.e:

![Server Home Dashboard](./screenshots/home-dashboard.png)

![Instance Dashboard](./screenshots/instance-dashobard.png)

## Usage

```
Usage: sysinfo-rs [-p] [-u] [-g] [-q] [-i <interval>]

Options:
  -p, --processes   print list of running processes
  -u, --users       print list of users on this system
  -g, --groups      print list of groups on this system
  -q, --quit        do not daemonize; get values once and quit
  -i, --interval    how often to fetch updated info, in seconds (default 300s,
                    minimum 15s)
  --help            display usage information
```

## Collected Info Data Schema Examples

| Schema              | Link                                          |
| ------------------- | --------------------------------------------- |
| System Information  | [Data Schema Example](./schemas/sysinfo.md)   |
| Process List        | [Data Schema Example](./schemas/processes.md) |
| Users               | [Data Schema Example](./schemas/users.md)     |
| Groups              | [Data Schema Example](./schemas/groups.md)    |

## Status

Very early stages of development. Can crash as error-checking is not thorough. Not recommended for production.