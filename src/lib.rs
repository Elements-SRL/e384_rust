//! Safe Rust wrapper over `e384CommLib`, accessed via the `e384c.h` C shim and bindgen.
//!
//! [`sys`] is the single `include!` of the generated bindings — every other module builds a
//! safe layer on top of it and nothing else touches the raw bindings directly. [`device::Device`]
//! owns the connection; its ~150 wrapped methods are split across several `device_*` modules
//! (channel commands, ranges, acquisition, calibration, compensation, protocols, custom
//! flags/state, value conversion, and device info), one `impl Device` block per file. Every
//! public function name is a rename of an `e384_*` C symbol and carries a
//! `/// Wraps `e384_...`` doc comment linking back to it.
//!
//! Two owned/borrowed handle types encode C-level lifetime rules the compiler can now enforce:
//! [`channel_model::Channel`] and [`board_model::Board`] borrow from the `Device` they came from
//! (the C API has no free function for them — they're invalidated on disconnect), while
//! [`device_acquisition::RxBuffer`] owns a library-allocated buffer it frees on `Drop`.
//!
//! Errors are reported via [`error_codes::ErrorCodes`], which mirrors the C++ library's error
//! group layout; `WarningValueClipped` is treated as success (the command still applied, just
//! clipped) rather than as an `Err`. See the crate README for the full module map and the
//! two-call size/fill protocol used throughout [`util`].

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod board_model;
pub mod channel_model;
pub mod device;
pub mod device_acquisition;
pub mod device_calibration;
pub mod device_channel_cmd;
pub mod device_compensation;
pub mod device_convert;
pub mod device_custom;
pub mod device_info;
pub mod device_protocol;
pub mod device_ranges;
pub mod error_codes;
pub mod sys;
pub mod util;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
