use crate::device::Device;
use crate::error_codes::ErrorCodes;
use crate::sys::{E384Measurement, E384RangedMeasurement};
use crate::util::{collect_list, collect_list_with_default_idx, translate};

/// `(device, uint16_t value, int32_t applyFlag) -> E384Err`
macro_rules! scalar_setter {
    ($name:ident, $sys_fn:path) => {
        pub fn $name(&self, value: u16, apply: bool) -> Result<(), ErrorCodes> {
            unsafe { translate($sys_fn(self.0, value, apply as i32)) }
        }
    };
}

/// `(device, uint32_t* out) -> E384Err`
macro_rules! u32_getter {
    ($name:ident, $sys_fn:path) => {
        pub fn $name(&self) -> Result<u32, ErrorCodes> {
            let mut out: u32 = 0;
            unsafe { translate($sys_fn(self.0, &mut out)) }?;
            Ok(out)
        }
    };
}

/// `(device, E384Measurement* out) -> E384Err`
macro_rules! measurement_getter {
    ($name:ident, $sys_fn:path) => {
        pub fn $name(&self) -> Result<E384Measurement, ErrorCodes> {
            let mut out = E384Measurement::default();
            unsafe { translate($sys_fn(self.0, &mut out)) }?;
            Ok(out)
        }
    };
}

/// Two-call `(device, E384Measurement* out, size_t* count) -> E384Err`
macro_rules! measurement_list_getter {
    ($name:ident, $sys_fn:path) => {
        pub fn $name(&self) -> Result<Vec<E384Measurement>, ErrorCodes> {
            let dev = self.0;
            unsafe { collect_list(|out, count| $sys_fn(dev, out, count)) }
        }
    };
}

/// Two-call `(device, E384RangedMeasurement* out, size_t* count) -> E384Err`, no default idx.
macro_rules! ranged_list_getter {
    ($name:ident, $sys_fn:path) => {
        pub fn $name(&self) -> Result<Vec<E384RangedMeasurement>, ErrorCodes> {
            let dev = self.0;
            unsafe { collect_list(|out, count| $sys_fn(dev, out, count)) }
        }
    };
}

/// Two-call `(device, E384RangedMeasurement* out, size_t* count, uint16_t* outDefaultIdx) -> E384Err`.
macro_rules! ranged_list_with_default_idx_getter {
    ($name:ident, $sys_fn:path) => {
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
    ($name:ident, $sys_fn:path) => {
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
    pub fn vc_current_range(&self) -> Result<E384RangedMeasurement, ErrorCodes> {
        let mut out = E384RangedMeasurement::default();
        unsafe { translate(crate::sys::e384_getVCCurrentRange(self.0, &mut out)) }?;
        Ok(out)
    }

    ranged_list_with_default_idx_getter!(vc_current_ranges, crate::sys::e384_getVCCurrentRanges);
    ranged_list_with_default_idx_getter!(vc_voltage_ranges, crate::sys::e384_getVCVoltageRanges);
    ranged_list_with_default_idx_getter!(cc_current_ranges, crate::sys::e384_getCCCurrentRanges);
    ranged_list_with_default_idx_getter!(cc_voltage_ranges, crate::sys::e384_getCCVoltageRanges);

    ranged_with_idx_getter!(max_vc_current_range, crate::sys::e384_getMaxVCCurrentRange);
    ranged_with_idx_getter!(min_vc_current_range, crate::sys::e384_getMinVCCurrentRange);
    ranged_with_idx_getter!(max_vc_voltage_range, crate::sys::e384_getMaxVCVoltageRange);
    ranged_with_idx_getter!(min_vc_voltage_range, crate::sys::e384_getMinVCVoltageRange);
    ranged_with_idx_getter!(max_cc_current_range, crate::sys::e384_getMaxCCCurrentRange);
    ranged_with_idx_getter!(min_cc_current_range, crate::sys::e384_getMinCCCurrentRange);
    ranged_with_idx_getter!(max_cc_voltage_range, crate::sys::e384_getMaxCCVoltageRange);
    ranged_with_idx_getter!(min_cc_voltage_range, crate::sys::e384_getMinCCVoltageRange);

    scalar_setter!(set_vc_current_range_all, crate::sys::e384_setVCCurrentRange_all);
    scalar_setter!(set_vc_voltage_range, crate::sys::e384_setVCVoltageRange);
    scalar_setter!(set_cc_current_range, crate::sys::e384_setCCCurrentRange);
    scalar_setter!(set_cc_voltage_range_all, crate::sys::e384_setCCVoltageRange_all);
    scalar_setter!(set_voltage_stimulus_lpf, crate::sys::e384_setVoltageStimulusLpf);
    scalar_setter!(set_current_stimulus_lpf, crate::sys::e384_setCurrentStimulusLpf);
    scalar_setter!(
        set_source_for_voltage_channel,
        crate::sys::e384_setSourceForVoltageChannel
    );
    scalar_setter!(
        set_source_for_current_channel,
        crate::sys::e384_setSourceForCurrentChannel
    );
    scalar_setter!(set_sampling_rate, crate::sys::e384_setSamplingRate);

    // -- index/scalar getters -----------------------------------------------------
    u32_getter!(clamping_modality_idx, crate::sys::e384_getClampingModalityIdx);
    u32_getter!(vc_voltage_range_idx, crate::sys::e384_getVCVoltageRangeIdx);
    u32_getter!(cc_current_range_idx, crate::sys::e384_getCCCurrentRangeIdx);
    u32_getter!(sampling_rate_idx, crate::sys::e384_getSamplingRateIdx);
    u32_getter!(
        max_downsampling_ratio_feature,
        crate::sys::e384_getMaxDownsamplingRatioFeature
    );
    u32_getter!(downsampling_ratio, crate::sys::e384_getDownsamplingRatio);
    u32_getter!(vc_voltage_filter_idx, crate::sys::e384_getVCVoltageFilterIdx);
    u32_getter!(vc_current_filter_idx, crate::sys::e384_getVCCurrentFilterIdx);
    u32_getter!(cc_voltage_filter_idx, crate::sys::e384_getCCVoltageFilterIdx);
    u32_getter!(cc_current_filter_idx, crate::sys::e384_getCCCurrentFilterIdx);
    u32_getter!(
        max_protocol_items_feature,
        crate::sys::e384_getMaxProtocolItemsFeature
    );
    u32_getter!(
        calibration_eeprom_size,
        crate::sys::e384_getCalibrationEepromSize
    );

    // -- measurement getters --------------------------------------------------------
    measurement_getter!(sampling_rate, crate::sys::e384_getSamplingRate);
    measurement_getter!(vc_voltage_filter, crate::sys::e384_getVCVoltageFilter);
    measurement_getter!(vc_current_filter, crate::sys::e384_getVCCurrentFilter);
    measurement_getter!(cc_voltage_filter, crate::sys::e384_getCCVoltageFilter);
    measurement_getter!(cc_current_filter, crate::sys::e384_getCCCurrentFilter);

    // -- measurement list getters -----------------------------------------------------
    measurement_list_getter!(
        sampling_rates_features,
        crate::sys::e384_getSamplingRatesFeatures
    );
    measurement_list_getter!(
        real_sampling_rates_features,
        crate::sys::e384_getRealSamplingRatesFeatures
    );
    measurement_list_getter!(vc_voltage_filters, crate::sys::e384_getVCVoltageFilters);
    measurement_list_getter!(vc_current_filters, crate::sys::e384_getVCCurrentFilters);
    measurement_list_getter!(cc_voltage_filters, crate::sys::e384_getCCVoltageFilters);
    measurement_list_getter!(cc_current_filters, crate::sys::e384_getCCCurrentFilters);
    measurement_list_getter!(voltage_hold_tuner, crate::sys::e384_getVoltageHoldTuner);

    // -- ranged list getters (no default idx) ------------------------------------------
    ranged_list_getter!(
        voltage_hold_tuner_features,
        crate::sys::e384_getVoltageHoldTunerFeatures
    );
    ranged_list_getter!(voltage_half_features, crate::sys::e384_getVoltageHalfFeatures);
    ranged_list_getter!(
        current_hold_tuner_features,
        crate::sys::e384_getCurrentHoldTunerFeatures
    );
    ranged_list_getter!(current_half_features, crate::sys::e384_getCurrentHalfFeatures);
    ranged_list_getter!(
        liquid_junction_ranges_features,
        crate::sys::e384_getLiquidJunctionRangesFeatures
    );

    // -- disambiguated overloads --------------------------------------------------------
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

    ranged_list_getter!(vc_current_range_list, crate::sys::e384_getVCCurrentRange_list);

    pub fn cc_voltage_range(&self) -> Result<E384RangedMeasurement, ErrorCodes> {
        let mut out = E384RangedMeasurement::default();
        unsafe { translate(crate::sys::e384_getCCVoltageRange(self.0, &mut out)) }?;
        Ok(out)
    }
    ranged_list_getter!(cc_voltage_range_list, crate::sys::e384_getCCVoltageRange_list);

    pub fn voltage_range(&self) -> Result<E384RangedMeasurement, ErrorCodes> {
        let mut out = E384RangedMeasurement::default();
        unsafe { translate(crate::sys::e384_getVoltageRange(self.0, &mut out)) }?;
        Ok(out)
    }
    ranged_list_getter!(voltage_range_list, crate::sys::e384_getVoltageRange_list);

    pub fn current_range(&self) -> Result<E384RangedMeasurement, ErrorCodes> {
        let mut out = E384RangedMeasurement::default();
        unsafe { translate(crate::sys::e384_getCurrentRange(self.0, &mut out)) }?;
        Ok(out)
    }
    ranged_list_getter!(current_range_list, crate::sys::e384_getCurrentRange_list);

    /// Two-call `(device, uint32_t* out, size_t* count) -> E384Err`
    pub fn vc_current_range_idx_list(&self) -> Result<Vec<u32>, ErrorCodes> {
        let dev = self.0;
        unsafe { collect_list(|out, count| crate::sys::e384_getVCCurrentRangeIdx_list(dev, out, count)) }
    }

    pub fn cc_voltage_range_idx_list(&self) -> Result<Vec<u32>, ErrorCodes> {
        let dev = self.0;
        unsafe { collect_list(|out, count| crate::sys::e384_getCCVoltageRangeIdx_list(dev, out, count)) }
    }

    /// Per-channel VC current ranges with their default index per channel.
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
