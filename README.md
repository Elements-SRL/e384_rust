# e384_rust

A safe Rust wrapper over `e384CommLib`, the control library for e384 amplifier hardware, accessed via the
`e384c.h` C shim and [`bindgen`](https://docs.rs/bindgen).

## Prerequisites / build

Nothing is vendored in this repo. `e384c` is a **static** library, and `e384c.lib` leaves
`e384commlib` and its transitive vendor SDKs (FTDI, Opal Kelly `okFrontPanel`, the FTDI MPSSE lib)
**unresolved** — every header/lib directory below must be supplied via environment variable before
`cargo build`/`check`/`test` will succeed; `build.rs` panics with an actionable message naming
whichever one is missing first.

| Env var | Points to |
|---|---|
| `E384C_INCLUDE_DIR` | directory containing `e384c.h` |
| `E384C_LIB_DIR` | directory containing `e384c.lib` |
| `E384COMMLIB_LIB_DIR` | directory containing `e384commlib.lib` |
| `FTD2XX_LIB_DIR` | directory containing `ftd2xx.lib` |
| `OKFRONTPANEL_LIB_DIR` | directory containing `okFrontPanel.lib` |
| `MPSSE_LIB_DIR` | directory containing `MPSSE.lib` |

None of these vars have a fallback or default — every directory must be set explicitly
(`cargo check` alone doesn't require the libs to actually be present at link time to pass, only the
directories to exist).

The three vendor **runtime DLLs** (`FTD2XX.dll`, `MPSSE.dll`, `okFrontPanel.dll`) are not this
crate's concern — it does not copy any DLLs next to the build output. The consumer binary (and its
installer) is responsible for placing those DLLs next to the final executable. `e384commlib` is
itself static, so it has no runtime DLL.

## Architecture overview

`src/sys.rs` is the single `include!` of the generated `bindings.rs` (raw `extern "C"` bindgen output) —
every other module builds on top of it and nothing else touches `bindings.rs` directly. The rest of the
crate is a safe layer over ~150 C functions, organized as one `Device` type with its methods split across
several files (Rust allows an inherent `impl` block to be split across modules, so this isn't one giant
file):

| Module | Wraps |
|---|---|
| `sys` | raw bindgen output |
| `error_codes` | `ErrorCodes` enum + `E384Err` → `ErrorCodes` mapping |
| `util` | `translate`, two-call helpers, string/list extraction, channel-command shape helpers |
| `device` | `Device` (connect/disconnect/discovery) |
| `device_ranges`, `device_channel_cmd` | VC/CC ranges & filters, shape A/B/C channel commands |
| `device_acquisition` | `RxBuffer<'d>`, RX data path |
| `device_calibration` | EEPROM read/write, OK RAM, calib file names/flags |
| `device_compensation` | compensation features/values/ranges/options, clamping modality |
| `device_protocol` | voltage/current protocol builders |
| `device_custom` | custom flags/options/doubles, debug bit/word, state array, zap, misc scalar commands |
| `device_convert` | raw ADC ↔ physical value conversion |
| `device_info` | device/channel/board counts, string getters, capability probes |
| `channel_model`, `board_model` | `Channel<'d>`, `Board<'d>` borrowed model handles |

Every public Rust function name is a rename of an `e384_*` C symbol (e.g. `set_voltage_hold_tuner` wraps
`e384_setVoltageHoldTuner`) — each one carries a `/// Wraps \`e384_...\`` doc comment so you can cross-
reference back to `e384c.h` or the vendor's C++ docs without grepping the generated bindings.

Two functions from the header are deliberately **not** wrapped, per the header's own comments:
`e384_getCustomOptions` (a ragged `vector<vector<string>>`, never exposed cleanly at the C boundary) and
`BoardModel::setChannelsOnBoard` (would need caller-constructed owned `ChannelModel*` handles).

## Error handling

`ErrorCodes` (`src/error_codes.rs`) mirrors `e384commlib_errorcodes.h` group-by-group (device detection,
eeprom communication, device connection, device commands, device features, device issues, library issues,
calibration). `WarningValueClipped` is the one code that is *not* a failure — the command was still
applied, just clipped to range — so `ErrorCodes::to_res` treats it as `Ok(())` and logs via
`tracing::warn!` instead of returning `Err`.

## Ownership / lifetime model

Three types encode C-level ownership rules that the compiler can now enforce:

- **`Channel<'d>` / `Board<'d>`** (`channel_model.rs` / `board_model.rs`): there is no
  `e384_channelModel_free`/`e384_boardModel_free` in the C API. These pointers are borrowed views into the
  connected device's internal model and are invalidated on disconnect. The `'d` lifetime ties them to the
  `&Device` they were obtained from, so the borrow checker rejects using one after its `Device` is dropped.
- **`RxBuffer<'d>`** (`device_acquisition.rs`): a library-*allocated* buffer (`e384_allocateRxDataBuffer`),
  unlike the borrowed model handles. It owns its memory and frees it in `Drop` via
  `e384_deallocateRxDataBuffer`; the `'d` lifetime just prevents using it past the owning `Device`'s life.

## Two-call size/fill protocol

Much of the C API follows the same idiom for variable-length outputs: call once with a null buffer to get
an element count, allocate a `Vec` of that size, then call again to fill it. `util.rs` factors this into
`collect_list`, `collect_matrix` (row/col variant), and `collect_list_with_default_idx` (adds a
default-index out-param, used by range getters). Call sites just supply a closure that fixes the device
handle and any other fixed parameters — see any `device_ranges.rs` getter for the pattern.

## Testing

`device.rs` has `list_demo_devices`, which depends on a demo `.pls` file existing in the user's home
directory (`e384_DEMO.pls`, alongside `e384_RX_RAW.pls`/`e384_TX.pls`/`EMCR_debug.pls`) — the test's
`FileManager` creates it if missing and restores prior file state afterward. There's no real hardware in
CI/dev environments, so new tests for wrapped functions should prefer exercising the demo device path the
same way, rather than requiring a physically connected device.
