use crate::r_message_dispatcher::E384Err;

const ERROR_GROUP_DEVICE_DETECTION: u32 =       0x00010000; /**!< Error codes related to device detection errors. */
const ERROR_GROUP_EEPROM_COMMUNICATION: u32 =   0x00020000; /**!< Error codes related to eeprom communication errors. */
const ERROR_GROUP_DEVICE_CONNECTION: u32 =      0x00030000; /**!< Error codes related to connection errors. */
const ERROR_GROUP_DEVICE_COMMANDS: u32 =        0x00040000; /**!< Error codes related to failed commands to the device. */
const ERROR_GROUP_DEVICE_FEATURES: u32 =        0x00050000; /**!< Error codes related to wrongly used features. */
const ERROR_GROUP_DEVICE_ISSUES: u32 =          0x00060000; /**!< Error codes related to issues with the device. */
const ERROR_GROUP_LIBRARY_ISSUES: u32 =         0x00070000; /**!< Error codes related to issues with the library. */
const ERROR_GROUP_CALIBRATION: u32 =            0x00080000;  /**!< Error codes related to issues with calibration parameters. */


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ErrorCodes {
    Success,
    ErrorNoDeviceFound,
    ErrorListDeviceFailed,
    ErrorDeviceNotFound,
    Unknown,
}

impl From<E384Err> for ErrorCodes {
    fn from(value: E384Err) -> Self {
        match value {
            0 => ErrorCodes::Success,
             val if val == ERROR_GROUP_DEVICE_DETECTION + 0x00000001 => ErrorCodes::ErrorNoDeviceFound,
             val if val == ERROR_GROUP_DEVICE_DETECTION + 0x00000002 => ErrorCodes::ErrorListDeviceFailed,
             val if val == ERROR_GROUP_DEVICE_DETECTION + 0x00000003 => ErrorCodes::ErrorDeviceNotFound,
            _ => ErrorCodes::Unknown,
        }
    }
}

impl ErrorCodes {
    pub fn to_res(&self) -> Result<(), Self> {
        match self {
            ErrorCodes::Success => Ok(()),
            v => Err(*v),
        }
    }
}