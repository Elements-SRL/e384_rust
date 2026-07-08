# Vendored `e384c`

This directory contains a redistributable copy of `e384c` (the Elements SRL C shim/wrapper
library that `e384_rust` binds to via `bindgen`):

```
include/e384c.h   # header bindgen parses
lib/e384c.lib     # import lib, used at link time
bin/e384c.dll     # runtime dll, copied next to build output by build.rs
```

`e384c` itself is safe to redistribute and is checked into this repo so the crate builds
out of the box with no setup.

**Not included:** `e384c.dll`'s own third-party runtime dependencies (e.g. Qt DLLs it was
built against). Their licensing isn't clear enough to vendor here. If `cargo build`/`run`
fails to find them at runtime, either:

- point `E384C_DLL_DIR` at a directory containing them (see root `README.md`), or
- ensure they're already on `PATH`.
