use std::marker::PhantomData;

use crate::device::Device;
use crate::error_codes::ErrorCodes;
use crate::sys::{E384ChannelModel, E384Measurement};
use crate::util::collect_list;

/// A channel handle borrowed from a connected `Device`'s internal model.
/// There is no `e384_channelModel_free` — the pointer is only valid for as
/// long as the `Device` it came from stays connected, which the lifetime
/// parameter enforces at compile time.
pub struct Channel<'d> {
    pub(crate) ptr: *mut E384ChannelModel,
    pub(crate) _device: PhantomData<&'d Device>,
}

impl Channel<'_> {
    pub fn id(&self) -> u16 {
        unsafe { crate::sys::e384_channelModel_getId(self.ptr) }
    }

    pub fn set_id(&mut self, id: u16) {
        unsafe { crate::sys::e384_channelModel_setId(self.ptr, id) };
    }

    pub fn is_on(&self) -> bool {
        unsafe { crate::sys::e384_channelModel_isOn(self.ptr) != 0 }
    }

    pub fn set_on(&mut self, on: bool) {
        unsafe { crate::sys::e384_channelModel_setOn(self.ptr, on as i32) };
    }

    pub fn is_recalibrating_readout_offset(&self) -> bool {
        unsafe { crate::sys::e384_channelModel_isRecalibratingReadoutOffset(self.ptr) != 0 }
    }

    pub fn set_recalibrating_readout_offset(&mut self, recalibrating: bool) {
        unsafe {
            crate::sys::e384_channelModel_setRecalibratingReadoutOffset(
                self.ptr,
                recalibrating as i32,
            )
        };
    }

    pub fn is_compensating_liquid_junction(&self) -> bool {
        unsafe { crate::sys::e384_channelModel_isCompensatingLiquidJunction(self.ptr) != 0 }
    }

    pub fn set_compensating_liquid_junction(&mut self, compensating: bool) {
        unsafe {
            crate::sys::e384_channelModel_setCompensatingLiquidJunction(
                self.ptr,
                compensating as i32,
            )
        };
    }

    pub fn is_compensating_cfast(&self) -> bool {
        unsafe { crate::sys::e384_channelModel_isCompensatingCfast(self.ptr) != 0 }
    }

    pub fn set_compensating_cfast(&mut self, compensating: bool) {
        unsafe { crate::sys::e384_channelModel_setCompensatingCfast(self.ptr, compensating as i32) };
    }

    pub fn is_compensating_cslow_rs(&self) -> bool {
        unsafe { crate::sys::e384_channelModel_isCompensatingCslowRs(self.ptr) != 0 }
    }

    pub fn set_compensating_cslow_rs(&mut self, compensating: bool) {
        unsafe {
            crate::sys::e384_channelModel_setCompensatingCslowRs(self.ptr, compensating as i32)
        };
    }

    pub fn is_compensating_rs_cp(&self) -> bool {
        unsafe { crate::sys::e384_channelModel_isCompensatingRsCp(self.ptr) != 0 }
    }

    pub fn set_compensating_rs_cp(&mut self, compensating: bool) {
        unsafe { crate::sys::e384_channelModel_setCompensatingRsCp(self.ptr, compensating as i32) };
    }

    pub fn is_compensating_rs_pg(&self) -> bool {
        unsafe { crate::sys::e384_channelModel_isCompensatingRsPg(self.ptr) != 0 }
    }

    pub fn set_compensating_rs_pg(&mut self, compensating: bool) {
        unsafe { crate::sys::e384_channelModel_setCompensatingRsPg(self.ptr, compensating as i32) };
    }

    /// No matching `is_compensating_cc_cfast` getter exists in the C API.
    pub fn set_compensating_cc_cfast(&mut self, compensating: bool) {
        unsafe {
            crate::sys::e384_channelModel_setCompensatingCcCfast(self.ptr, compensating as i32)
        };
    }

    pub fn is_stim_active(&self) -> bool {
        unsafe { crate::sys::e384_channelModel_isStimActive(self.ptr) != 0 }
    }

    pub fn set_stim_active(&mut self, active: bool) {
        unsafe { crate::sys::e384_channelModel_setStimActive(self.ptr, active as i32) };
    }

    pub fn vhold(&self) -> E384Measurement {
        unsafe { crate::sys::e384_channelModel_getVhold(self.ptr) }
    }

    pub fn set_vhold(&mut self, vhold: E384Measurement) {
        unsafe { crate::sys::e384_channelModel_setVhold(self.ptr, vhold) };
    }

    pub fn chold(&self) -> E384Measurement {
        unsafe { crate::sys::e384_channelModel_getChold(self.ptr) }
    }

    pub fn set_chold(&mut self, chold: E384Measurement) {
        unsafe { crate::sys::e384_channelModel_setChold(self.ptr, chold) };
    }

    pub fn vhalf(&self) -> E384Measurement {
        unsafe { crate::sys::e384_channelModel_getVhalf(self.ptr) }
    }

    pub fn set_vhalf(&mut self, vhalf: E384Measurement) {
        unsafe { crate::sys::e384_channelModel_setVhalf(self.ptr, vhalf) };
    }

    pub fn chalf(&self) -> E384Measurement {
        unsafe { crate::sys::e384_channelModel_getChalf(self.ptr) }
    }

    pub fn set_chalf(&mut self, chalf: E384Measurement) {
        unsafe { crate::sys::e384_channelModel_setChalf(self.ptr, chalf) };
    }

    pub fn liquid_junction_voltage(&self) -> E384Measurement {
        unsafe { crate::sys::e384_channelModel_getLiquidJunctionVoltage(self.ptr) }
    }

    pub fn set_liquid_junction_voltage(&mut self, voltage: E384Measurement) {
        unsafe { crate::sys::e384_channelModel_setLiquidJunctionVoltage(self.ptr, voltage) };
    }
}

pub(crate) fn wrap_channels(ptrs: Vec<*mut E384ChannelModel>) -> Vec<Channel<'static>> {
    ptrs.into_iter()
        .map(|ptr| Channel {
            ptr,
            _device: PhantomData,
        })
        .collect()
}

impl Device {
    pub fn channels(&self) -> Result<Vec<Channel<'_>>, ErrorCodes> {
        let dev = self.0;
        let ptrs =
            unsafe { collect_list(|out, count| crate::sys::e384_getChannels(dev, out, count)) }?;
        Ok(wrap_channels(ptrs))
    }

    pub fn channels_on_board(&self, board_idx: u16) -> Result<Vec<Channel<'_>>, ErrorCodes> {
        let dev = self.0;
        let ptrs = unsafe {
            collect_list(|out, count| {
                crate::sys::e384_getChannelsOnBoard(dev, board_idx, out, count)
            })
        }?;
        Ok(wrap_channels(ptrs))
    }

    pub fn channels_on_row(&self, row_idx: u16) -> Result<Vec<Channel<'_>>, ErrorCodes> {
        let dev = self.0;
        let ptrs = unsafe {
            collect_list(|out, count| crate::sys::e384_getChannelsOnRow(dev, row_idx, out, count))
        }?;
        Ok(wrap_channels(ptrs))
    }
}
