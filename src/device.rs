//! `Device` — the connection handle every other `device_*` module adds methods to. Covers
//! device discovery (`e384_detectDevices`/`e384_listAllDevices`) and the connect/disconnect
//! lifecycle. `Drop` calls `e384_disconnect` unconditionally and ignores its result, per the
//! header's contract that the handle is invalid afterward regardless of the returned error code.

use std::ffi::CString;
use tracing::instrument;

use crate::error_codes::ErrorCodes;
use crate::sys::{E384Device, E384DeviceList};
use crate::util::{owned_string_list, translate};

/// An open connection to an e384 device. `E384ChannelModel`/`E384BoardModel` handles obtained via
/// [`crate::channel_model::Channel`]/[`crate::board_model::Board`] and the RX buffer obtained via
/// [`crate::device_acquisition::RxBuffer`] all borrow from this type and become unusable once it
/// is dropped.
pub struct Device(pub(crate) *mut E384Device);

impl Device {
    /// Wraps `e384_connect`.
    #[instrument]
    pub fn connect(device_id: &str) -> Result<Self, ErrorCodes> {
        let Ok(c_id) = CString::new(device_id) else {
            return Err(ErrorCodes::ErrorDeviceNotFound);
        };
        let mut ptr: *mut E384Device = std::ptr::null_mut();
        unsafe { translate(crate::sys::e384_connect(c_id.as_ptr(), &mut ptr)) }?;
        Ok(Device(ptr))
    }

    /// Wraps `e384_detectDevices`.
    #[instrument]
    pub fn list_devices() -> Result<Vec<String>, ErrorCodes> {
        let mut list: *mut E384DeviceList = std::ptr::null_mut();
        unsafe { translate(crate::sys::e384_detectDevices(&mut list)) }?;
        let devices = unsafe { owned_string_list(list) };
        tracing::info!("Found {} device(s):", devices.len());
        Ok(devices)
    }

    /// Wraps `e384_listAllDevices` (includes already-owned devices, unlike `list_devices`).
    #[instrument]
    pub fn list_all_devices() -> Result<Vec<String>, ErrorCodes> {
        let mut list: *mut E384DeviceList = std::ptr::null_mut();
        unsafe { translate(crate::sys::e384_listAllDevices(&mut list)) }?;
        let devices = unsafe { owned_string_list(list) };
        tracing::info!("Found {} device(s):", devices.len());
        Ok(devices)
    }
}

impl Drop for Device {
    /// Wraps `e384_disconnect` (`overheatFlag` always `0`); result ignored, see module docs.
    fn drop(&mut self) {
        unsafe { crate::sys::e384_disconnect(self.0, 0) };
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        fs::{self, File},
        io,
        path::PathBuf,
    };

    use crate::device::Device;

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    enum PlsFiles {
        E384_DEMO,
        E384_RX_RAW,
        E384_TX,
        EMCR_debug,
    }
    impl PlsFiles {
        fn get(&self) -> &str {
            match self {
                PlsFiles::E384_DEMO => "e384_DEMO.pls",
                PlsFiles::E384_RX_RAW => "e384_RX_RAW.pls",
                PlsFiles::E384_TX => "e384_TX.pls",
                PlsFiles::EMCR_debug => "EMCR_debug.pls",
            }
        }
    }

    struct FileManager {
        home: PathBuf,
        stautses: HashMap<PlsFiles, bool>,
    }

    impl FileManager {
        fn new() -> Self {
            let Some(home) = std::env::home_dir() else {
                eprintln!("Home dir could not be found");
                std::process::exit(1);
            };
            let stautses = vec![
                PlsFiles::E384_DEMO,
                PlsFiles::E384_RX_RAW,
                PlsFiles::E384_TX,
                PlsFiles::EMCR_debug,
            ]
            .into_iter()
            .map(|s| {
                let p = home.join(s.get());
                (s, p.is_file())
            })
            .collect();
            Self { home, stautses }
        }
        fn restore(&self) {
            self.stautses.iter().for_each(|(pf, s)| {
                if *s {
                    let _ = self.create(pf);
                }
            });
        }
        fn create(&self, pf: &PlsFiles) -> io::Result<()> {
            let p = self.home.join(pf.get());
            if p.is_file() {
                Ok(())
            } else {
                match File::create(p) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                }
            }
        }
        #[allow(dead_code)]
        fn remove(&self, pf: PlsFiles) -> io::Result<()> {
            let p = self.home.join(pf.get());
            if !p.is_file() {
                return Ok(());
            } else {
                fs::remove_file(p)
            }
        }
    }

    #[test]
    fn list_demo_devices() {
        let fm = FileManager::new();
        let Ok(_) = fm.create(&PlsFiles::E384_DEMO) else {
            eprintln!("Could not create file");
            std::process::exit(1);
        };
        assert!(fm.home.join(PlsFiles::E384_DEMO.get()).is_file());
        let Ok(devs) = Device::list_devices() else {
            eprintln!("Errors during list_devices");
            std::process::exit(1);
        };
        assert!(devs.len() >= 6);
        fm.restore();
    }
}
