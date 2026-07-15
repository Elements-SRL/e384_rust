//! RX data acquisition path: [`RxBuffer`] plus `e384_purgeData`. Unlike [`crate::channel_model`]/
//! [`crate::board_model`]'s borrowed handles, the RX buffer is library-*allocated* memory that
//! this side must free, which `Drop` does.

use std::marker::PhantomData;

use tracing::instrument;

use crate::device::Device;
use crate::error_codes::ErrorCodes;
use crate::sys::{E384Device, E384RxOutput};
use crate::util::translate;

/// Library-allocated RX data buffer, sized via `e384_getRxDataBufferSize` and
/// reused across repeated `e384_getNextMessage` calls. Tied to the `Device`
/// it was allocated from; `Drop` releases it via `e384_deallocateRxDataBuffer`.
#[derive(Debug)]
pub struct RxBuffer<'d> {
    dev: *mut E384Device,
    data: *mut i16,
    len: usize,
    _device: PhantomData<&'d Device>,
}

impl RxBuffer<'_> {
    /// Buffer capacity in samples, as reported by `e384_getRxDataBufferSize`.
    pub fn len(&self) -> usize {
        self.len
    }

    /// True if the buffer has zero capacity.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Wraps `e384_getNextMessage`. The returned slice borrows this buffer and is only valid
    /// until the next call to `next_message`.
    #[instrument(level = "trace")]
    pub fn next_message(&mut self, msg_type: i32) -> Result<(E384RxOutput, &[i16]), ErrorCodes> {
        let mut rx_out = E384RxOutput::default();
        unsafe {
            translate(crate::sys::e384_getNextMessage(
                self.dev,
                &mut rx_out,
                self.data,
                msg_type,
            ))
        }?;
        let n = (rx_out.dataLen as usize).min(self.len);
        let slice = unsafe { std::slice::from_raw_parts(self.data, n) };
        Ok((rx_out, slice))
    }
}

impl Drop for RxBuffer<'_> {
    /// Wraps `e384_deallocateRxDataBuffer`.
    fn drop(&mut self) {
        unsafe { crate::sys::e384_deallocateRxDataBuffer(self.dev, &mut self.data) };
    }
}

impl Device {
    /// Wraps `e384_getRxDataBufferSize` + `e384_allocateRxDataBuffer`.
    #[instrument(level = "trace")]
    pub fn allocate_rx_buffer(&self) -> Result<RxBuffer<'_>, ErrorCodes> {
        let mut size: u32 = 0;
        unsafe { translate(crate::sys::e384_getRxDataBufferSize(self.0, &mut size)) }?;

        let mut data: *mut i16 = std::ptr::null_mut();
        unsafe { translate(crate::sys::e384_allocateRxDataBuffer(self.0, &mut data)) }?;

        Ok(RxBuffer {
            dev: self.0,
            data,
            len: size as usize,
            _device: PhantomData,
        })
    }

    /// Wraps `e384_purgeData`.
    #[instrument(level = "trace")]
    pub fn purge_data(&self) -> Result<(), ErrorCodes> {
        unsafe { translate(crate::sys::e384_purgeData(self.0)) }
    }
}
