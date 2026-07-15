//! Raw ADC sample (`i16`) to physical value (`f64`) conversion, and the disambiguated
//! per-channel overloads of the single-value converters.

use tracing::instrument;

use crate::device::Device;
use crate::error_codes::ErrorCodes;
use crate::util::translate;

impl Device {
    /// Wraps `e384_convertVoltageValues`. Out-of-place: `flt_values` must be the same length as
    /// `int_values`.
    #[instrument(level = "trace")]
    pub fn convert_voltage_values(
        &self,
        int_values: &mut [i16],
        flt_values: &mut [f64],
    ) -> Result<(), ErrorCodes> {
        assert_eq!(int_values.len(), flt_values.len());
        unsafe {
            translate(crate::sys::e384_convertVoltageValues(
                self.0,
                int_values.as_mut_ptr(),
                flt_values.as_mut_ptr(),
                int_values.len() as i32,
            ))
        }
    }

    /// Wraps `e384_convertCurrentValues`. `int_values`/`flt_values` must be equal length.
    #[instrument(level = "trace")]
    pub fn convert_current_values(
        &self,
        int_values: &mut [i16],
        flt_values: &mut [f64],
    ) -> Result<(), ErrorCodes> {
        assert_eq!(int_values.len(), flt_values.len());
        unsafe {
            translate(crate::sys::e384_convertCurrentValues(
                self.0,
                int_values.as_mut_ptr(),
                flt_values.as_mut_ptr(),
                int_values.len() as i32,
            ))
        }
    }

    /// Wraps `e384_convertTemperatureValues`. `int_values`/`flt_values` must be sized to the
    /// device's temperature channel count.
    #[instrument(level = "trace")]
    pub fn convert_temperature_values(
        &self,
        int_values: &mut [i16],
        flt_values: &mut [f64],
    ) -> Result<(), ErrorCodes> {
        unsafe {
            translate(crate::sys::e384_convertTemperatureValues(
                self.0,
                int_values.as_mut_ptr(),
                flt_values.as_mut_ptr(),
            ))
        }
    }

    /// Wraps `e384_convertOnTimeValue`.
    #[instrument(level = "trace")]
    pub fn convert_on_time_value(&self, int_values: &mut [i16; 2]) -> Result<f64, ErrorCodes> {
        let mut flt_value = 0.0;
        unsafe {
            translate(crate::sys::e384_convertOnTimeValue(
                self.0,
                int_values.as_mut_ptr(),
                &mut flt_value,
            ))
        }?;
        Ok(flt_value)
    }

    /// Wraps `e384_convertVoltageValue`.
    #[instrument(level = "trace")]
    pub fn convert_voltage_value(&self, int_value: i16) -> Result<f64, ErrorCodes> {
        let mut out = 0.0;
        unsafe {
            translate(crate::sys::e384_convertVoltageValue(
                self.0, int_value, &mut out,
            ))
        }?;
        Ok(out)
    }

    /// Wraps `e384_convertVoltageValue_byChannel`.
    #[instrument(level = "trace")]
    pub fn convert_voltage_value_by_channel(
        &self,
        int_value: i16,
        channel_idx: u16,
    ) -> Result<f64, ErrorCodes> {
        let mut out = 0.0;
        unsafe {
            translate(crate::sys::e384_convertVoltageValue_byChannel(
                self.0,
                int_value,
                channel_idx,
                &mut out,
            ))
        }?;
        Ok(out)
    }

    /// Wraps `e384_convertCurrentValue`.
    #[instrument(level = "trace")]
    pub fn convert_current_value(&self, int_value: i16) -> Result<f64, ErrorCodes> {
        let mut out = 0.0;
        unsafe {
            translate(crate::sys::e384_convertCurrentValue(
                self.0, int_value, &mut out,
            ))
        }?;
        Ok(out)
    }

    /// Wraps `e384_convertCurrentValue_byChannel`.
    #[instrument(level = "trace")]
    pub fn convert_current_value_by_channel(
        &self,
        int_value: i16,
        channel_idx: u16,
    ) -> Result<f64, ErrorCodes> {
        let mut out = 0.0;
        unsafe {
            translate(crate::sys::e384_convertCurrentValue_byChannel(
                self.0,
                int_value,
                channel_idx,
                &mut out,
            ))
        }?;
        Ok(out)
    }
}
