use std::marker::PhantomData;

use crate::channel_model::{wrap_channels, Channel};
use crate::device::Device;
use crate::error_codes::ErrorCodes;
use crate::sys::{E384BoardModel, E384Measurement};
use crate::util::collect_list;

/// A board handle borrowed from a connected `Device`'s internal model.
/// There is no `e384_boardModel_free` — valid only while the originating
/// `Device` stays connected, enforced here via the lifetime parameter.
pub struct Board<'d> {
    pub(crate) ptr: *mut E384BoardModel,
    pub(crate) _device: PhantomData<&'d Device>,
}

impl<'d> Board<'d> {
    pub fn id(&self) -> u16 {
        unsafe { crate::sys::e384_boardModel_getId(self.ptr) }
    }

    pub fn set_id(&mut self, id: u16) {
        unsafe { crate::sys::e384_boardModel_setId(self.ptr, id) };
    }

    pub fn gate_voltage(&self) -> E384Measurement {
        unsafe { crate::sys::e384_boardModel_getGateVoltage(self.ptr) }
    }

    pub fn set_gate_voltage(&mut self, voltage: E384Measurement) {
        unsafe { crate::sys::e384_boardModel_setGateVoltage(self.ptr, voltage) };
    }

    pub fn source_voltage(&self) -> E384Measurement {
        unsafe { crate::sys::e384_boardModel_getSourceVoltage(self.ptr) }
    }

    pub fn set_source_voltage(&mut self, voltage: E384Measurement) {
        unsafe { crate::sys::e384_boardModel_setSourceVoltage(self.ptr, voltage) };
    }

    pub fn channels_on_board(&self) -> Result<Vec<Channel<'d>>, ErrorCodes> {
        let ptr = self.ptr;
        let ptrs = unsafe {
            collect_list(|out, count| crate::sys::e384_boardModel_getChannelsOnBoard(ptr, out, count))
        }?;
        Ok(wrap_channels(ptrs))
    }
}

impl Device {
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
