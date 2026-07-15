//! Custom flags/options/doubles, debug bit/word, state array, zap, SPI command, ADC core, and
//! the misc scalar/measurement commands from the header's hand-written tail section.
//! `e384_getCustomOptions` (ragged `vector<vector<string>>`) is deliberately not wrapped — the
//! header itself defers it as a nested opaque-tree problem never cleanly exposed at the C boundary.

use tracing::instrument;

use crate::device::Device;
use crate::error_codes::ErrorCodes;
use crate::sys::{E384Measurement, E384PidParams, E384RangedMeasurement};
use crate::util::{owned_string_list, translate};

/// `(device, int32_t value1, int32_t value2) -> E384Err`
macro_rules! two_int_cmd {
    ($doc:literal, $name:ident, $sys_fn:path) => {
        #[doc = $doc]
        #[instrument(level = "trace")]
        pub fn $name(&self, value1: i32, value2: i32) -> Result<(), ErrorCodes> {
            unsafe { translate($sys_fn(self.0, value1, value2)) }
        }
    };
}

/// `(device, int32_t value) -> E384Err`
macro_rules! one_int_cmd {
    ($doc:literal, $name:ident, $sys_fn:path) => {
        #[doc = $doc]
        #[instrument(level = "trace")]
        pub fn $name(&self, value: i32) -> Result<(), ErrorCodes> {
            unsafe { translate($sys_fn(self.0, value)) }
        }
    };
}

/// `(device, E384Measurement value, int32_t flag) -> E384Err`
macro_rules! measurement_flag_cmd {
    ($doc:literal, $name:ident, $sys_fn:path) => {
        #[doc = $doc]
        #[instrument(level = "trace")]
        pub fn $name(&self, value: E384Measurement, flag: bool) -> Result<(), ErrorCodes> {
            unsafe { translate($sys_fn(self.0, value, flag as i32)) }
        }
    };
}

impl Device {
    two_int_cmd!(
        "Wraps `e384_resetAsic`.",
        reset_asic,
        crate::sys::e384_resetAsic
    );
    two_int_cmd!(
        "Wraps `e384_resetFpga`.",
        reset_fpga,
        crate::sys::e384_resetFpga
    );
    two_int_cmd!(
        "Wraps `e384_turnVoltageReaderOn`.",
        turn_voltage_reader_on,
        crate::sys::e384_turnVoltageReaderOn
    );
    two_int_cmd!(
        "Wraps `e384_turnCurrentReaderOn`.",
        turn_current_reader_on,
        crate::sys::e384_turnCurrentReaderOn
    );
    two_int_cmd!(
        "Wraps `e384_turnVoltageStimulusOn`.",
        turn_voltage_stimulus_on,
        crate::sys::e384_turnVoltageStimulusOn
    );
    two_int_cmd!(
        "Wraps `e384_turnCurrentStimulusOn`.",
        turn_current_stimulus_on,
        crate::sys::e384_turnCurrentStimulusOn
    );
    two_int_cmd!(
        "Wraps `e384_enableVcCompensations`.",
        enable_vc_compensations,
        crate::sys::e384_enableVcCompensations
    );
    two_int_cmd!(
        "Wraps `e384_enableCcCompensations`.",
        enable_cc_compensations,
        crate::sys::e384_enableCcCompensations
    );

    one_int_cmd!(
        "Wraps `e384_subtractLiquidJunctionFromCc`.",
        subtract_liquid_junction_from_cc,
        crate::sys::e384_subtractLiquidJunctionFromCc
    );
    one_int_cmd!(
        "Wraps `e384_setCalibrationMode`.",
        set_calibration_mode,
        crate::sys::e384_setCalibrationMode
    );

    measurement_flag_cmd!(
        "Wraps `e384_setVoltageReference`.",
        set_voltage_reference,
        crate::sys::e384_setVoltageReference
    );
    measurement_flag_cmd!(
        "Wraps `e384_setCoolingFansSpeed`.",
        set_cooling_fans_speed,
        crate::sys::e384_setCoolingFansSpeed
    );
    measurement_flag_cmd!(
        "Wraps `e384_setTemperatureControl`.",
        set_temperature_control,
        crate::sys::e384_setTemperatureControl
    );

    /// Wraps `e384_enableRxMessageType`.
    #[instrument(level = "trace")]
    pub fn enable_rx_message_type(&self, message_type: i32, flag: bool) -> Result<(), ErrorCodes> {
        unsafe {
            translate(crate::sys::e384_enableRxMessageType(
                self.0,
                message_type,
                flag as i32,
            ))
        }
    }

    /// Wraps `e384_setAdcCore`. `channels`/`clamping_modes` must be equal length.
    #[instrument(level = "trace")]
    pub fn set_adc_core(
        &self,
        channels: &[u16],
        clamping_modes: &[i32],
        apply: bool,
    ) -> Result<(), ErrorCodes> {
        assert_eq!(channels.len(), clamping_modes.len());
        unsafe {
            translate(crate::sys::e384_setAdcCore(
                self.0,
                channels.as_ptr(),
                clamping_modes.as_ptr(),
                channels.len(),
                apply as i32,
            ))
        }
    }

    /// Wraps `e384_sendSpiCommand`.
    #[instrument(level = "trace")]
    pub fn send_spi_command(&self, command: u32, data_load: u32) -> Result<(), ErrorCodes> {
        unsafe { translate(crate::sys::e384_sendSpiCommand(self.0, command, data_load)) }
    }

    /// Wraps `e384_setCustomFlag`.
    #[instrument(level = "trace")]
    pub fn set_custom_flag(&self, idx: u16, flag: bool, apply: bool) -> Result<(), ErrorCodes> {
        unsafe {
            translate(crate::sys::e384_setCustomFlag(
                self.0,
                idx,
                flag as i32,
                apply as i32,
            ))
        }
    }

    /// Wraps `e384_setCustomOption`.
    #[instrument(level = "trace")]
    pub fn set_custom_option(&self, idx: u16, value: u16, apply: bool) -> Result<(), ErrorCodes> {
        unsafe {
            translate(crate::sys::e384_setCustomOption(
                self.0,
                idx,
                value,
                apply as i32,
            ))
        }
    }

    /// Wraps `e384_setCustomDouble`.
    #[instrument(level = "trace")]
    pub fn set_custom_double(&self, idx: u16, value: f64, apply: bool) -> Result<(), ErrorCodes> {
        unsafe {
            translate(crate::sys::e384_setCustomDouble(
                self.0,
                idx,
                value,
                apply as i32,
            ))
        }
    }

    /// Wraps `e384_setDebugBit`.
    #[instrument(level = "trace")]
    pub fn set_debug_bit(
        &self,
        word_offset: u16,
        bit_offset: u16,
        status: bool,
        apply: bool,
    ) -> Result<(), ErrorCodes> {
        unsafe {
            translate(crate::sys::e384_setDebugBit(
                self.0,
                word_offset,
                bit_offset,
                status as i32,
                apply as i32,
            ))
        }
    }

    /// Wraps `e384_setDebugWord`.
    #[instrument(level = "trace")]
    pub fn set_debug_word(&self, word_offset: u16, word_value: u16) -> Result<(), ErrorCodes> {
        unsafe {
            translate(crate::sys::e384_setDebugWord(
                self.0,
                word_offset,
                word_value,
            ))
        }
    }

    /// Wraps `e384_setStateArrayEnabled`.
    #[instrument(level = "trace")]
    pub fn set_state_array_enabled(&self, ch_idx: i32, enabled: bool) -> Result<(), ErrorCodes> {
        unsafe {
            translate(crate::sys::e384_setStateArrayEnabled(
                self.0,
                ch_idx,
                enabled as i32,
            ))
        }
    }

    /// Wraps `e384_setTemperatureControlPid`.
    #[instrument(level = "trace")]
    pub fn set_temperature_control_pid(&self, params: E384PidParams) -> Result<(), ErrorCodes> {
        unsafe { translate(crate::sys::e384_setTemperatureControlPid(self.0, params)) }
    }

    /// Wraps `e384_zap`.
    #[instrument(level = "trace")]
    pub fn zap(&self, channels: &[u16], duration: E384Measurement) -> Result<(), ErrorCodes> {
        unsafe {
            translate(crate::sys::e384_zap(
                self.0,
                channels.as_ptr(),
                channels.len(),
                duration,
            ))
        }
    }

    /// Wraps `e384_setStateArrayStructure`.
    #[instrument(level = "trace")]
    pub fn set_state_array_structure(
        &self,
        number_of_states: i32,
        initial_state: i32,
        reaction_time: E384Measurement,
    ) -> Result<(), ErrorCodes> {
        unsafe {
            translate(crate::sys::e384_setStateArrayStructure(
                self.0,
                number_of_states,
                initial_state,
                reaction_time,
            ))
        }
    }

    /// Wraps `e384_setSateArrayState` (typo preserved from the C header's exported symbol name).
    #[allow(clippy::too_many_arguments)]
    #[instrument(level = "trace")]
    pub fn set_state_array_state(
        &self,
        state_idx: i32,
        voltage: E384Measurement,
        timeout_state_flag: bool,
        timeout: E384Measurement,
        timeout_state: i32,
        min_trigger_value: E384Measurement,
        max_trigger_value: E384Measurement,
        trigger_state: i32,
        trigger_flag: bool,
        delta_flag: bool,
    ) -> Result<(), ErrorCodes> {
        unsafe {
            translate(crate::sys::e384_setSateArrayState(
                self.0,
                state_idx,
                voltage,
                timeout_state_flag as i32,
                timeout,
                timeout_state,
                min_trigger_value,
                max_trigger_value,
                trigger_state,
                trigger_flag as i32,
                delta_flag as i32,
            ))
        }
    }

    /// Wraps `e384_getCustomFlags`: custom boolean flags, their default values, and their
    /// display names.
    #[instrument(level = "trace")]
    pub fn custom_flags(&self) -> Result<(Vec<bool>, Vec<String>), ErrorCodes> {
        let mut count: usize = 0;
        let mut names = std::ptr::null_mut();
        unsafe {
            translate(crate::sys::e384_getCustomFlags(
                self.0,
                std::ptr::null_mut(),
                &mut count,
                &mut names,
            ))
        }?;
        let mut defaults = vec![0u8; count];
        if count > 0 {
            unsafe {
                translate(crate::sys::e384_getCustomFlags(
                    self.0,
                    defaults.as_mut_ptr(),
                    &mut count,
                    &mut names,
                ))
            }?;
            defaults.truncate(count);
        }
        let names = if names.is_null() {
            Vec::new()
        } else {
            unsafe { owned_string_list(names) }
        };
        Ok((defaults.into_iter().map(|v| v != 0).collect(), names))
    }

    /// Wraps `e384_getCustomDoubles`: custom double-valued ranges, their default values, and
    /// their display names.
    #[instrument(level = "trace")]
    pub fn custom_doubles(
        &self,
    ) -> Result<(Vec<E384RangedMeasurement>, Vec<f64>, Vec<String>), ErrorCodes> {
        let mut count: usize = 0;
        let mut names = std::ptr::null_mut();
        unsafe {
            translate(crate::sys::e384_getCustomDoubles(
                self.0,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut count,
                &mut names,
            ))
        }?;
        let mut ranges = vec![E384RangedMeasurement::default(); count];
        let mut defaults = vec![0f64; count];
        if count > 0 {
            unsafe {
                translate(crate::sys::e384_getCustomDoubles(
                    self.0,
                    ranges.as_mut_ptr(),
                    defaults.as_mut_ptr(),
                    &mut count,
                    &mut names,
                ))
            }?;
            ranges.truncate(count);
            defaults.truncate(count);
        }
        let names = if names.is_null() {
            Vec::new()
        } else {
            unsafe { owned_string_list(names) }
        };
        Ok((ranges, defaults, names))
    }
}
