use crate::device::Device;
use crate::error_codes::ErrorCodes;
use crate::sys::E384Measurement;
use crate::util::{channel_cmd_bool, channel_cmd_measurement, channel_cmd_update};

/// Shape A: `(device, channelIndexes, values, count, applyFlag)`.
macro_rules! measurement_cmd {
    ($name:ident, $sys_fn:path) => {
        pub fn $name(
            &self,
            channels: &[u16],
            values: &[E384Measurement],
            apply: bool,
        ) -> Result<(), ErrorCodes> {
            let dev = self.0;
            unsafe {
                channel_cmd_measurement(
                    |c, v, n, a| $sys_fn(dev, c, v, n, a),
                    channels,
                    values,
                    apply,
                )
            }
        }
    };
}

/// Shape B: `(device, channelIndexes, count, applyFlag)`.
macro_rules! update_cmd {
    ($name:ident, $sys_fn:path) => {
        pub fn $name(&self, channels: &[u16], apply: bool) -> Result<(), ErrorCodes> {
            let dev = self.0;
            unsafe { channel_cmd_update(|c, n, a| $sys_fn(dev, c, n, a), channels, apply) }
        }
    };
}

/// Shape C: `(device, channelIndexes, onValues, count, applyFlag)`.
macro_rules! bool_cmd {
    ($name:ident, $sys_fn:path) => {
        pub fn $name(&self, channels: &[u16], on: &[bool], apply: bool) -> Result<(), ErrorCodes> {
            let dev = self.0;
            unsafe { channel_cmd_bool(|c, o, n, a| $sys_fn(dev, c, o, n, a), channels, on, apply) }
        }
    };
}

impl Device {
    measurement_cmd!(set_voltage_hold_tuner, crate::sys::e384_setVoltageHoldTuner);
    measurement_cmd!(set_current_hold_tuner, crate::sys::e384_setCurrentHoldTuner);
    measurement_cmd!(set_voltage_half, crate::sys::e384_setVoltageHalf);
    measurement_cmd!(set_current_half, crate::sys::e384_setCurrentHalf);
    measurement_cmd!(
        set_liquid_junction_voltage,
        crate::sys::e384_setLiquidJunctionVoltage
    );
    measurement_cmd!(set_gate_voltages, crate::sys::e384_setGateVoltages);
    measurement_cmd!(set_source_voltages, crate::sys::e384_setSourceVoltages);
    measurement_cmd!(
        set_calib_vc_current_gain,
        crate::sys::e384_setCalibVcCurrentGain
    );
    measurement_cmd!(
        set_calib_vc_current_offset,
        crate::sys::e384_setCalibVcCurrentOffset
    );
    measurement_cmd!(
        set_calib_vc_voltage_gain,
        crate::sys::e384_setCalibVcVoltageGain
    );
    measurement_cmd!(
        set_calib_vc_voltage_offset,
        crate::sys::e384_setCalibVcVoltageOffset
    );
    measurement_cmd!(
        set_calib_cc_current_gain,
        crate::sys::e384_setCalibCcCurrentGain
    );
    measurement_cmd!(
        set_calib_cc_current_offset,
        crate::sys::e384_setCalibCcCurrentOffset
    );
    measurement_cmd!(
        set_calib_cc_voltage_gain,
        crate::sys::e384_setCalibCcVoltageGain
    );
    measurement_cmd!(
        set_calib_cc_voltage_offset,
        crate::sys::e384_setCalibCcVoltageOffset
    );
    measurement_cmd!(
        set_calib_rs_corr_offset_dac,
        crate::sys::e384_setCalibRsCorrOffsetDac
    );
    measurement_cmd!(
        set_calib_r_shunt_conductance,
        crate::sys::e384_setCalibRShuntConductance
    );

    update_cmd!(
        update_calib_vc_current_gain,
        crate::sys::e384_updateCalibVcCurrentGain
    );
    update_cmd!(
        update_calib_vc_current_offset,
        crate::sys::e384_updateCalibVcCurrentOffset
    );
    update_cmd!(
        update_calib_vc_voltage_gain,
        crate::sys::e384_updateCalibVcVoltageGain
    );
    update_cmd!(
        update_calib_vc_voltage_offset,
        crate::sys::e384_updateCalibVcVoltageOffset
    );
    update_cmd!(
        update_calib_cc_current_gain,
        crate::sys::e384_updateCalibCcCurrentGain
    );
    update_cmd!(
        update_calib_cc_current_offset,
        crate::sys::e384_updateCalibCcCurrentOffset
    );
    update_cmd!(
        update_calib_cc_voltage_gain,
        crate::sys::e384_updateCalibCcVoltageGain
    );
    update_cmd!(
        update_calib_cc_voltage_offset,
        crate::sys::e384_updateCalibCcVoltageOffset
    );
    update_cmd!(
        update_calib_rs_corr_offset_dac,
        crate::sys::e384_updateCalibRsCorrOffsetDac
    );
    update_cmd!(
        update_calib_r_shunt_conductance,
        crate::sys::e384_updateCalibRShuntConductance
    );
    update_cmd!(
        reset_offset_recalibration,
        crate::sys::e384_resetOffsetRecalibration
    );
    update_cmd!(
        reset_liquid_junction_voltage,
        crate::sys::e384_resetLiquidJunctionVoltage
    );

    bool_cmd!(enable_stimulus, crate::sys::e384_enableStimulus);
    bool_cmd!(turn_channels_on, crate::sys::e384_turnChannelsOn);
    bool_cmd!(turn_cal_sw_on, crate::sys::e384_turnCalSwOn);
    bool_cmd!(turn_vc_sw_on, crate::sys::e384_turnVcSwOn);
    bool_cmd!(turn_cc_sw_on, crate::sys::e384_turnCcSwOn);
    bool_cmd!(enable_cc_stimulus, crate::sys::e384_enableCcStimulus);
    bool_cmd!(
        readout_offset_recalibration,
        crate::sys::e384_readoutOffsetRecalibration
    );
    bool_cmd!(
        liquid_junction_compensation,
        crate::sys::e384_liquidJunctionCompensation
    );
    bool_cmd!(
        digital_offset_compensation,
        crate::sys::e384_digitalOffsetCompensation
    );
}
