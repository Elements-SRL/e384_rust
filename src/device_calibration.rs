use crate::device::Device;
use crate::error_codes::ErrorCodes;
use crate::util::{collect_matrix, owned_string_list, translate};

impl Device {
    pub fn write_calibration_eeprom(
        &self,
        value: &[u32],
        address: &[u32],
        size: &[u32],
    ) -> Result<(), ErrorCodes> {
        assert_eq!(value.len(), address.len());
        assert_eq!(value.len(), size.len());
        unsafe {
            translate(crate::sys::e384_writeCalibrationEeprom(
                self.0,
                value.as_ptr(),
                address.as_ptr(),
                size.as_ptr(),
                value.len(),
            ))
        }
    }

    pub fn read_calibration_eeprom(
        &self,
        address: &[u32],
        size: &[u32],
    ) -> Result<Vec<u32>, ErrorCodes> {
        assert_eq!(address.len(), size.len());
        let mut out = vec![0u32; address.len()];
        unsafe {
            translate(crate::sys::e384_readCalibrationEeprom(
                self.0,
                address.as_ptr(),
                size.as_ptr(),
                address.len(),
                out.as_mut_ptr(),
            ))
        }?;
        Ok(out)
    }

    pub fn calib_file_names(&self) -> Result<Vec<String>, ErrorCodes> {
        let mut list = std::ptr::null_mut();
        unsafe { translate(crate::sys::e384_getCalibFileNames(self.0, &mut list)) }?;
        Ok(unsafe { owned_string_list(list) })
    }

    /// Row-major flags matrix, one row per calibration file.
    pub fn calib_files_flags(&self) -> Result<(Vec<u8>, usize, usize), ErrorCodes> {
        let dev = self.0;
        unsafe {
            collect_matrix(|out, rows, cols| crate::sys::e384_getCalibFilesFlags(dev, out, rows, cols))
        }
    }

    pub fn ok_move_calibration_eeprom_to_rams(&self) -> Result<(), ErrorCodes> {
        unsafe { translate(crate::sys::e384_okMoveCalibrationEepromToRams(self.0)) }
    }

    pub fn ok_move_calibration_rams_to_eeprom(&self) -> Result<(), ErrorCodes> {
        unsafe { translate(crate::sys::e384_okMoveCalibrationRamsToEeprom(self.0)) }
    }

    pub fn ok_select_calibration_ram(&self, ram_idx: u16) -> Result<(), ErrorCodes> {
        unsafe { translate(crate::sys::e384_okSelectCalibrationRam(self.0, ram_idx)) }
    }

    pub fn ok_write_calibration_ram(&self, address: u16, value: u8) -> Result<(), ErrorCodes> {
        unsafe { translate(crate::sys::e384_okWriteCalibrationRam(self.0, address, value)) }
    }

    pub fn ok_read_calibration_ram(&self) -> Result<(), ErrorCodes> {
        unsafe { translate(crate::sys::e384_okReadCalibrationRam(self.0)) }
    }
}
