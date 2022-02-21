# zli

Rust re-implementation of zli previously written in Julia, which in hindsight wasn't the most appropriate choice.
The hardware this tool is deployed to has very limited memory resources and performing i/o operations with Julia incurs many overheads which reduces the performance down to annoyingly slow speeds.

This rewrite should offer substantial performance improvements, especially i/o speeds and memory usage.

## Reverse-engineering how `dfu-util` is used by `particle-cli`

Particle devices have their [*Device OS*](https://docs.particle.io/reference/device-os/firmware/) loaded in flash memory by `dfu-util`.
This tool,`dfu-util`, is described by its author's at Siemens as [^1]:

> the host side implementation of the DFU 1.0 [1] and DFU 1.1 [2] specification of the USB forum [...] intended to download and upload firmware to devices connected over USB.

[^1]: Source and description available [here](https://github.com/siemens/dfu-util).
