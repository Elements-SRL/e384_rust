use std::ffi::CStr;
use tracing::instrument;
use crate::{error_codes::ErrorCodes, util::translate};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

struct RMessageDispatcher(*mut E384Device);

impl RMessageDispatcher {
    #[instrument]
    pub fn list_devices() -> Result<Vec<String>, ErrorCodes> {
        let mut list: *mut E384DeviceList = std::ptr::null_mut();
        unsafe { translate(e384_detectDevices(&mut list).into()) }?;

        let count = unsafe { e384_deviceList_count(list) };
        tracing::info!("Found {count} device(s):");

        let mut device_ids: Vec<String> = Vec::with_capacity(count);
        for i in 0..count {
            let ptr = unsafe { e384_deviceList_get(list, i) };
            if ptr.is_null() {
                continue;
            }
            // Copy the string out now — it's only valid until deviceList_free.
            let id = unsafe { CStr::from_ptr(ptr) }
                .to_string_lossy()
                .into_owned();
            device_ids.push(id);
        }
        unsafe { e384_deviceList_free(list) };
        Ok(device_ids)
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs::{self, File}, io, path::PathBuf};
    use crate::r_message_dispatcher::RMessageDispatcher;

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

    struct FileManager{
        home: PathBuf, 
        stautses: HashMap<PlsFiles, bool>
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
            .into_iter().map(|s| {
                let p = home.join(s.get());
                (s, p.is_file())
            })
            .collect();
            Self{home, stautses}
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
                    Err(e) => Err(e)
                }
            }
        }
        fn remove(&self, pf: PlsFiles) -> io::Result<()>  {
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
        let Ok(devs) = RMessageDispatcher::list_devices() else {
            eprintln!("Errors during list_devices");
            std::process::exit(1);
        };
        assert!(devs.len() >= 6);
        fm.restore();
    }

    // #[test]
    // #[serial(demo_devices)]
    // fn no_devs() {
    //     let fm = FileManager::new();
    //     let Ok(()) = fm.remove(PlsFiles::E384_DEMO) else {
    //         eprintln!("Could not delete {}", PlsFiles::E384_DEMO.get());
    //         std::process::exit(1);
    //     };
    //     assert!(!fm.home.join(PlsFiles::E384_DEMO.get()).is_file());
    //     match RMessageDispatcher::list_devices() { 
    //         Ok(devs) => {
    //             println!("{:?}", devs);
    //             assert!(devs.len() < 6)
    //         },
    //         Err(e) => {
    //             assert_eq!(e, ErrorCodes::ErrorNoDeviceFound)
    //         }
    //     }
    //     fm.restore();
    // }
}
