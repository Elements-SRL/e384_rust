//! Device/channel/board counts, string getters (device name/serial, calib paths), and capability
//! probes. Also [`device_info_for_id`], the one function in the whole API that takes a raw
//! device id string instead of a connected `Device` handle — it's a static query, callable
//! before connecting.

use std::ffi::CString;

use tracing::instrument;

use crate::device::Device;
use crate::error_codes::ErrorCodes;
use crate::util::{owned_string, translate};

/// Device version/firmware info returned by [`device_info_for_id`] and [`Device::device_info`].
pub struct DeviceVersionInfo {
    pub device_version: u32,
    pub device_sub_version: u32,
    pub fw_major: u32,
    pub fw_minor: u32,
    pub fw_patch: u32,
}

/// Wraps `e384_getDeviceInfoForId`. Static query, callable before connecting: takes a raw device
/// id string (as returned by `Device::list_devices`) rather than a live handle.
#[instrument(level = "trace")]
pub fn device_info_for_id(device_id: &str) -> Result<DeviceVersionInfo, ErrorCodes> {
    let Ok(c_id) = CString::new(device_id) else {
        return Err(ErrorCodes::ErrorDeviceNotFound);
    };
    let mut info = DeviceVersionInfo {
        device_version: 0,
        device_sub_version: 0,
        fw_major: 0,
        fw_minor: 0,
        fw_patch: 0,
    };
    unsafe {
        translate(crate::sys::e384_getDeviceInfoForId(
            c_id.as_ptr(),
            &mut info.device_version,
            &mut info.device_sub_version,
            &mut info.fw_major,
            &mut info.fw_minor,
            &mut info.fw_patch,
        ))
    }?;
    Ok(info)
}

/// `(device, outResult: *mut i32) -> E384Err` capability probe.
macro_rules! probe {
    ($doc:literal, $name:ident, $sys_fn:path) => {
        #[doc = $doc]
        #[instrument(level = "trace")]
        pub fn $name(&self) -> Result<bool, ErrorCodes> {
            let mut out: i32 = 0;
            unsafe { translate($sys_fn(self.0, &mut out)) }?;
            Ok(out != 0)
        }
    };
}

/// `(device, outStr: *mut *mut E384String) -> E384Err`
macro_rules! string_getter {
    ($doc:literal, $name:ident, $sys_fn:path) => {
        #[doc = $doc]
        #[instrument(level = "trace")]
        pub fn $name(&self) -> Result<String, ErrorCodes> {
            let mut raw = std::ptr::null_mut();
            unsafe { translate($sys_fn(self.0, &mut raw)) }?;
            Ok(unsafe { owned_string(raw) })
        }
    };
}

impl Device {
    /// Wraps `e384_getDeviceInfo`.
    #[instrument(level = "trace")]
    pub fn device_info(&self) -> Result<DeviceVersionInfo, ErrorCodes> {
        let mut info = DeviceVersionInfo {
            device_version: 0,
            device_sub_version: 0,
            fw_major: 0,
            fw_minor: 0,
            fw_patch: 0,
        };
        unsafe {
            translate(crate::sys::e384_getDeviceInfo(
                self.0,
                &mut info.device_version,
                &mut info.device_sub_version,
                &mut info.fw_major,
                &mut info.fw_minor,
                &mut info.fw_patch,
            ))
        }?;
        Ok(info)
    }

    /// Wraps `e384_getChannelNumberFeatures_u16`.
    #[instrument(level = "trace")]
    pub fn channel_number_features_u16(&self) -> Result<(u16, u16), ErrorCodes> {
        let mut voltage: u16 = 0;
        let mut current: u16 = 0;
        unsafe {
            translate(crate::sys::e384_getChannelNumberFeatures_u16(
                self.0,
                &mut voltage,
                &mut current,
            ))
        }?;
        Ok((voltage, current))
    }

    /// Wraps `e384_getChannelNumberFeatures_int`.
    #[instrument(level = "trace")]
    pub fn channel_number_features_int(&self) -> Result<(i32, i32), ErrorCodes> {
        let mut voltage: i32 = 0;
        let mut current: i32 = 0;
        unsafe {
            translate(crate::sys::e384_getChannelNumberFeatures_int(
                self.0,
                &mut voltage,
                &mut current,
            ))
        }?;
        Ok((voltage, current))
    }

    /// Wraps `e384_getChannelNumberFeatures_intGp`.
    #[instrument(level = "trace")]
    pub fn channel_number_features_int_gp(&self) -> Result<(i32, i32, i32), ErrorCodes> {
        let mut voltage: i32 = 0;
        let mut current: i32 = 0;
        let mut gp: i32 = 0;
        unsafe {
            translate(crate::sys::e384_getChannelNumberFeatures_intGp(
                self.0,
                &mut voltage,
                &mut current,
                &mut gp,
            ))
        }?;
        Ok((voltage, current, gp))
    }

    /// Wraps `e384_getBoardsNumberFeatures_u16`.
    #[instrument(level = "trace")]
    pub fn boards_number_features_u16(&self) -> Result<u16, ErrorCodes> {
        let mut out: u16 = 0;
        unsafe {
            translate(crate::sys::e384_getBoardsNumberFeatures_u16(
                self.0, &mut out,
            ))
        }?;
        Ok(out)
    }

    /// Wraps `e384_getBoardsNumberFeatures_int`.
    #[instrument(level = "trace")]
    pub fn boards_number_features_int(&self) -> Result<i32, ErrorCodes> {
        let mut out: i32 = 0;
        unsafe {
            translate(crate::sys::e384_getBoardsNumberFeatures_int(
                self.0, &mut out,
            ))
        }?;
        Ok(out)
    }

    string_getter!(
        "Wraps `e384_getDeviceName`.",
        device_name,
        crate::sys::e384_getDeviceName
    );
    string_getter!(
        "Wraps `e384_getDeviceSerial`.",
        device_serial,
        crate::sys::e384_getDeviceSerial
    );
    string_getter!(
        "Wraps `e384_getSerialNumber`.",
        serial_number,
        crate::sys::e384_getSerialNumber
    );
    string_getter!(
        "Wraps `e384_getCalibMappingFileDir`.",
        calib_mapping_file_dir,
        crate::sys::e384_getCalibMappingFileDir
    );
    string_getter!(
        "Wraps `e384_getCalibMappingFilePath`.",
        calib_mapping_file_path,
        crate::sys::e384_getCalibMappingFilePath
    );

    probe!(
        "Wraps `e384_hasCalSw`.",
        has_cal_sw,
        crate::sys::e384_hasCalSw
    );
    probe!(
        "Wraps `e384_hasGateVoltages`.",
        has_gate_voltages,
        crate::sys::e384_hasGateVoltages
    );
    probe!(
        "Wraps `e384_hasSourceVoltages`.",
        has_source_voltages,
        crate::sys::e384_hasSourceVoltages
    );
    probe!(
        "Wraps `e384_isEpisodic`.",
        is_episodic,
        crate::sys::e384_isEpisodic
    );
    probe!(
        "Wraps `e384_hasProperHeaderPackets`.",
        has_proper_header_packets,
        crate::sys::e384_hasProperHeaderPackets
    );
    probe!(
        "Wraps `e384_hasIndependentVCCurrentRanges`.",
        has_independent_vc_current_ranges,
        crate::sys::e384_hasIndependentVCCurrentRanges
    );
    probe!(
        "Wraps `e384_hasIndependentCCVoltageRanges`.",
        has_independent_cc_voltage_ranges,
        crate::sys::e384_hasIndependentCCVoltageRanges
    );
    probe!(
        "Wraps `e384_hasChannelSwitches`.",
        has_channel_switches,
        crate::sys::e384_hasChannelSwitches
    );
    probe!(
        "Wraps `e384_hasStimulusSwitches`.",
        has_stimulus_switches,
        crate::sys::e384_hasStimulusSwitches
    );
    probe!(
        "Wraps `e384_hasOffsetCompensation`.",
        has_offset_compensation,
        crate::sys::e384_hasOffsetCompensation
    );
    probe!(
        "Wraps `e384_hasStimulusHalf`.",
        has_stimulus_half,
        crate::sys::e384_hasStimulusHalf
    );
    probe!(
        "Wraps `e384_hasProtocols`.",
        has_protocols,
        crate::sys::e384_hasProtocols
    );
    probe!(
        "Wraps `e384_hasProtocolStepFeature`.",
        has_protocol_step_feature,
        crate::sys::e384_hasProtocolStepFeature
    );
    probe!(
        "Wraps `e384_hasProtocolRampFeature`.",
        has_protocol_ramp_feature,
        crate::sys::e384_hasProtocolRampFeature
    );
    probe!(
        "Wraps `e384_hasProtocolSinFeature`.",
        has_protocol_sin_feature,
        crate::sys::e384_hasProtocolSinFeature
    );
    probe!(
        "Wraps `e384_isStateArrayAvailable`.",
        is_state_array_available,
        crate::sys::e384_isStateArrayAvailable
    );
    probe!(
        "Wraps `e384_getCalibrationStatus`.",
        calibration_status,
        crate::sys::e384_getCalibrationStatus
    );
}
