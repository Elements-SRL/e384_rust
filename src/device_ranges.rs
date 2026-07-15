//! VC/CC current & voltage ranges, filters, sampling rate, and the disambiguated range-getter
//! overload set (`e384c.h` splits several C++ overloads by suffix, e.g. `_list`/`_perChannel`).
//! Range getters that use the two-call protocol with a default-index out-param return
//! `(ranges, default_idx)` — see `crate::util::collect_list_with_default_idx`.

use tracing::instrument;

use crate::device::Device;
use crate::error_codes::ErrorCodes;
use crate::sys::{E384Measurement, E384RangedMeasurement};
use crate::util::{collect_list, collect_list_with_default_idx, translate};

/// `(device, uint16_t value, int32_t applyFlag) -> E384Err`
macro_rules! scalar_setter {
    ($doc:literal, $name:ident, $sys_fn:path) => {
        #[doc = $doc]
        #[instrument(level = "trace")]
        pub fn $name(&self, value: u16, apply: bool) -> Result<(), ErrorCodes> {
            unsafe { translate($sys_fn(self.0, value, apply as i32)) }
        }
    };
}

/// `(device, uint32_t* out) -> E384Err`
macro_rules! u32_getter {
    ($doc:literal, $name:ident, $sys_fn:path) => {
        #[doc = $doc]
        #[instrument(level = "trace")]
        pub fn $name(&self) -> Result<u32, ErrorCodes> {
            let mut out: u32 = 0;
            unsafe { translate($sys_fn(self.0, &mut out)) }?;
            Ok(out)
        }
    };
}

/// `(device, E384Measurement* out) -> E384Err`
macro_rules! measurement_getter {
    ($doc:literal, $name:ident, $sys_fn:path) => {
        #[doc = $doc]
        #[instrument(level = "trace")]
        pub fn $name(&self) -> Result<E384Measurement, ErrorCodes> {
            let mut out = E384Measurement::default();
            unsafe { translate($sys_fn(self.0, &mut out)) }?;
            Ok(out)
        }
    };
}

/// Two-call `(device, E384Measurement* out, size_t* count) -> E384Err`
macro_rules! measurement_list_getter {
    ($doc:literal, $name:ident, $sys_fn:path) => {
        #[doc = $doc]
        #[instrument(level = "trace")]
        pub fn $name(&self) -> Result<Vec<E384Measurement>, ErrorCodes> {
            let dev = self.0;
            unsafe { collect_list(|out, count| $sys_fn(dev, out, count)) }
        }
    };
}

/// Two-call `(device, E384RangedMeasurement* out, size_t* count) -> E384Err`, no default idx.
macro_rules! ranged_list_getter {
    ($doc:literal, $name:ident, $sys_fn:path) => {
        #[doc = $doc]
        #[instrument(level = "trace")]
        pub fn $name(&self) -> Result<Vec<E384RangedMeasurement>, ErrorCodes> {
            let dev = self.0;
            unsafe { collect_list(|out, count| $sys_fn(dev, out, count)) }
        }
    };
}

/// Two-call `(device, E384RangedMeasurement* out, size_t* count, uint16_t* outDefaultIdx) -> E384Err`.
macro_rules! ranged_list_with_default_idx_getter {
    ($doc:literal, $name:ident, $sys_fn:path) => {
        #[doc = $doc]
        #[instrument(level = "trace")]
        pub fn $name(&self) -> Result<(Vec<E384RangedMeasurement>, u16), ErrorCodes> {
            let dev = self.0;
            unsafe {
                collect_list_with_default_idx(|out, count, idx| $sys_fn(dev, out, count, idx))
            }
        }
    };
}

/// `(device, E384RangedMeasurement* outRange, uint32_t* outIdx) -> E384Err`
macro_rules! ranged_with_idx_getter {
    ($doc:literal, $name:ident, $sys_fn:path) => {
        #[doc = $doc]
        #[instrument(level = "trace")]
        pub fn $name(&self) -> Result<(E384RangedMeasurement, u32), ErrorCodes> {
            let mut range = E384RangedMeasurement::default();
            let mut idx: u32 = 0;
            unsafe { translate($sys_fn(self.0, &mut range, &mut idx)) }?;
            Ok((range, idx))
        }
    };
}

impl Device {
    // -- current/voltage ranges --------------------------------------------------
    /// Wraps `e384_getVCCurrentRange`.
    #[instrument(level = "trace")]
    pub fn vc_current_range(&self) -> Result<E384RangedMeasurement, ErrorCodes> {
        let mut out = E384RangedMeasurement::default();
        unsafe { translate(crate::sys::e384_getVCCurrentRange(self.0, &mut out)) }?;
        Ok(out)
    }

    ranged_list_with_default_idx_getter!(
        "Wraps `e384_getVCCurrentRanges`.",
        vc_current_ranges,
        crate::sys::e384_getVCCurrentRanges
    );
    ranged_list_with_default_idx_getter!(
        "Wraps `e384_getVCVoltageRanges`.",
        vc_voltage_ranges,
        crate::sys::e384_getVCVoltageRanges
    );
    ranged_list_with_default_idx_getter!(
        "Wraps `e384_getCCCurrentRanges`.",
        cc_current_ranges,
        crate::sys::e384_getCCCurrentRanges
    );
    ranged_list_with_default_idx_getter!(
        "Wraps `e384_getCCVoltageRanges`.",
        cc_voltage_ranges,
        crate::sys::e384_getCCVoltageRanges
    );

    ranged_with_idx_getter!(
        "Wraps `e384_getMaxVCCurrentRange`.",
        max_vc_current_range,
        crate::sys::e384_getMaxVCCurrentRange
    );
    ranged_with_idx_getter!(
        "Wraps `e384_getMinVCCurrentRange`.",
        min_vc_current_range,
        crate::sys::e384_getMinVCCurrentRange
    );
    ranged_with_idx_getter!(
        "Wraps `e384_getMaxVCVoltageRange`.",
        max_vc_voltage_range,
        crate::sys::e384_getMaxVCVoltageRange
    );
    ranged_with_idx_getter!(
        "Wraps `e384_getMinVCVoltageRange`.",
        min_vc_voltage_range,
        crate::sys::e384_getMinVCVoltageRange
    );
    ranged_with_idx_getter!(
        "Wraps `e384_getMaxCCCurrentRange`.",
        max_cc_current_range,
        crate::sys::e384_getMaxCCCurrentRange
    );
    ranged_with_idx_getter!(
        "Wraps `e384_getMinCCCurrentRange`.",
        min_cc_current_range,
        crate::sys::e384_getMinCCCurrentRange
    );
    ranged_with_idx_getter!(
        "Wraps `e384_getMaxCCVoltageRange`.",
        max_cc_voltage_range,
        crate::sys::e384_getMaxCCVoltageRange
    );
    ranged_with_idx_getter!(
        "Wraps `e384_getMinCCVoltageRange`.",
        min_cc_voltage_range,
        crate::sys::e384_getMinCCVoltageRange
    );

    scalar_setter!(
        "Wraps `e384_setVCCurrentRange_all`.",
        set_vc_current_range_all,
        crate::sys::e384_setVCCurrentRange_all
    );
    scalar_setter!(
        "Wraps `e384_setVCVoltageRange`.",
        set_vc_voltage_range,
        crate::sys::e384_setVCVoltageRange
    );
    scalar_setter!(
        "Wraps `e384_setCCCurrentRange`.",
        set_cc_current_range,
        crate::sys::e384_setCCCurrentRange
    );
    scalar_setter!(
        "Wraps `e384_setCCVoltageRange_all`.",
        set_cc_voltage_range_all,
        crate::sys::e384_setCCVoltageRange_all
    );
    scalar_setter!(
        "Wraps `e384_setVoltageStimulusLpf`.",
        set_voltage_stimulus_lpf,
        crate::sys::e384_setVoltageStimulusLpf
    );
    scalar_setter!(
        "Wraps `e384_setCurrentStimulusLpf`.",
        set_current_stimulus_lpf,
        crate::sys::e384_setCurrentStimulusLpf
    );
    scalar_setter!(
        "Wraps `e384_setSourceForVoltageChannel`.",
        set_source_for_voltage_channel,
        crate::sys::e384_setSourceForVoltageChannel
    );
    scalar_setter!(
        "Wraps `e384_setSourceForCurrentChannel`.",
        set_source_for_current_channel,
        crate::sys::e384_setSourceForCurrentChannel
    );
    scalar_setter!(
        "Wraps `e384_setSamplingRate`.",
        set_sampling_rate,
        crate::sys::e384_setSamplingRate
    );

    /// Wraps `e384_setDownsamplingRatio` (`(device, uint32_t value) -> E384Err`).
    #[instrument(level = "trace")]
    pub fn set_downsampling_ratio(&self, value: u32) -> Result<(), ErrorCodes> {
        unsafe { translate(crate::sys::e384_setDownsamplingRatio(self.0, value)) }
    }

    // -- index/scalar getters -----------------------------------------------------
    u32_getter!(
        "Wraps `e384_getClampingModalityIdx`.",
        clamping_modality_idx,
        crate::sys::e384_getClampingModalityIdx
    );
    u32_getter!(
        "Wraps `e384_getVCVoltageRangeIdx`.",
        vc_voltage_range_idx,
        crate::sys::e384_getVCVoltageRangeIdx
    );
    u32_getter!(
        "Wraps `e384_getVCCurrentRangeIdx`.",
        vc_current_range_idx,
        crate::sys::e384_getVCCurrentRangeIdx
    );
    u32_getter!(
        "Wraps `e384_getCCCurrentRangeIdx`.",
        cc_current_range_idx,
        crate::sys::e384_getCCCurrentRangeIdx
    );
    u32_getter!(
        "Wraps `e384_getCCVoltageRangeIdx`.",
        cc_voltage_range_idx,
        crate::sys::e384_getCCVoltageRangeIdx
    );
    u32_getter!(
        "Wraps `e384_getSamplingRateIdx`.",
        sampling_rate_idx,
        crate::sys::e384_getSamplingRateIdx
    );
    u32_getter!(
        "Wraps `e384_getMaxDownsamplingRatioFeature`.",
        max_downsampling_ratio_feature,
        crate::sys::e384_getMaxDownsamplingRatioFeature
    );
    u32_getter!(
        "Wraps `e384_getDownsamplingRatio`.",
        downsampling_ratio,
        crate::sys::e384_getDownsamplingRatio
    );
    u32_getter!(
        "Wraps `e384_getVCVoltageFilterIdx`.",
        vc_voltage_filter_idx,
        crate::sys::e384_getVCVoltageFilterIdx
    );
    u32_getter!(
        "Wraps `e384_getVCCurrentFilterIdx`.",
        vc_current_filter_idx,
        crate::sys::e384_getVCCurrentFilterIdx
    );
    u32_getter!(
        "Wraps `e384_getCCVoltageFilterIdx`.",
        cc_voltage_filter_idx,
        crate::sys::e384_getCCVoltageFilterIdx
    );
    u32_getter!(
        "Wraps `e384_getCCCurrentFilterIdx`.",
        cc_current_filter_idx,
        crate::sys::e384_getCCCurrentFilterIdx
    );
    u32_getter!(
        "Wraps `e384_getMaxProtocolItemsFeature`.",
        max_protocol_items_feature,
        crate::sys::e384_getMaxProtocolItemsFeature
    );
    u32_getter!(
        "Wraps `e384_getCalibrationEepromSize`.",
        calibration_eeprom_size,
        crate::sys::e384_getCalibrationEepromSize
    );

    // -- measurement getters --------------------------------------------------------
    measurement_getter!(
        "Wraps `e384_getSamplingRate`.",
        sampling_rate,
        crate::sys::e384_getSamplingRate
    );
    measurement_getter!(
        "Wraps `e384_getVCVoltageFilter`.",
        vc_voltage_filter,
        crate::sys::e384_getVCVoltageFilter
    );
    measurement_getter!(
        "Wraps `e384_getVCCurrentFilter`.",
        vc_current_filter,
        crate::sys::e384_getVCCurrentFilter
    );
    measurement_getter!(
        "Wraps `e384_getCCVoltageFilter`.",
        cc_voltage_filter,
        crate::sys::e384_getCCVoltageFilter
    );
    measurement_getter!(
        "Wraps `e384_getCCCurrentFilter`.",
        cc_current_filter,
        crate::sys::e384_getCCCurrentFilter
    );

    // -- measurement list getters -----------------------------------------------------
    measurement_list_getter!(
        "Wraps `e384_getSamplingRatesFeatures`.",
        sampling_rates_features,
        crate::sys::e384_getSamplingRatesFeatures
    );
    measurement_list_getter!(
        "Wraps `e384_getRealSamplingRatesFeatures`.",
        real_sampling_rates_features,
        crate::sys::e384_getRealSamplingRatesFeatures
    );
    measurement_list_getter!(
        "Wraps `e384_getVCVoltageFilters`.",
        vc_voltage_filters,
        crate::sys::e384_getVCVoltageFilters
    );
    measurement_list_getter!(
        "Wraps `e384_getVCCurrentFilters`.",
        vc_current_filters,
        crate::sys::e384_getVCCurrentFilters
    );
    measurement_list_getter!(
        "Wraps `e384_getCCVoltageFilters`.",
        cc_voltage_filters,
        crate::sys::e384_getCCVoltageFilters
    );
    measurement_list_getter!(
        "Wraps `e384_getCCCurrentFilters`.",
        cc_current_filters,
        crate::sys::e384_getCCCurrentFilters
    );
    measurement_list_getter!(
        "Wraps `e384_getVoltageHoldTuner`.",
        voltage_hold_tuner,
        crate::sys::e384_getVoltageHoldTuner
    );

    // -- ranged list getters (no default idx) ------------------------------------------
    ranged_list_getter!(
        "Wraps `e384_getVoltageHoldTunerFeatures`.",
        voltage_hold_tuner_features,
        crate::sys::e384_getVoltageHoldTunerFeatures
    );
    ranged_list_getter!(
        "Wraps `e384_getVoltageHalfFeatures`.",
        voltage_half_features,
        crate::sys::e384_getVoltageHalfFeatures
    );
    ranged_list_getter!(
        "Wraps `e384_getCurrentHoldTunerFeatures`.",
        current_hold_tuner_features,
        crate::sys::e384_getCurrentHoldTunerFeatures
    );
    ranged_list_getter!(
        "Wraps `e384_getCurrentHalfFeatures`.",
        current_half_features,
        crate::sys::e384_getCurrentHalfFeatures
    );
    ranged_list_getter!(
        "Wraps `e384_getLiquidJunctionRangesFeatures`.",
        liquid_junction_ranges_features,
        crate::sys::e384_getLiquidJunctionRangesFeatures
    );

    // -- disambiguated overloads --------------------------------------------------------
    /// Wraps `e384_setVCCurrentRange_perChannel`.
    #[instrument(level = "trace")]
    pub fn set_vc_current_range_per_channel(
        &self,
        channels: &[u16],
        current_range_idx: &[u16],
        apply: bool,
    ) -> Result<(), ErrorCodes> {
        assert_eq!(channels.len(), current_range_idx.len());
        unsafe {
            translate(crate::sys::e384_setVCCurrentRange_perChannel(
                self.0,
                channels.as_ptr(),
                current_range_idx.as_ptr(),
                channels.len(),
                apply as i32,
            ))
        }
    }

    /// Wraps `e384_setCCVoltageRange_perChannel`.
    #[instrument(level = "trace")]
    pub fn set_cc_voltage_range_per_channel(
        &self,
        channels: &[u16],
        voltage_range_idx: &[u16],
        apply: bool,
    ) -> Result<(), ErrorCodes> {
        assert_eq!(channels.len(), voltage_range_idx.len());
        unsafe {
            translate(crate::sys::e384_setCCVoltageRange_perChannel(
                self.0,
                channels.as_ptr(),
                voltage_range_idx.as_ptr(),
                channels.len(),
                apply as i32,
            ))
        }
    }

    ranged_list_getter!(
        "Wraps `e384_getVCCurrentRange_list`.",
        vc_current_range_list,
        crate::sys::e384_getVCCurrentRange_list
    );

    /// Wraps `e384_getCCVoltageRange`.
    #[instrument(level = "trace")]
    pub fn cc_voltage_range(&self) -> Result<E384RangedMeasurement, ErrorCodes> {
        let mut out = E384RangedMeasurement::default();
        unsafe { translate(crate::sys::e384_getCCVoltageRange(self.0, &mut out)) }?;
        Ok(out)
    }
    ranged_list_getter!(
        "Wraps `e384_getCCVoltageRange_list`.",
        cc_voltage_range_list,
        crate::sys::e384_getCCVoltageRange_list
    );

    /// Wraps `e384_getVoltageRange`.
    #[instrument(level = "trace")]
    pub fn voltage_range(&self) -> Result<E384RangedMeasurement, ErrorCodes> {
        let mut out = E384RangedMeasurement::default();
        unsafe { translate(crate::sys::e384_getVoltageRange(self.0, &mut out)) }?;
        Ok(out)
    }
    ranged_list_getter!(
        "Wraps `e384_getVoltageRange_list`.",
        voltage_range_list,
        crate::sys::e384_getVoltageRange_list
    );

    /// Wraps `e384_getCurrentRange`.
    #[instrument(level = "trace")]
    pub fn current_range(&self) -> Result<E384RangedMeasurement, ErrorCodes> {
        let mut out = E384RangedMeasurement::default();
        unsafe { translate(crate::sys::e384_getCurrentRange(self.0, &mut out)) }?;
        Ok(out)
    }
    ranged_list_getter!(
        "Wraps `e384_getCurrentRange_list`.",
        current_range_list,
        crate::sys::e384_getCurrentRange_list
    );

    /// Wraps `e384_getVCCurrentRangeIdx_list` (two-call `(device, uint32_t* out, size_t* count) -> E384Err`).
    #[instrument(level = "trace")]
    pub fn vc_current_range_idx_list(&self) -> Result<Vec<u32>, ErrorCodes> {
        let dev = self.0;
        unsafe {
            collect_list(|out, count| crate::sys::e384_getVCCurrentRangeIdx_list(dev, out, count))
        }
    }

    /// Wraps `e384_getCCVoltageRangeIdx_list`.
    #[instrument(level = "trace")]
    pub fn cc_voltage_range_idx_list(&self) -> Result<Vec<u32>, ErrorCodes> {
        let dev = self.0;
        unsafe {
            collect_list(|out, count| crate::sys::e384_getCCVoltageRangeIdx_list(dev, out, count))
        }
    }

    /// Wraps `e384_getVCCurrentRanges_perChannel`: per-channel VC current ranges with their
    /// default index per channel.
    #[instrument(level = "trace")]
    pub fn vc_current_ranges_per_channel(
        &self,
    ) -> Result<(Vec<E384RangedMeasurement>, Vec<u16>), ErrorCodes> {
        let mut ranges_count: usize = 0;
        let mut idx_count: usize = 0;
        unsafe {
            translate(crate::sys::e384_getVCCurrentRanges_perChannel(
                self.0,
                std::ptr::null_mut(),
                &mut ranges_count,
                std::ptr::null_mut(),
                &mut idx_count,
            ))
        }?;
        let mut ranges = vec![E384RangedMeasurement::default(); ranges_count];
        let mut idxs = vec![0u16; idx_count];
        if ranges_count > 0 || idx_count > 0 {
            unsafe {
                translate(crate::sys::e384_getVCCurrentRanges_perChannel(
                    self.0,
                    ranges.as_mut_ptr(),
                    &mut ranges_count,
                    idxs.as_mut_ptr(),
                    &mut idx_count,
                ))
            }?;
            ranges.truncate(ranges_count);
            idxs.truncate(idx_count);
        }
        Ok((ranges, idxs))
    }
}
