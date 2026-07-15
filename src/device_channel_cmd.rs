//! Shape A/B/C channel commands (per-channel hold tuner, calibration gain/offset, and on/off
//! switches). Each macro below takes a doc string as its first argument so every generated
//! method still gets its own one-line `/// Wraps \`e384_...\`` doc comment, even though the
//! bodies are otherwise identical mechanical calls into [`crate::util`]'s shape helpers.

use tracing::instrument;

use crate::device::Device;
use crate::error_codes::ErrorCodes;
use crate::sys::E384Measurement;
use crate::util::{channel_cmd_bool, channel_cmd_measurement, channel_cmd_update};

/// Shape A: `(device, channelIndexes, values, count, applyFlag)`.
macro_rules! measurement_cmd {
    ($doc:literal, $name:ident, $sys_fn:path) => {
        #[doc = $doc]
        #[instrument(level = "trace")]
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
    ($doc:literal, $name:ident, $sys_fn:path) => {
        #[doc = $doc]
        #[instrument(level = "trace")]
        pub fn $name(&self, channels: &[u16], apply: bool) -> Result<(), ErrorCodes> {
            let dev = self.0;
            unsafe { channel_cmd_update(|c, n, a| $sys_fn(dev, c, n, a), channels, apply) }
        }
    };
}

/// Shape C: `(device, channelIndexes, onValues, count, applyFlag)`.
macro_rules! bool_cmd {
    ($doc:literal, $name:ident, $sys_fn:path) => {
        #[doc = $doc]
        #[instrument(level = "trace")]
        pub fn $name(&self, channels: &[u16], on: &[bool], apply: bool) -> Result<(), ErrorCodes> {
            let dev = self.0;
            unsafe { channel_cmd_bool(|c, o, n, a| $sys_fn(dev, c, o, n, a), channels, on, apply) }
        }
    };
}

impl Device {
    measurement_cmd!(
        "Wraps `e384_setVoltageHoldTuner`.",
        set_voltage_hold_tuner,
        crate::sys::e384_setVoltageHoldTuner
    );
    measurement_cmd!(
        "Wraps `e384_setCurrentHoldTuner`.",
        set_current_hold_tuner,
        crate::sys::e384_setCurrentHoldTuner
    );
    measurement_cmd!(
        "Wraps `e384_setVoltageHalf`.",
        set_voltage_half,
        crate::sys::e384_setVoltageHalf
    );
    measurement_cmd!(
        "Wraps `e384_setCurrentHalf`.",
        set_current_half,
        crate::sys::e384_setCurrentHalf
    );
    measurement_cmd!(
        "Wraps `e384_setLiquidJunctionVoltage`.",
        set_liquid_junction_voltage,
        crate::sys::e384_setLiquidJunctionVoltage
    );
    measurement_cmd!(
        "Wraps `e384_setGateVoltages` (indexes are BOARD indexes, not channel indexes).",
        set_gate_voltages,
        crate::sys::e384_setGateVoltages
    );
    measurement_cmd!(
        "Wraps `e384_setSourceVoltages` (indexes are BOARD indexes, not channel indexes).",
        set_source_voltages,
        crate::sys::e384_setSourceVoltages
    );
    measurement_cmd!(
        "Wraps `e384_setCalibVcCurrentGain`.",
        set_calib_vc_current_gain,
        crate::sys::e384_setCalibVcCurrentGain
    );
    measurement_cmd!(
        "Wraps `e384_setCalibVcCurrentOffset`.",
        set_calib_vc_current_offset,
        crate::sys::e384_setCalibVcCurrentOffset
    );
    measurement_cmd!(
        "Wraps `e384_setCalibVcVoltageGain`.",
        set_calib_vc_voltage_gain,
        crate::sys::e384_setCalibVcVoltageGain
    );
    measurement_cmd!(
        "Wraps `e384_setCalibVcVoltageOffset`.",
        set_calib_vc_voltage_offset,
        crate::sys::e384_setCalibVcVoltageOffset
    );
    measurement_cmd!(
        "Wraps `e384_setCalibCcCurrentGain`.",
        set_calib_cc_current_gain,
        crate::sys::e384_setCalibCcCurrentGain
    );
    measurement_cmd!(
        "Wraps `e384_setCalibCcCurrentOffset`.",
        set_calib_cc_current_offset,
        crate::sys::e384_setCalibCcCurrentOffset
    );
    measurement_cmd!(
        "Wraps `e384_setCalibCcVoltageGain`.",
        set_calib_cc_voltage_gain,
        crate::sys::e384_setCalibCcVoltageGain
    );
    measurement_cmd!(
        "Wraps `e384_setCalibCcVoltageOffset`.",
        set_calib_cc_voltage_offset,
        crate::sys::e384_setCalibCcVoltageOffset
    );
    measurement_cmd!(
        "Wraps `e384_setCalibRsCorrOffsetDac`.",
        set_calib_rs_corr_offset_dac,
        crate::sys::e384_setCalibRsCorrOffsetDac
    );
    measurement_cmd!(
        "Wraps `e384_setCalibRShuntConductance`.",
        set_calib_r_shunt_conductance,
        crate::sys::e384_setCalibRShuntConductance
    );

    update_cmd!(
        "Wraps `e384_updateCalibVcCurrentGain`.",
        update_calib_vc_current_gain,
        crate::sys::e384_updateCalibVcCurrentGain
    );
    update_cmd!(
        "Wraps `e384_updateCalibVcCurrentOffset`.",
        update_calib_vc_current_offset,
        crate::sys::e384_updateCalibVcCurrentOffset
    );
    update_cmd!(
        "Wraps `e384_updateCalibVcVoltageGain`.",
        update_calib_vc_voltage_gain,
        crate::sys::e384_updateCalibVcVoltageGain
    );
    update_cmd!(
        "Wraps `e384_updateCalibVcVoltageOffset`.",
        update_calib_vc_voltage_offset,
        crate::sys::e384_updateCalibVcVoltageOffset
    );
    update_cmd!(
        "Wraps `e384_updateCalibCcCurrentGain`.",
        update_calib_cc_current_gain,
        crate::sys::e384_updateCalibCcCurrentGain
    );
    update_cmd!(
        "Wraps `e384_updateCalibCcCurrentOffset`.",
        update_calib_cc_current_offset,
        crate::sys::e384_updateCalibCcCurrentOffset
    );
    update_cmd!(
        "Wraps `e384_updateCalibCcVoltageGain`.",
        update_calib_cc_voltage_gain,
        crate::sys::e384_updateCalibCcVoltageGain
    );
    update_cmd!(
        "Wraps `e384_updateCalibCcVoltageOffset`.",
        update_calib_cc_voltage_offset,
        crate::sys::e384_updateCalibCcVoltageOffset
    );
    update_cmd!(
        "Wraps `e384_updateCalibRsCorrOffsetDac`.",
        update_calib_rs_corr_offset_dac,
        crate::sys::e384_updateCalibRsCorrOffsetDac
    );
    update_cmd!(
        "Wraps `e384_updateCalibRShuntConductance`.",
        update_calib_r_shunt_conductance,
        crate::sys::e384_updateCalibRShuntConductance
    );
    update_cmd!(
        "Wraps `e384_resetOffsetRecalibration`.",
        reset_offset_recalibration,
        crate::sys::e384_resetOffsetRecalibration
    );
    update_cmd!(
        "Wraps `e384_resetLiquidJunctionVoltage`.",
        reset_liquid_junction_voltage,
        crate::sys::e384_resetLiquidJunctionVoltage
    );

    bool_cmd!(
        "Wraps `e384_enableStimulus`.",
        enable_stimulus,
        crate::sys::e384_enableStimulus
    );
    bool_cmd!(
        "Wraps `e384_turnChannelsOn`.",
        turn_channels_on,
        crate::sys::e384_turnChannelsOn
    );
    bool_cmd!(
        "Wraps `e384_turnCalSwOn`.",
        turn_cal_sw_on,
        crate::sys::e384_turnCalSwOn
    );
    bool_cmd!(
        "Wraps `e384_turnVcSwOn`.",
        turn_vc_sw_on,
        crate::sys::e384_turnVcSwOn
    );
    bool_cmd!(
        "Wraps `e384_turnCcSwOn`.",
        turn_cc_sw_on,
        crate::sys::e384_turnCcSwOn
    );
    bool_cmd!(
        "Wraps `e384_enableCcStimulus`.",
        enable_cc_stimulus,
        crate::sys::e384_enableCcStimulus
    );
    bool_cmd!(
        "Wraps `e384_readoutOffsetRecalibration`.",
        readout_offset_recalibration,
        crate::sys::e384_readoutOffsetRecalibration
    );
    bool_cmd!(
        "Wraps `e384_liquidJunctionCompensation`.",
        liquid_junction_compensation,
        crate::sys::e384_liquidJunctionCompensation
    );
    bool_cmd!(
        "Wraps `e384_digitalOffsetCompensation`.",
        digital_offset_compensation,
        crate::sys::e384_digitalOffsetCompensation
    );
}
