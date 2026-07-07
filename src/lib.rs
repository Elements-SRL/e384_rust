#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod sys;
pub mod device;
pub mod device_channel_cmd;
pub mod device_ranges;
pub mod device_acquisition;
pub mod device_calibration;
pub mod device_compensation;
pub mod device_protocol;
pub mod device_custom;
pub mod device_convert;
pub mod device_info;
pub mod channel_model;
pub mod board_model;
pub mod error_codes;
pub mod util;


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
