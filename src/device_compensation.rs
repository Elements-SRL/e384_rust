//! Compensation features/values/ranges/options, the compensation value/flags matrix, and
//! clamping modality getters/setters.

use tracing::instrument;

use crate::device::Device;
use crate::error_codes::ErrorCodes;
use crate::sys::{
    E384ChannelSources, E384CompensationControl, E384Measurement, E384RangedMeasurement,
};
use crate::util::{collect_list, collect_matrix, owned_string_list, translate};

impl Device {
    /// Wraps `e384_getCompensationEnables`.
    #[instrument(level = "trace")]
    pub fn compensation_enables(
        &self,
        channels: &[u16],
        comp_type: i32,
    ) -> Result<Vec<bool>, ErrorCodes> {
        let mut out = vec![0u8; channels.len()];
        unsafe {
            translate(crate::sys::e384_getCompensationEnables(
                self.0,
                channels.as_ptr(),
                channels.len(),
                comp_type,
                out.as_mut_ptr(),
            ))
        }?;
        Ok(out.into_iter().map(|v| v != 0).collect())
    }

    /// Wraps `e384_getCompFeatures`: ranges plus the feature's default value.
    #[instrument(level = "trace")]
    pub fn comp_features(
        &self,
        feature: i32,
    ) -> Result<(Vec<E384RangedMeasurement>, f64), ErrorCodes> {
        let mut count: usize = 0;
        let mut default_value: f64 = 0.0;
        unsafe {
            translate(crate::sys::e384_getCompFeatures(
                self.0,
                feature,
                std::ptr::null_mut(),
                &mut count,
                &mut default_value,
            ))
        }?;
        let mut buf = vec![E384RangedMeasurement::default(); count];
        if count > 0 {
            unsafe {
                translate(crate::sys::e384_getCompFeatures(
                    self.0,
                    feature,
                    buf.as_mut_ptr(),
                    &mut count,
                    &mut default_value,
                ))
            }?;
            buf.truncate(count);
        }
        Ok((buf, default_value))
    }

    /// Wraps `e384_getCompOptionsFeatures`.
    #[instrument(level = "trace")]
    pub fn comp_options_features(&self, comp_type: i32) -> Result<Vec<String>, ErrorCodes> {
        let mut list = std::ptr::null_mut();
        unsafe {
            translate(crate::sys::e384_getCompOptionsFeatures(
                self.0, comp_type, &mut list,
            ))
        }?;
        Ok(unsafe { owned_string_list(list) })
    }

    /// Wraps `e384_getCompValueMatrix`. Row-major compensation value matrix.
    #[instrument(level = "trace")]
    pub fn comp_value_matrix(&self) -> Result<(Vec<f64>, usize, usize), ErrorCodes> {
        let dev = self.0;
        unsafe {
            collect_matrix(|out, rows, cols| {
                crate::sys::e384_getCompValueMatrix(dev, out, rows, cols)
            })
        }
    }

    /// Wraps `e384_getCompensationControl`.
    #[instrument(level = "trace")]
    pub fn compensation_control(&self, param: i32) -> Result<E384CompensationControl, ErrorCodes> {
        let mut out = E384CompensationControl::default();
        unsafe {
            translate(crate::sys::e384_getCompensationControl(
                self.0, param, &mut out,
            ))
        }?;
        Ok(out)
    }

    /// Wraps `e384_enableCompensation`. `channels`/`on` must be equal length.
    #[instrument(level = "trace")]
    pub fn enable_compensation(
        &self,
        channels: &[u16],
        on: &[bool],
        comp_type: i32,
        apply: bool,
    ) -> Result<(), ErrorCodes> {
        assert_eq!(channels.len(), on.len());
        let on_u8: Vec<u8> = on.iter().map(|&b| b as u8).collect();
        unsafe {
            translate(crate::sys::e384_enableCompensation(
                self.0,
                channels.as_ptr(),
                on_u8.as_ptr(),
                channels.len(),
                comp_type,
                apply as i32,
            ))
        }
    }

    /// Wraps `e384_setCompValues`. `channels`/`values` must be equal length.
    #[instrument(level = "trace")]
    pub fn set_comp_values(
        &self,
        channels: &[u16],
        values: &[f64],
        param_to_update: i32,
        apply: bool,
    ) -> Result<(), ErrorCodes> {
        assert_eq!(channels.len(), values.len());
        unsafe {
            translate(crate::sys::e384_setCompValues(
                self.0,
                channels.as_ptr(),
                values.as_ptr(),
                channels.len(),
                param_to_update,
                apply as i32,
            ))
        }
    }

    /// Wraps `e384_setCompRanges`. `channels`/`ranges` must be equal length.
    #[instrument(level = "trace")]
    pub fn set_comp_ranges(
        &self,
        channels: &[u16],
        ranges: &[u16],
        param_to_update: i32,
        apply: bool,
    ) -> Result<(), ErrorCodes> {
        assert_eq!(channels.len(), ranges.len());
        unsafe {
            translate(crate::sys::e384_setCompRanges(
                self.0,
                channels.as_ptr(),
                ranges.as_ptr(),
                channels.len(),
                param_to_update,
                apply as i32,
            ))
        }
    }

    /// Wraps `e384_setCompOptions`. `channels`/`options` must be equal length.
    #[instrument(level = "trace")]
    pub fn set_comp_options(
        &self,
        channels: &[u16],
        options: &[u16],
        comp_type: i32,
        apply: bool,
    ) -> Result<(), ErrorCodes> {
        assert_eq!(channels.len(), options.len());
        unsafe {
            translate(crate::sys::e384_setCompOptions(
                self.0,
                channels.as_ptr(),
                options.as_ptr(),
                channels.len(),
                comp_type,
                apply as i32,
            ))
        }
    }

    /// Wraps `e384_hasCompFeature`.
    #[instrument(level = "trace")]
    pub fn has_comp_feature(&self, feature: i32) -> Result<bool, ErrorCodes> {
        let mut out: i32 = 0;
        unsafe { translate(crate::sys::e384_hasCompFeature(self.0, feature, &mut out)) }?;
        Ok(out != 0)
    }

    /// Wraps `e384_getReadoutOffsetRecalibrationStatuses`.
    #[instrument(level = "trace")]
    pub fn readout_offset_recalibration_statuses(
        &self,
        channels: &[u16],
    ) -> Result<Vec<i32>, ErrorCodes> {
        let mut out = vec![0i32; channels.len()];
        unsafe {
            translate(crate::sys::e384_getReadoutOffsetRecalibrationStatuses(
                self.0,
                channels.as_ptr(),
                channels.len(),
                out.as_mut_ptr(),
            ))
        }?;
        Ok(out)
    }

    /// Wraps `e384_getLiquidJunctionStatuses`.
    #[instrument(level = "trace")]
    pub fn liquid_junction_statuses(&self, channels: &[u16]) -> Result<Vec<i32>, ErrorCodes> {
        let mut out = vec![0i32; channels.len()];
        unsafe {
            translate(crate::sys::e384_getLiquidJunctionStatuses(
                self.0,
                channels.as_ptr(),
                channels.len(),
                out.as_mut_ptr(),
            ))
        }?;
        Ok(out)
    }

    /// Wraps `e384_getLiquidJunctionVoltages`.
    #[instrument(level = "trace")]
    pub fn liquid_junction_voltages(
        &self,
        channels: &[u16],
    ) -> Result<Vec<E384Measurement>, ErrorCodes> {
        let mut out = vec![E384Measurement::default(); channels.len()];
        unsafe {
            translate(crate::sys::e384_getLiquidJunctionVoltages(
                self.0,
                channels.as_ptr(),
                channels.len(),
                out.as_mut_ptr(),
            ))
        }?;
        Ok(out)
    }

    /// Wraps `e384_getClampingModalitiesFeatures`.
    #[instrument(level = "trace")]
    pub fn clamping_modalities_features(&self) -> Result<Vec<i32>, ErrorCodes> {
        let dev = self.0;
        unsafe {
            collect_list(|out, count| {
                crate::sys::e384_getClampingModalitiesFeatures(dev, out, count)
            })
        }
    }

    /// Wraps `e384_getClampingModality`.
    #[instrument(level = "trace")]
    pub fn clamping_modality(&self) -> Result<i32, ErrorCodes> {
        let mut out: i32 = 0;
        unsafe { translate(crate::sys::e384_getClampingModality(self.0, &mut out)) }?;
        Ok(out)
    }

    /// Wraps `e384_setClampingModality_byIdx`.
    #[instrument(level = "trace")]
    pub fn set_clamping_modality_by_idx(
        &self,
        idx: u32,
        apply: bool,
        stop_protocol: bool,
    ) -> Result<(), ErrorCodes> {
        unsafe {
            translate(crate::sys::e384_setClampingModality_byIdx(
                self.0,
                idx,
                apply as i32,
                stop_protocol as i32,
            ))
        }
    }

    /// Wraps `e384_setClampingModality_byEnum`.
    #[instrument(level = "trace")]
    pub fn set_clamping_modality_by_enum(
        &self,
        mode: i32,
        apply: bool,
        stop_protocol: bool,
    ) -> Result<(), ErrorCodes> {
        unsafe {
            translate(crate::sys::e384_setClampingModality_byEnum(
                self.0,
                mode,
                apply as i32,
                stop_protocol as i32,
            ))
        }
    }

    /// Wraps `e384_getAvailableChannelsSourcesFeatures`.
    #[instrument(level = "trace")]
    pub fn available_channels_sources_features(
        &self,
    ) -> Result<(E384ChannelSources, E384ChannelSources), ErrorCodes> {
        let mut voltage = E384ChannelSources::default();
        let mut current = E384ChannelSources::default();
        unsafe {
            translate(crate::sys::e384_getAvailableChannelsSourcesFeatures(
                self.0,
                &mut voltage,
                &mut current,
            ))
        }?;
        Ok((voltage, current))
    }

    /// Wraps `e384_getTemperatureChannelsFeatures`: ranges plus per-channel display names.
    #[instrument(level = "trace")]
    pub fn temperature_channels_features(
        &self,
    ) -> Result<(Vec<E384RangedMeasurement>, Vec<String>), ErrorCodes> {
        let mut count: usize = 0;
        let mut names = std::ptr::null_mut();
        unsafe {
            translate(crate::sys::e384_getTemperatureChannelsFeatures(
                self.0,
                std::ptr::null_mut(),
                &mut count,
                &mut names,
            ))
        }?;
        let mut ranges = vec![E384RangedMeasurement::default(); count];
        if count > 0 {
            unsafe {
                translate(crate::sys::e384_getTemperatureChannelsFeatures(
                    self.0,
                    ranges.as_mut_ptr(),
                    &mut count,
                    &mut names,
                ))
            }?;
            ranges.truncate(count);
        }
        let names = if names.is_null() {
            Vec::new()
        } else {
            unsafe { owned_string_list(names) }
        };
        Ok((ranges, names))
    }
}
