//! `ErrorCodes` mirrors `e384commlib_errorcodes.h` group-by-group. Variant doc comments are the
//! original C++ library's descriptions; the trailing `+ 0x...` values in [`From<E384Err>`] must
//! match the header's `group + offset` constants exactly, not be recomputed.

use crate::sys::E384Err;

const ERROR_GROUP_DEVICE_DETECTION: u32 = 0x00010000;
const ERROR_GROUP_EEPROM_COMMUNICATION: u32 = 0x00020000;
const ERROR_GROUP_DEVICE_CONNECTION: u32 = 0x00030000;
const ERROR_GROUP_DEVICE_COMMANDS: u32 = 0x00040000;
const ERROR_GROUP_DEVICE_FEATURES: u32 = 0x00050000;
const ERROR_GROUP_DEVICE_ISSUES: u32 = 0x00060000;
const ERROR_GROUP_LIBRARY_ISSUES: u32 = 0x00070000;
const ERROR_GROUP_CALIBRATION: u32 = 0x00080000;

/// Error codes returned by `e384CommLib`, translated from the raw `E384Err` group+offset values.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ErrorCodes {
    /// Method returns with no errors.
    Success,

    // ErrorGroupDeviceDetection
    /// Returned when no devices are detected.
    ErrorNoDeviceFound,
    /// Returned when device detection fails, e.g. due to USB driver installation issues.
    ErrorListDeviceFailed,
    /// Returned when the required device for connection is not found.
    ErrorDeviceNotFound,

    // ErrorGroupEepromCommunication
    /// Returned when trying to connect to a device eeprom which is already connected.
    ErrorEepromAlreadyConnected,
    /// Returned when connection to a device eeprom fails.
    ErrorEepromConnectionFailed,
    /// Returned when disconnection from a device eeprom fails.
    ErrorEepromDisconnectionFailed,
    /// Returned when trying to communicate with a device eeprom if none is connected.
    ErrorEepromNotConnected,
    /// Returned when reading from a device eeprom fails.
    ErrorEepromReadFailed,
    /// Returned when writing to a device eeprom fails.
    ErrorEepromWriteFailed,
    /// Returned when the eeprom is not recognized.
    ErrorEepromNotRecognized,
    /// Returned when trying to read or write an invalid eeprom address.
    ErrorEepromInvalidAddress,

    // ErrorGroupDeviceConnection
    /// Returned when the device type is not recognized.
    ErrorDeviceTypeNotRecognized,
    /// Returned when trying to connect to a device which is already connected.
    ErrorDeviceAlreadyConnected,
    /// Returned when trying to communicate with a device if none is connected.
    ErrorDeviceNotConnected,
    /// Returned when connection to a device fails.
    ErrorDeviceConnectionFailed,
    /// Returned when FTDI communication channel configuration fails.
    ErrorFtdiConfigurationFailed,
    /// Returned when failed to ping the device during connection.
    ErrorConnectionPingFailed,
    /// Returned when failed to reset the FPGA during connection.
    ErrorConnectionFpgaResetFailed,
    /// Returned when failed to reset the chip during connection.
    ErrorConnectionChipResetFailed,
    /// Returned when disconnection from a device fails.
    ErrorDeviceDisconnectionFailed,
    /// Returned when the FPGA FW is not correctly loaded.
    ErrorDeviceFwLoadingFailed,
    /// Returned when the FPGA FW is old and must be upgraded.
    ErrorDeviceToBeUpgraded,
    /// Returned when the FPGA FW cannot be upgraded.
    ErrorDeviceNotUpgradable,
    /// Returned when the FW cannot be found.
    ErrorFwNotFound,
    /// Returned when something goes wrong with the FW upgrade procedure.
    ErrorFwUpgradeFailed,
    /// Returned when the device is not connected to a Super Speed USB 3.
    ErrorNotConnectedToUsb3,

    // ErrorGroupDeviceCommands
    /// Returned when sending a message to a device fails.
    ErrorSendMessageFailed,
    /// Returned when trying to use a command that is not implemented for the current device.
    ErrorCommandNotImplemented,
    /// Returned when trying to set a value out of range for the current device (e.g. a current
    /// range index bigger than the total number of current ranges available).
    ErrorValueOutOfRange,
    /// Returned when trying to set a value that has already been set (e.g. an identical
    /// compensation value).
    ErrorUnchangedValue,
    /// Returned when a protocol loop's closing item does not follow the allowed item ordering.
    ErrorBadlyFormedProtocolLoop,
    /// Returned when trying to read data but there's no data available.
    ErrorNoDataAvailable,
    /// Returned when reading copies of the same data header.
    ErrorRepeatedHeader,
    /// Returned when reading copies of the same protocol tail.
    ErrorRepeatedTail,
    /// Returned when reading an ill-formed message, e.g. a data acquisition packet without a
    /// first sample index.
    ErrorIllFormedMessage,
    /// Returned when a command is used in the wrong clamp modality (e.g. setting holding voltage
    /// in current clamp).
    ErrorWrongClampModality,
    /// Not a failure: the command was still applied but the value was clipped to range (e.g. a
    /// voltage offset bigger than the DAC range). [`ErrorCodes::to_res`] treats this as `Ok(())`.
    WarningValueClipped,
    /// Returned when trying to set a compensation that is not enabled.
    ErrorCompensationNotEnabled,
    /// Returned when trying to activate digital offset compensation and readout offset
    /// recalibration together.
    ErrorLiquidJunctionAndRecalibration,
    /// Returned when an input vector does not have the right size (e.g. channels and
    /// measurements vectors of mismatched length).
    ErrorWrongInputLength,

    // ErrorGroupDeviceFeatures
    /// Returned when trying to use a feature not implemented for the current device.
    ErrorFeatureNotImplemented,
    /// Returned when there are no upgrades available for the current device.
    ErrorUpgradesNotAvailable,

    // ErrorGroupDeviceIssues
    /// Returned when the device has expired.
    ErrorExpiredDevice,

    // ErrorGroupLibraryIssues
    /// Returned when the library fails to initialize memory.
    ErrorMemoryInitialization,

    // ErrorGroupCalibration
    /// Returned when the calibration folder is missing.
    ErrorCalibrationDirMissing,
    /// Returned when the calibration mapping file cannot be opened.
    ErrorCalibrationMappingNotOpened,
    /// Returned when the calibration mapping file is corrupted.
    ErrorCalibrationMappingCorrupted,
    /// Returned when a calibration file is corrupted.
    ErrorCalibrationFileCorrupted,
    /// Returned when a calibration file is missing.
    ErrorCalibrationFileMissing,
    /// Returned when there's a bug in the calibration loading procedure.
    ErrorCalibrationSoftwareBug,
    /// Returned when calibration was not loaded yet.
    ErrorCalibrationNotLoadedYet,
    /// Returned when board numbering in the mapping file is wrong.
    ErrorCalibrationMappingWrongNumbering,

    /// Undefined error — the raw code didn't match any known group+offset constant.
    ErrorUnknown,
}

impl From<E384Err> for ErrorCodes {
    fn from(value: E384Err) -> Self {
        match value {
            0x00000000 => ErrorCodes::Success,

            v if v == ERROR_GROUP_DEVICE_DETECTION + 0x00000001 => ErrorCodes::ErrorNoDeviceFound,
            v if v == ERROR_GROUP_DEVICE_DETECTION + 0x00000002 => {
                ErrorCodes::ErrorListDeviceFailed
            }
            v if v == ERROR_GROUP_DEVICE_DETECTION + 0x00000003 => ErrorCodes::ErrorDeviceNotFound,

            v if v == ERROR_GROUP_EEPROM_COMMUNICATION + 0x00000001 => {
                ErrorCodes::ErrorEepromAlreadyConnected
            }
            v if v == ERROR_GROUP_EEPROM_COMMUNICATION + 0x00000002 => {
                ErrorCodes::ErrorEepromConnectionFailed
            }
            v if v == ERROR_GROUP_EEPROM_COMMUNICATION + 0x00000003 => {
                ErrorCodes::ErrorEepromDisconnectionFailed
            }
            v if v == ERROR_GROUP_EEPROM_COMMUNICATION + 0x00000004 => {
                ErrorCodes::ErrorEepromNotConnected
            }
            v if v == ERROR_GROUP_EEPROM_COMMUNICATION + 0x00000005 => {
                ErrorCodes::ErrorEepromReadFailed
            }
            v if v == ERROR_GROUP_EEPROM_COMMUNICATION + 0x00000006 => {
                ErrorCodes::ErrorEepromWriteFailed
            }
            v if v == ERROR_GROUP_EEPROM_COMMUNICATION + 0x00000007 => {
                ErrorCodes::ErrorEepromNotRecognized
            }
            v if v == ERROR_GROUP_EEPROM_COMMUNICATION + 0x00000008 => {
                ErrorCodes::ErrorEepromInvalidAddress
            }

            v if v == ERROR_GROUP_DEVICE_CONNECTION + 0x00000001 => {
                ErrorCodes::ErrorDeviceTypeNotRecognized
            }
            v if v == ERROR_GROUP_DEVICE_CONNECTION + 0x00000002 => {
                ErrorCodes::ErrorDeviceAlreadyConnected
            }
            v if v == ERROR_GROUP_DEVICE_CONNECTION + 0x00000003 => {
                ErrorCodes::ErrorDeviceNotConnected
            }
            v if v == ERROR_GROUP_DEVICE_CONNECTION + 0x00000004 => {
                ErrorCodes::ErrorDeviceConnectionFailed
            }
            v if v == ERROR_GROUP_DEVICE_CONNECTION + 0x00000005 => {
                ErrorCodes::ErrorFtdiConfigurationFailed
            }
            v if v == ERROR_GROUP_DEVICE_CONNECTION + 0x00000006 => {
                ErrorCodes::ErrorConnectionPingFailed
            }
            v if v == ERROR_GROUP_DEVICE_CONNECTION + 0x00000007 => {
                ErrorCodes::ErrorConnectionFpgaResetFailed
            }
            v if v == ERROR_GROUP_DEVICE_CONNECTION + 0x00000008 => {
                ErrorCodes::ErrorConnectionChipResetFailed
            }
            v if v == ERROR_GROUP_DEVICE_CONNECTION + 0x00000009 => {
                ErrorCodes::ErrorDeviceDisconnectionFailed
            }
            v if v == ERROR_GROUP_DEVICE_CONNECTION + 0x0000000A => {
                ErrorCodes::ErrorDeviceFwLoadingFailed
            }
            v if v == ERROR_GROUP_DEVICE_CONNECTION + 0x0000000B => {
                ErrorCodes::ErrorDeviceToBeUpgraded
            }
            v if v == ERROR_GROUP_DEVICE_CONNECTION + 0x0000000C => {
                ErrorCodes::ErrorDeviceNotUpgradable
            }
            v if v == ERROR_GROUP_DEVICE_CONNECTION + 0x0000000D => ErrorCodes::ErrorFwNotFound,
            v if v == ERROR_GROUP_DEVICE_CONNECTION + 0x0000000E => {
                ErrorCodes::ErrorFwUpgradeFailed
            }
            v if v == ERROR_GROUP_DEVICE_CONNECTION + 0x0000000F => {
                ErrorCodes::ErrorNotConnectedToUsb3
            }

            v if v == ERROR_GROUP_DEVICE_COMMANDS + 0x00000001 => {
                ErrorCodes::ErrorSendMessageFailed
            }
            v if v == ERROR_GROUP_DEVICE_COMMANDS + 0x00000002 => {
                ErrorCodes::ErrorCommandNotImplemented
            }
            v if v == ERROR_GROUP_DEVICE_COMMANDS + 0x00000003 => ErrorCodes::ErrorValueOutOfRange,
            v if v == ERROR_GROUP_DEVICE_COMMANDS + 0x00000004 => ErrorCodes::ErrorUnchangedValue,
            v if v == ERROR_GROUP_DEVICE_COMMANDS + 0x00000005 => {
                ErrorCodes::ErrorBadlyFormedProtocolLoop
            }
            v if v == ERROR_GROUP_DEVICE_COMMANDS + 0x00000006 => ErrorCodes::ErrorNoDataAvailable,
            v if v == ERROR_GROUP_DEVICE_COMMANDS + 0x00000007 => ErrorCodes::ErrorRepeatedHeader,
            v if v == ERROR_GROUP_DEVICE_COMMANDS + 0x00000008 => ErrorCodes::ErrorRepeatedTail,
            v if v == ERROR_GROUP_DEVICE_COMMANDS + 0x00000009 => ErrorCodes::ErrorIllFormedMessage,
            v if v == ERROR_GROUP_DEVICE_COMMANDS + 0x0000000A => {
                ErrorCodes::ErrorWrongClampModality
            }
            v if v == ERROR_GROUP_DEVICE_COMMANDS + 0x8000000B => ErrorCodes::WarningValueClipped,
            v if v == ERROR_GROUP_DEVICE_COMMANDS + 0x0000000C => {
                ErrorCodes::ErrorCompensationNotEnabled
            }
            v if v == ERROR_GROUP_DEVICE_COMMANDS + 0x0000000D => {
                ErrorCodes::ErrorLiquidJunctionAndRecalibration
            }
            v if v == ERROR_GROUP_DEVICE_COMMANDS + 0x0000000E => ErrorCodes::ErrorWrongInputLength,

            ERROR_GROUP_DEVICE_FEATURES => ErrorCodes::ErrorFeatureNotImplemented,
            v if v == ERROR_GROUP_DEVICE_FEATURES + 0x00000001 => {
                ErrorCodes::ErrorUpgradesNotAvailable
            }

            ERROR_GROUP_DEVICE_ISSUES => ErrorCodes::ErrorExpiredDevice,

            ERROR_GROUP_LIBRARY_ISSUES => ErrorCodes::ErrorMemoryInitialization,

            ERROR_GROUP_CALIBRATION => ErrorCodes::ErrorCalibrationDirMissing,
            v if v == ERROR_GROUP_CALIBRATION + 0x00000001 => {
                ErrorCodes::ErrorCalibrationMappingNotOpened
            }
            v if v == ERROR_GROUP_CALIBRATION + 0x00000002 => {
                ErrorCodes::ErrorCalibrationMappingCorrupted
            }
            v if v == ERROR_GROUP_CALIBRATION + 0x00000003 => {
                ErrorCodes::ErrorCalibrationFileCorrupted
            }
            v if v == ERROR_GROUP_CALIBRATION + 0x00000004 => {
                ErrorCodes::ErrorCalibrationFileMissing
            }
            v if v == ERROR_GROUP_CALIBRATION + 0x00000005 => {
                ErrorCodes::ErrorCalibrationSoftwareBug
            }
            v if v == ERROR_GROUP_CALIBRATION + 0x00000006 => {
                ErrorCodes::ErrorCalibrationNotLoadedYet
            }
            v if v == ERROR_GROUP_CALIBRATION + 0x00000007 => {
                ErrorCodes::ErrorCalibrationMappingWrongNumbering
            }

            _ => ErrorCodes::ErrorUnknown,
        }
    }
}

impl ErrorCodes {
    /// Converts to a `Result`, treating [`ErrorCodes::Success`] and
    /// [`ErrorCodes::WarningValueClipped`] as `Ok(())` (the latter logs a warning since the
    /// command was still applied, just clipped) and every other variant as `Err`.
    pub fn to_res(self) -> Result<(), Self> {
        match self {
            ErrorCodes::Success => Ok(()),
            ErrorCodes::WarningValueClipped => {
                tracing::warn!("value clipped by device");
                Ok(())
            }
            v => Err(v),
        }
    }
}
