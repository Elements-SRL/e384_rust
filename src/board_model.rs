//! [`Board`] — a borrowed handle over `E384BoardModel`, mirroring [`crate::channel_model`]:
//! there is no `e384_boardModel_free`, so getters/setters are plain field accessors and the
//! handle's lifetime is tied to the owning `Device`.
//!
//! `BoardModel::setChannelsOnBoard` is deliberately not wrapped — the C API comment notes it
//! would need caller-constructed owned `ChannelModel*` handles, which the borrowed-handle model
//! here doesn't support; that wiring is internal-only, done at connect time.

use std::marker::PhantomData;

use tracing::instrument;

use crate::channel_model::{Channel, wrap_channels};
use crate::device::Device;
use crate::error_codes::ErrorCodes;
use crate::sys::{E384BoardModel, E384Measurement};
use crate::util::collect_list;

/// A board handle borrowed from a connected `Device`'s internal model.
/// There is no `e384_boardModel_free` — valid only while the originating
/// `Device` stays connected, enforced here via the lifetime parameter.
#[derive(Debug)]
pub struct Board<'d> {
    pub(crate) ptr: *mut E384BoardModel,
    pub(crate) _device: PhantomData<&'d Device>,
}

impl<'d> Board<'d> {
    /// Wraps `e384_boardModel_getId`.
    #[instrument(level = "trace")]
    pub fn id(&self) -> u16 {
        unsafe { crate::sys::e384_boardModel_getId(self.ptr) }
    }

    /// Wraps `e384_boardModel_setId`.
    #[instrument(level = "trace")]
    pub fn set_id(&mut self, id: u16) {
        unsafe { crate::sys::e384_boardModel_setId(self.ptr, id) };
    }

    /// Wraps `e384_boardModel_getGateVoltage`.
    #[instrument(level = "trace")]
    pub fn gate_voltage(&self) -> E384Measurement {
        unsafe { crate::sys::e384_boardModel_getGateVoltage(self.ptr) }
    }

    /// Wraps `e384_boardModel_setGateVoltage`.
    #[instrument(level = "trace")]
    pub fn set_gate_voltage(&mut self, voltage: E384Measurement) {
        unsafe { crate::sys::e384_boardModel_setGateVoltage(self.ptr, voltage) };
    }

    /// Wraps `e384_boardModel_getSourceVoltage`.
    #[instrument(level = "trace")]
    pub fn source_voltage(&self) -> E384Measurement {
        unsafe { crate::sys::e384_boardModel_getSourceVoltage(self.ptr) }
    }

    /// Wraps `e384_boardModel_setSourceVoltage`.
    #[instrument(level = "trace")]
    pub fn set_source_voltage(&mut self, voltage: E384Measurement) {
        unsafe { crate::sys::e384_boardModel_setSourceVoltage(self.ptr, voltage) };
    }

    /// Wraps `e384_boardModel_getChannelsOnBoard`.
    #[instrument(level = "trace")]
    pub fn channels_on_board(&self) -> Result<Vec<Channel<'d>>, ErrorCodes> {
        let ptr = self.ptr;
        let ptrs = unsafe {
            collect_list(|out, count| {
                crate::sys::e384_boardModel_getChannelsOnBoard(ptr, out, count)
            })
        }?;
        Ok(wrap_channels(ptrs))
    }
}

impl Device {
    /// Wraps `e384_getBoards`.
    #[instrument(level = "trace")]
    pub fn boards(&self) -> Result<Vec<Board<'_>>, ErrorCodes> {
        let dev = self.0;
        let ptrs =
            unsafe { collect_list(|out, count| crate::sys::e384_getBoards(dev, out, count)) }?;
        Ok(ptrs
            .into_iter()
            .map(|ptr| Board {
                ptr,
                _device: PhantomData,
            })
            .collect())
    }
}
