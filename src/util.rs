//! Shared building blocks reused by every `device_*` module: raw `E384Err` → `Result` translation,
//! the two-call size/fill list protocol (`collect_list`/`collect_matrix`/
//! `collect_list_with_default_idx`), owned string/list extraction, and the shape A/B/C channel
//! command helpers (`channel_cmd_*`).

use std::ffi::CStr;

use tracing::instrument;

use crate::{
    error_codes::ErrorCodes,
    sys::{E384DeviceList, E384Err, E384Measurement, E384String},
};

/// Converts a raw `E384Err` into a `Result`, via [`ErrorCodes::from`] + [`ErrorCodes::to_res`].
#[instrument]
pub fn translate(err: E384Err) -> Result<(), ErrorCodes> {
    let e: ErrorCodes = err.into();
    e.to_res()
}

/// Two-call size/fill list protocol: call once with a null buffer to get the
/// element count, allocate, then call again to fill it.
pub(crate) unsafe fn collect_list<T: Default + Clone>(
    mut call: impl FnMut(*mut T, *mut usize) -> E384Err,
) -> Result<Vec<T>, ErrorCodes> {
    let mut count: usize = 0;
    translate(call(std::ptr::null_mut(), &mut count))?;
    let mut buf = vec![T::default(); count];
    if count > 0 {
        translate(call(buf.as_mut_ptr(), &mut count))?;
        buf.truncate(count);
    }
    Ok(buf)
}

/// Two-call size/fill protocol for row-major matrices sized by rows*cols.
pub(crate) unsafe fn collect_matrix<T: Default + Clone>(
    mut call: impl FnMut(*mut T, *mut usize, *mut usize) -> E384Err,
) -> Result<(Vec<T>, usize, usize), ErrorCodes> {
    let mut rows: usize = 0;
    let mut cols: usize = 0;
    translate(call(std::ptr::null_mut(), &mut rows, &mut cols))?;
    let mut buf = vec![T::default(); rows * cols];
    if rows * cols > 0 {
        translate(call(buf.as_mut_ptr(), &mut rows, &mut cols))?;
        buf.truncate(rows * cols);
    }
    Ok((buf, rows, cols))
}

/// Reads an owned `E384String*`, copying its contents out, then frees it.
pub(crate) unsafe fn owned_string(raw: *mut E384String) -> String {
    let s = unsafe { CStr::from_ptr(crate::sys::e384_string_get(raw)) }
        .to_string_lossy()
        .into_owned();
    unsafe { crate::sys::e384_string_free(raw) };
    s
}

/// Two-call size/fill list protocol that also carries a default-index
/// out-param, as used by e.g. `e384_getVCCurrentRanges`.
pub(crate) unsafe fn collect_list_with_default_idx<T: Default + Clone>(
    mut call: impl FnMut(*mut T, *mut usize, *mut u16) -> E384Err,
) -> Result<(Vec<T>, u16), ErrorCodes> {
    let mut count: usize = 0;
    let mut default_idx: u16 = 0;
    translate(call(std::ptr::null_mut(), &mut count, &mut default_idx))?;
    let mut buf = vec![T::default(); count];
    if count > 0 {
        translate(call(buf.as_mut_ptr(), &mut count, &mut default_idx))?;
        buf.truncate(count);
    }
    Ok((buf, default_idx))
}

/// Shape A channel command: `(device, channelIndexes, values, count, applyFlag)`.
pub(crate) unsafe fn channel_cmd_measurement(
    call: impl FnOnce(*const u16, *const E384Measurement, usize, i32) -> E384Err,
    channels: &[u16],
    values: &[E384Measurement],
    apply: bool,
) -> Result<(), ErrorCodes> {
    assert_eq!(channels.len(), values.len());
    translate(call(
        channels.as_ptr(),
        values.as_ptr(),
        channels.len(),
        apply as i32,
    ))
}

/// Shape B channel command: `(device, channelIndexes, count, applyFlag)`.
pub(crate) unsafe fn channel_cmd_update(
    call: impl FnOnce(*const u16, usize, i32) -> E384Err,
    channels: &[u16],
    apply: bool,
) -> Result<(), ErrorCodes> {
    translate(call(channels.as_ptr(), channels.len(), apply as i32))
}

/// Shape C channel command: `(device, channelIndexes, onValues, count, applyFlag)`.
pub(crate) unsafe fn channel_cmd_bool(
    call: impl FnOnce(*const u16, *const u8, usize, i32) -> E384Err,
    channels: &[u16],
    on: &[bool],
    apply: bool,
) -> Result<(), ErrorCodes> {
    assert_eq!(channels.len(), on.len());
    let on_u8: Vec<u8> = on.iter().map(|&b| b as u8).collect();
    translate(call(
        channels.as_ptr(),
        on_u8.as_ptr(),
        channels.len(),
        apply as i32,
    ))
}

/// Reads an owned `E384DeviceList*` into a `Vec<String>`, then frees it.
/// Used both for device discovery and every other function that reuses this
/// handle type as a generic `vector<string>` getter.
pub(crate) unsafe fn owned_string_list(list: *mut E384DeviceList) -> Vec<String> {
    let count = unsafe { crate::sys::e384_deviceList_count(list) };
    let mut out: Vec<String> = Vec::with_capacity(count);
    for i in 0..count {
        let ptr = unsafe { crate::sys::e384_deviceList_get(list, i) };
        if ptr.is_null() {
            continue;
        }
        // Copy the string out now — it's only valid until deviceList_free.
        out.push(
            unsafe { CStr::from_ptr(ptr) }
                .to_string_lossy()
                .into_owned(),
        );
    }
    unsafe { crate::sys::e384_deviceList_free(list) };
    out
}
