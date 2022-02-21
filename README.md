# zli

Rust re-implantation of zli previously written in Julia, which in hindsight wasn't the most appropriate choice.
The hardware this tool is deployed to has very limited memory resources and performing i/o operations with Julia inccurs many overheads which reduces the performance down to annoyingly slow speeds.

This rewrite should offer substantial performance improvements, especially i/o speeds and memory usage.

