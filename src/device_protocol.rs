use crate::device::Device;
use crate::error_codes::ErrorCodes;
use crate::sys::{E384Measurement, E384RangedMeasurement};
use crate::util::{collect_list, translate};

impl Device {
    pub fn set_voltage_protocol_structure(
        &self,
        prot_id: u16,
        items_num: u16,
        sweeps_num: u16,
        v_rest: E384Measurement,
        stop_protocol: bool,
    ) -> Result<(), ErrorCodes> {
        unsafe {
            translate(crate::sys::e384_setVoltageProtocolStructure(
                self.0,
                prot_id,
                items_num,
                sweeps_num,
                v_rest,
                stop_protocol as i32,
            ))
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn set_voltage_protocol_step(
        &self,
        item_idx: u16,
        next_item_idx: u16,
        loop_reps: u16,
        apply_steps: bool,
        v0: E384Measurement,
        v0_step: E384Measurement,
        t0: E384Measurement,
        t0_step: E384Measurement,
        v_half: bool,
        active_digital_outputs: &[u16],
    ) -> Result<(), ErrorCodes> {
        unsafe {
            translate(crate::sys::e384_setVoltageProtocolStep(
                self.0,
                item_idx,
                next_item_idx,
                loop_reps,
                apply_steps as i32,
                v0,
                v0_step,
                t0,
                t0_step,
                v_half as i32,
                active_digital_outputs.as_ptr(),
                active_digital_outputs.len(),
            ))
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn set_voltage_protocol_ramp(
        &self,
        item_idx: u16,
        next_item_idx: u16,
        loop_reps: u16,
        apply_steps: bool,
        v0: E384Measurement,
        v0_step: E384Measurement,
        v_final: E384Measurement,
        v_final_step: E384Measurement,
        t0: E384Measurement,
        t0_step: E384Measurement,
        v_half: bool,
        active_digital_outputs: &[u16],
    ) -> Result<(), ErrorCodes> {
        unsafe {
            translate(crate::sys::e384_setVoltageProtocolRamp(
                self.0,
                item_idx,
                next_item_idx,
                loop_reps,
                apply_steps as i32,
                v0,
                v0_step,
                v_final,
                v_final_step,
                t0,
                t0_step,
                v_half as i32,
                active_digital_outputs.as_ptr(),
                active_digital_outputs.len(),
            ))
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn set_voltage_protocol_sin(
        &self,
        item_idx: u16,
        next_item_idx: u16,
        loop_reps: u16,
        apply_steps: bool,
        v0: E384Measurement,
        v0_step: E384Measurement,
        v_amp: E384Measurement,
        v_amp_step: E384Measurement,
        f0: E384Measurement,
        f0_step: E384Measurement,
        v_half: bool,
        active_digital_outputs: &[u16],
    ) -> Result<(), ErrorCodes> {
        unsafe {
            translate(crate::sys::e384_setVoltageProtocolSin(
                self.0,
                item_idx,
                next_item_idx,
                loop_reps,
                apply_steps as i32,
                v0,
                v0_step,
                v_amp,
                v_amp_step,
                f0,
                f0_step,
                v_half as i32,
                active_digital_outputs.as_ptr(),
                active_digital_outputs.len(),
            ))
        }
    }

    pub fn set_current_protocol_structure(
        &self,
        prot_id: u16,
        items_num: u16,
        sweeps_num: u16,
        i_rest: E384Measurement,
        stop_protocol: bool,
    ) -> Result<(), ErrorCodes> {
        unsafe {
            translate(crate::sys::e384_setCurrentProtocolStructure(
                self.0,
                prot_id,
                items_num,
                sweeps_num,
                i_rest,
                stop_protocol as i32,
            ))
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn set_current_protocol_step(
        &self,
        item_idx: u16,
        next_item_idx: u16,
        loop_reps: u16,
        apply_steps: bool,
        i0: E384Measurement,
        i0_step: E384Measurement,
        t0: E384Measurement,
        t0_step: E384Measurement,
        c_half: bool,
        active_digital_outputs: &[u16],
    ) -> Result<(), ErrorCodes> {
        unsafe {
            translate(crate::sys::e384_setCurrentProtocolStep(
                self.0,
                item_idx,
                next_item_idx,
                loop_reps,
                apply_steps as i32,
                i0,
                i0_step,
                t0,
                t0_step,
                c_half as i32,
                active_digital_outputs.as_ptr(),
                active_digital_outputs.len(),
            ))
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn set_current_protocol_ramp(
        &self,
        item_idx: u16,
        next_item_idx: u16,
        loop_reps: u16,
        apply_steps: bool,
        i0: E384Measurement,
        i0_step: E384Measurement,
        i_final: E384Measurement,
        i_final_step: E384Measurement,
        t0: E384Measurement,
        t0_step: E384Measurement,
        c_half: bool,
        active_digital_outputs: &[u16],
    ) -> Result<(), ErrorCodes> {
        unsafe {
            translate(crate::sys::e384_setCurrentProtocolRamp(
                self.0,
                item_idx,
                next_item_idx,
                loop_reps,
                apply_steps as i32,
                i0,
                i0_step,
                i_final,
                i_final_step,
                t0,
                t0_step,
                c_half as i32,
                active_digital_outputs.as_ptr(),
                active_digital_outputs.len(),
            ))
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn set_current_protocol_sin(
        &self,
        item_idx: u16,
        next_item_idx: u16,
        loop_reps: u16,
        apply_steps: bool,
        i0: E384Measurement,
        i0_step: E384Measurement,
        i_amp: E384Measurement,
        i_amp_step: E384Measurement,
        f0: E384Measurement,
        f0_step: E384Measurement,
        c_half: bool,
        active_digital_outputs: &[u16],
    ) -> Result<(), ErrorCodes> {
        unsafe {
            translate(crate::sys::e384_setCurrentProtocolSin(
                self.0,
                item_idx,
                next_item_idx,
                loop_reps,
                apply_steps as i32,
                i0,
                i0_step,
                i_amp,
                i_amp_step,
                f0,
                f0_step,
                c_half as i32,
                active_digital_outputs.as_ptr(),
                active_digital_outputs.len(),
            ))
        }
    }

    pub fn voltage_protocol_range_feature(
        &self,
        range_idx: u16,
    ) -> Result<E384RangedMeasurement, ErrorCodes> {
        let mut out = E384RangedMeasurement::default();
        unsafe {
            translate(crate::sys::e384_getVoltageProtocolRangeFeature(
                self.0, range_idx, &mut out,
            ))
        }?;
        Ok(out)
    }

    pub fn current_protocol_range_feature(
        &self,
        range_idx: u16,
    ) -> Result<E384RangedMeasurement, ErrorCodes> {
        let mut out = E384RangedMeasurement::default();
        unsafe {
            translate(crate::sys::e384_getCurrentProtocolRangeFeature(
                self.0, range_idx, &mut out,
            ))
        }?;
        Ok(out)
    }

    pub fn voltage_ramp_tuner_features(
        &self,
    ) -> Result<(Vec<E384RangedMeasurement>, E384RangedMeasurement), ErrorCodes> {
        let mut duration_range = E384RangedMeasurement::default();
        let dev = self.0;
        let voltage_ranges = unsafe {
            collect_list(|out, count| {
                crate::sys::e384_getVoltageRampTunerFeatures(dev, out, count, &mut duration_range)
            })
        }?;
        Ok((voltage_ranges, duration_range))
    }
}
