# zli

Rust re-implementation of zli previously written in Julia, which in hindsight wasn't the most appropriate choice.
The hardware this tool is deployed to has very limited memory resources and performing i/o operations with Julia incurs many overheads which reduces the performance down to annoyingly slow speeds.

This rewrite should offer substantial performance improvements, especially i/o speeds and memory usage.

## Reverse-engineering OS flash memory

Particle devices have their [*Device OS*](https://docs.particle.io/reference/device-os/firmware/) loaded in flash memory by `dfu-util`.
This tool is described by its author's at Siemens as [^1]:

> the host side implementation of the DFU 1.0 [1] and DFU 1.1 [2] specification of the USB forum [...] intended to download and upload firmware to devices connected over USB.

### Objective

The current `particle-cli` is a little bit bloated for what I'm trying to get done here.
I want to streamline the number of calls needed to be made to: `particle flash <args> <firmware>` in order to speed up the firmware flash procedure.

Not only should this increase the overall performance of the tool, it also means I get to learn about flash memory.

### Observations

Serial connections to the Photon are made via the USB protocol.
Host side, the interface is represented by a mount point at `/sys/class/tty/ttyACM0` which is symlink-ed to `/sys/devices/pci0000/0000:00:14.0/usb1/1-2/1-2:1.0/tty/ttyACM0`.
This directory manages the `uevent`, `power` and `driver` subsystem for handling the interface between this USB device and the system.

When our Photon is connected, we can verify that this is the correct interface location by checking the file named: `interface` and seeing that the interface name is in fact: `Photon Serial`.
We can also obtain the serial interface 

---

[^1]: Source and description available [here](https://github.com/siemens/dfu-util).
