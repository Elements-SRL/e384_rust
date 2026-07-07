use crate::sys::E384Err;

const ERROR_GROUP_DEVICE_DETECTION: u32 = 0x00010000;
const ERROR_GROUP_EEPROM_COMMUNICATION: u32 = 0x00020000;
const ERROR_GROUP_DEVICE_CONNECTION: u32 = 0x00030000;
const ERROR_GROUP_DEVICE_COMMANDS: u32 = 0x00040000;
const ERROR_GROUP_DEVICE_FEATURES: u32 = 0x00050000;
const ERROR_GROUP_DEVICE_ISSUES: u32 = 0x00060000;
const ERROR_GROUP_LIBRARY_ISSUES: u32 = 0x00070000;
const ERROR_GROUP_CALIBRATION: u32 = 0x00080000;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ErrorCodes {
    Success,

    // ErrorGroupDeviceDetection
    ErrorNoDeviceFound,
    ErrorListDeviceFailed,
    ErrorDeviceNotFound,

    // ErrorGroupEepromCommunication
    ErrorEepromAlreadyConnected,
    ErrorEepromConnectionFailed,
    ErrorEepromDisconnectionFailed,
    ErrorEepromNotConnected,
    ErrorEepromReadFailed,
    ErrorEepromWriteFailed,
    ErrorEepromNotRecognized,
    ErrorEepromInvalidAddress,

    // ErrorGroupDeviceConnection
    ErrorDeviceTypeNotRecognized,
    ErrorDeviceAlreadyConnected,
    ErrorDeviceNotConnected,
    ErrorDeviceConnectionFailed,
    ErrorFtdiConfigurationFailed,
    ErrorConnectionPingFailed,
    ErrorConnectionFpgaResetFailed,
    ErrorConnectionChipResetFailed,
    ErrorDeviceDisconnectionFailed,
    ErrorDeviceFwLoadingFailed,
    ErrorDeviceToBeUpgraded,
    ErrorDeviceNotUpgradable,
    ErrorFwNotFound,
    ErrorFwUpgradeFailed,
    ErrorNotConnectedToUsb3,

    // ErrorGroupDeviceCommands
    ErrorSendMessageFailed,
    ErrorCommandNotImplemented,
    ErrorValueOutOfRange,
    ErrorUnchangedValue,
    ErrorBadlyFormedProtocolLoop,
    ErrorNoDataAvailable,
    ErrorRepeatedHeader,
    ErrorRepeatedTail,
    ErrorIllFormedMessage,
    ErrorWrongClampModality,
    WarningValueClipped,
    ErrorCompensationNotEnabled,
    ErrorLiquidJunctionAndRecalibration,
    ErrorWrongInputLength,

    // ErrorGroupDeviceFeatures
    ErrorFeatureNotImplemented,
    ErrorUpgradesNotAvailable,

    // ErrorGroupDeviceIssues
    ErrorExpiredDevice,

    // ErrorGroupLibraryIssues
    ErrorMemoryInitialization,

    // ErrorGroupCalibration
    ErrorCalibrationDirMissing,
    ErrorCalibrationMappingNotOpened,
    ErrorCalibrationMappingCorrupted,
    ErrorCalibrationFileCorrupted,
    ErrorCalibrationFileMissing,
    ErrorCalibrationSoftwareBug,
    ErrorCalibrationNotLoadedYet,
    ErrorCalibrationMappingWrongNumbering,

    ErrorUnknown,
}

impl From<E384Err> for ErrorCodes {
    fn from(value: E384Err) -> Self {
        match value {
            0x00000000 => ErrorCodes::Success,

            v if v == ERROR_GROUP_DEVICE_DETECTION + 0x00000001 => ErrorCodes::ErrorNoDeviceFound,
            v if v == ERROR_GROUP_DEVICE_DETECTION + 0x00000002 => ErrorCodes::ErrorListDeviceFailed,
            v if v == ERROR_GROUP_DEVICE_DETECTION + 0x00000003 => ErrorCodes::ErrorDeviceNotFound,

            v if v == ERROR_GROUP_EEPROM_COMMUNICATION + 0x00000001 => ErrorCodes::ErrorEepromAlreadyConnected,
            v if v == ERROR_GROUP_EEPROM_COMMUNICATION + 0x00000002 => ErrorCodes::ErrorEepromConnectionFailed,
            v if v == ERROR_GROUP_EEPROM_COMMUNICATION + 0x00000003 => ErrorCodes::ErrorEepromDisconnectionFailed,
            v if v == ERROR_GROUP_EEPROM_COMMUNICATION + 0x00000004 => ErrorCodes::ErrorEepromNotConnected,
            v if v == ERROR_GROUP_EEPROM_COMMUNICATION + 0x00000005 => ErrorCodes::ErrorEepromReadFailed,
            v if v == ERROR_GROUP_EEPROM_COMMUNICATION + 0x00000006 => ErrorCodes::ErrorEepromWriteFailed,
            v if v == ERROR_GROUP_EEPROM_COMMUNICATION + 0x00000007 => ErrorCodes::ErrorEepromNotRecognized,
            v if v == ERROR_GROUP_EEPROM_COMMUNICATION + 0x00000008 => ErrorCodes::ErrorEepromInvalidAddress,

            v if v == ERROR_GROUP_DEVICE_CONNECTION + 0x00000001 => ErrorCodes::ErrorDeviceTypeNotRecognized,
            v if v == ERROR_GROUP_DEVICE_CONNECTION + 0x00000002 => ErrorCodes::ErrorDeviceAlreadyConnected,
            v if v == ERROR_GROUP_DEVICE_CONNECTION + 0x00000003 => ErrorCodes::ErrorDeviceNotConnected,
            v if v == ERROR_GROUP_DEVICE_CONNECTION + 0x00000004 => ErrorCodes::ErrorDeviceConnectionFailed,
            v if v == ERROR_GROUP_DEVICE_CONNECTION + 0x00000005 => ErrorCodes::ErrorFtdiConfigurationFailed,
            v if v == ERROR_GROUP_DEVICE_CONNECTION + 0x00000006 => ErrorCodes::ErrorConnectionPingFailed,
            v if v == ERROR_GROUP_DEVICE_CONNECTION + 0x00000007 => ErrorCodes::ErrorConnectionFpgaResetFailed,
            v if v == ERROR_GROUP_DEVICE_CONNECTION + 0x00000008 => ErrorCodes::ErrorConnectionChipResetFailed,
            v if v == ERROR_GROUP_DEVICE_CONNECTION + 0x00000009 => ErrorCodes::ErrorDeviceDisconnectionFailed,
            v if v == ERROR_GROUP_DEVICE_CONNECTION + 0x0000000A => ErrorCodes::ErrorDeviceFwLoadingFailed,
            v if v == ERROR_GROUP_DEVICE_CONNECTION + 0x0000000B => ErrorCodes::ErrorDeviceToBeUpgraded,
            v if v == ERROR_GROUP_DEVICE_CONNECTION + 0x0000000C => ErrorCodes::ErrorDeviceNotUpgradable,
            v if v == ERROR_GROUP_DEVICE_CONNECTION + 0x0000000D => ErrorCodes::ErrorFwNotFound,
            v if v == ERROR_GROUP_DEVICE_CONNECTION + 0x0000000E => ErrorCodes::ErrorFwUpgradeFailed,
            v if v == ERROR_GROUP_DEVICE_CONNECTION + 0x0000000F => ErrorCodes::ErrorNotConnectedToUsb3,

            v if v == ERROR_GROUP_DEVICE_COMMANDS + 0x00000001 => ErrorCodes::ErrorSendMessageFailed,
            v if v == ERROR_GROUP_DEVICE_COMMANDS + 0x00000002 => ErrorCodes::ErrorCommandNotImplemented,
            v if v == ERROR_GROUP_DEVICE_COMMANDS + 0x00000003 => ErrorCodes::ErrorValueOutOfRange,
            v if v == ERROR_GROUP_DEVICE_COMMANDS + 0x00000004 => ErrorCodes::ErrorUnchangedValue,
            v if v == ERROR_GROUP_DEVICE_COMMANDS + 0x00000005 => ErrorCodes::ErrorBadlyFormedProtocolLoop,
            v if v == ERROR_GROUP_DEVICE_COMMANDS + 0x00000006 => ErrorCodes::ErrorNoDataAvailable,
            v if v == ERROR_GROUP_DEVICE_COMMANDS + 0x00000007 => ErrorCodes::ErrorRepeatedHeader,
            v if v == ERROR_GROUP_DEVICE_COMMANDS + 0x00000008 => ErrorCodes::ErrorRepeatedTail,
            v if v == ERROR_GROUP_DEVICE_COMMANDS + 0x00000009 => ErrorCodes::ErrorIllFormedMessage,
            v if v == ERROR_GROUP_DEVICE_COMMANDS + 0x0000000A => ErrorCodes::ErrorWrongClampModality,
            v if v == ERROR_GROUP_DEVICE_COMMANDS + 0x8000000B => ErrorCodes::WarningValueClipped,
            v if v == ERROR_GROUP_DEVICE_COMMANDS + 0x0000000C => ErrorCodes::ErrorCompensationNotEnabled,
            v if v == ERROR_GROUP_DEVICE_COMMANDS + 0x0000000D => ErrorCodes::ErrorLiquidJunctionAndRecalibration,
            v if v == ERROR_GROUP_DEVICE_COMMANDS + 0x0000000E => ErrorCodes::ErrorWrongInputLength,

            v if v == ERROR_GROUP_DEVICE_FEATURES + 0x00000000 => ErrorCodes::ErrorFeatureNotImplemented,
            v if v == ERROR_GROUP_DEVICE_FEATURES + 0x00000001 => ErrorCodes::ErrorUpgradesNotAvailable,

            v if v == ERROR_GROUP_DEVICE_ISSUES + 0x00000000 => ErrorCodes::ErrorExpiredDevice,

            v if v == ERROR_GROUP_LIBRARY_ISSUES + 0x00000000 => ErrorCodes::ErrorMemoryInitialization,

            v if v == ERROR_GROUP_CALIBRATION + 0x00000000 => ErrorCodes::ErrorCalibrationDirMissing,
            v if v == ERROR_GROUP_CALIBRATION + 0x00000001 => ErrorCodes::ErrorCalibrationMappingNotOpened,
            v if v == ERROR_GROUP_CALIBRATION + 0x00000002 => ErrorCodes::ErrorCalibrationMappingCorrupted,
            v if v == ERROR_GROUP_CALIBRATION + 0x00000003 => ErrorCodes::ErrorCalibrationFileCorrupted,
            v if v == ERROR_GROUP_CALIBRATION + 0x00000004 => ErrorCodes::ErrorCalibrationFileMissing,
            v if v == ERROR_GROUP_CALIBRATION + 0x00000005 => ErrorCodes::ErrorCalibrationSoftwareBug,
            v if v == ERROR_GROUP_CALIBRATION + 0x00000006 => ErrorCodes::ErrorCalibrationNotLoadedYet,
            v if v == ERROR_GROUP_CALIBRATION + 0x00000007 => ErrorCodes::ErrorCalibrationMappingWrongNumbering,

            _ => ErrorCodes::ErrorUnknown,
        }
    }
}

impl ErrorCodes {
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
