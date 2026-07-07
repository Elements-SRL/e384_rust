use std::ffi::CString;

use crate::device::Device;
use crate::error_codes::ErrorCodes;
use crate::util::{owned_string, translate};

pub struct DeviceVersionInfo {
    pub device_version: u32,
    pub device_sub_version: u32,
    pub fw_major: u32,
    pub fw_minor: u32,
    pub fw_patch: u32,
}

/// Static query, callable before connecting: takes a raw device id string
/// (as returned by `Device::list_devices`) rather than a live handle.
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
    ($name:ident, $sys_fn:path) => {
        pub fn $name(&self) -> Result<bool, ErrorCodes> {
            let mut out: i32 = 0;
            unsafe { translate($sys_fn(self.0, &mut out)) }?;
            Ok(out != 0)
        }
    };
}

/// `(device, outStr: *mut *mut E384String) -> E384Err`
macro_rules! string_getter {
    ($name:ident, $sys_fn:path) => {
        pub fn $name(&self) -> Result<String, ErrorCodes> {
            let mut raw = std::ptr::null_mut();
            unsafe { translate($sys_fn(self.0, &mut raw)) }?;
            Ok(unsafe { owned_string(raw) })
        }
    };
}

impl Device {
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

    pub fn boards_number_features_u16(&self) -> Result<u16, ErrorCodes> {
        let mut out: u16 = 0;
        unsafe { translate(crate::sys::e384_getBoardsNumberFeatures_u16(self.0, &mut out)) }?;
        Ok(out)
    }

    pub fn boards_number_features_int(&self) -> Result<i32, ErrorCodes> {
        let mut out: i32 = 0;
        unsafe { translate(crate::sys::e384_getBoardsNumberFeatures_int(self.0, &mut out)) }?;
        Ok(out)
    }

    string_getter!(device_name, crate::sys::e384_getDeviceName);
    string_getter!(device_serial, crate::sys::e384_getDeviceSerial);
    string_getter!(serial_number, crate::sys::e384_getSerialNumber);
    string_getter!(calib_mapping_file_dir, crate::sys::e384_getCalibMappingFileDir);
    string_getter!(calib_mapping_file_path, crate::sys::e384_getCalibMappingFilePath);

    probe!(has_cal_sw, crate::sys::e384_hasCalSw);
    probe!(has_gate_voltages, crate::sys::e384_hasGateVoltages);
    probe!(has_source_voltages, crate::sys::e384_hasSourceVoltages);
    probe!(is_episodic, crate::sys::e384_isEpisodic);
    probe!(has_proper_header_packets, crate::sys::e384_hasProperHeaderPackets);
    probe!(
        has_independent_vc_current_ranges,
        crate::sys::e384_hasIndependentVCCurrentRanges
    );
    probe!(
        has_independent_cc_voltage_ranges,
        crate::sys::e384_hasIndependentCCVoltageRanges
    );
    probe!(has_channel_switches, crate::sys::e384_hasChannelSwitches);
    probe!(has_stimulus_switches, crate::sys::e384_hasStimulusSwitches);
    probe!(has_offset_compensation, crate::sys::e384_hasOffsetCompensation);
    probe!(has_stimulus_half, crate::sys::e384_hasStimulusHalf);
    probe!(has_protocols, crate::sys::e384_hasProtocols);
    probe!(has_protocol_step_feature, crate::sys::e384_hasProtocolStepFeature);
    probe!(has_protocol_ramp_feature, crate::sys::e384_hasProtocolRampFeature);
    probe!(has_protocol_sin_feature, crate::sys::e384_hasProtocolSinFeature);
    probe!(is_state_array_available, crate::sys::e384_isStateArrayAvailable);
    probe!(calibration_status, crate::sys::e384_getCalibrationStatus);
}
