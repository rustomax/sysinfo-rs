# sysinfo-rs

Small helper program that daemonizes itself and prints out information about the host, such as hostname, ip addresses, memory size, etc at regular intervals in JSON format.

By design doesn't have configurability.

Intended to be run via systemd or another wrapper reading results from stdout.


## Status:

Very early stages of development. No real error checking yet. Not recommended for production.