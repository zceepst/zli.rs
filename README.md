# zli

Rust re-implementation of zli [previously implemented](~/Documents/work/zioxi-tools/zli) in Julia.
This first attempt was bit slow due do overheads inherent to the JIT runtime.

The hardware we are deploying to here is __incredibly__ slow -- a Raspberry Pi would be an improvement (this is not a joke).
So, how do we fix this?

Well, having been bombarded with post after post about the *blazing fast* performance of Rust on the orange site over and over again, I've decided to give it a go.

This will most likely all be poorly optimized code.
