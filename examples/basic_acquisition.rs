//! Rust port of the reference e384CommLib C++ sample: detect/connect, inspect device features,
//! run digital offset compensation while draining acquisition data, run a 3-step voltage
//! protocol while logging samples to disk and applying a mid-run voltage offset.
//!
//! `E384Measurement`/`E384RangedMeasurement` carry `prefix`/`unit` as raw codes (no `niceLabel`
//! equivalent exists in this crate), so this example includes its own small formatters.

#![allow(non_upper_case_globals)]

use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;

use e384_rust::device::Device;
use e384_rust::error_codes::ErrorCodes;
use e384_rust::sys::{
    E384Measurement, E384RangedMeasurement, E384UnitPfx_E384_PFX_FEMTO,
    E384UnitPfx_E384_PFX_GIGA, E384UnitPfx_E384_PFX_KILO, E384UnitPfx_E384_PFX_MEGA,
    E384UnitPfx_E384_PFX_MICRO, E384UnitPfx_E384_PFX_MILLI, E384UnitPfx_E384_PFX_NANO,
    E384UnitPfx_E384_PFX_NONE, E384UnitPfx_E384_PFX_PETA, E384UnitPfx_E384_PFX_PICO,
    E384UnitPfx_E384_PFX_TERA, E384Unit_E384_UNIT_A, E384Unit_E384_UNIT_C, E384Unit_E384_UNIT_HZ,
    E384Unit_E384_UNIT_NONE, E384Unit_E384_UNIT_S, E384Unit_E384_UNIT_V,
};

fn prefix_multiplier(prefix: i32) -> f64 {
    match prefix {
        E384UnitPfx_E384_PFX_FEMTO => 1e-15,
        E384UnitPfx_E384_PFX_PICO => 1e-12,
        E384UnitPfx_E384_PFX_NANO => 1e-9,
        E384UnitPfx_E384_PFX_MICRO => 1e-6,
        E384UnitPfx_E384_PFX_MILLI => 1e-3,
        E384UnitPfx_E384_PFX_NONE => 1.0,
        E384UnitPfx_E384_PFX_KILO => 1e3,
        E384UnitPfx_E384_PFX_MEGA => 1e6,
        E384UnitPfx_E384_PFX_GIGA => 1e9,
        E384UnitPfx_E384_PFX_TERA => 1e12,
        E384UnitPfx_E384_PFX_PETA => 1e15,
        _ => 1.0,
    }
}

fn prefix_symbol(prefix: i32) -> &'static str {
    match prefix {
        E384UnitPfx_E384_PFX_FEMTO => "f",
        E384UnitPfx_E384_PFX_PICO => "p",
        E384UnitPfx_E384_PFX_NANO => "n",
        E384UnitPfx_E384_PFX_MICRO => "u",
        E384UnitPfx_E384_PFX_MILLI => "m",
        E384UnitPfx_E384_PFX_NONE => "",
        E384UnitPfx_E384_PFX_KILO => "k",
        E384UnitPfx_E384_PFX_MEGA => "M",
        E384UnitPfx_E384_PFX_GIGA => "G",
        E384UnitPfx_E384_PFX_TERA => "T",
        E384UnitPfx_E384_PFX_PETA => "P",
        _ => "?",
    }
}

fn unit_str(unit: i32) -> &'static str {
    match unit {
        E384Unit_E384_UNIT_NONE => "",
        E384Unit_E384_UNIT_V => "V",
        E384Unit_E384_UNIT_A => "A",
        E384Unit_E384_UNIT_HZ => "Hz",
        E384Unit_E384_UNIT_S => "s",
        E384Unit_E384_UNIT_C => "C",
        _ => "?",
    }
}

fn describe_measurement(m: E384Measurement) -> String {
    format!("{}{}{}", m.value, prefix_symbol(m.prefix), unit_str(m.unit))
}

fn describe_range(r: E384RangedMeasurement) -> String {
    format!(
        "[{}, {}] step {} {}{}",
        r.min,
        r.max,
        r.step,
        prefix_symbol(r.prefix),
        unit_str(r.unit)
    )
}

fn no_prefix_value(m: E384Measurement) -> f64 {
    m.value * prefix_multiplier(m.prefix)
}

fn write_samples(file: &mut File, samples: &[i16]) -> std::io::Result<()> {
    let bytes: Vec<u8> = samples.iter().flat_map(|s| s.to_le_bytes()).collect();
    file.write_all(&bytes)
}

fn main() -> Result<(), ErrorCodes> {
    // Detect the available devices.
    let device_ids = match Device::list_devices() {
        Ok(ids) => ids,
        Err(ErrorCodes::ErrorNoDeviceFound) => {
            println!("Error device not found");
            return Ok(());
        }
        Err(e) => {
            println!("Another error: {e:?}");
            return Err(e);
        }
    };
    for (i, id) in device_ids.iter().enumerate() {
        println!("Detected device {i}: {id}");
    }

    // Connect to the available device.
    let device = Device::connect(&device_ids[0])?;

    // Check the available voltage and current channels.
    let (voltage_channels_num, current_channels_num) = device.channel_number_features_u16()?;
    println!("Available voltage channels: {voltage_channels_num}");
    println!("Available current channels: {current_channels_num}");
    println!();

    // Check the available current ranges.
    let (current_ranges, _) = device.vc_current_ranges()?;
    println!("Available current ranges");
    for (i, range) in current_ranges.iter().enumerate() {
        println!("\t Current range {i}: {}", describe_range(*range));
    }
    println!();

    let (_voltage_ranges, _) = device.vc_voltage_ranges()?;

    // Check the available sampling rates.
    let sampling_rates = device.sampling_rates_features()?;
    println!("Available sampling rates");
    for (i, rate) in sampling_rates.iter().enumerate() {
        println!("\t Sampling rate {i}: {}", describe_measurement(*rate));
    }
    println!();

    // set current range 100nA
    let apply_flag = true;
    let _res = device.set_vc_current_range_all(0, apply_flag);

    // set sampling rate 1.25MHz
    let _res = device.set_sampling_rate(5, apply_flag);
    let sampling_rate = device.sampling_rate()?;

    // Wait 5s and then purge to avoid downloading old data.
    thread::sleep(Duration::from_millis(5000));
    device.purge_data()?;

    // turn on digital offset compensation on all channels
    let channel_indexes = [0u16];
    let mut on_values = [true];
    let _ = device.digital_offset_compensation(&channel_indexes, &on_values, true);
    println!("Started digital offset compensation");

    // allocate the buffer needed to read data while the digital offset compensation runs
    let mut wait_buffer = device.allocate_rx_buffer()?;
    let mut acc_data_to_read: u32 = 0;
    let target_samples = (10.0 * no_prefix_value(sampling_rate)) as u32;
    while acc_data_to_read <= target_samples {
        // Read packets. This has to be done for the digital offset compensation to work.
        match  wait_buffer.next_message(-1) {
            Ok((rx_out, _data)) => {
                if rx_out.dataLen > 0 {
                    acc_data_to_read +=
                        rx_out.dataLen / (voltage_channels_num + current_channels_num) as u32;
                }
            },
            Err(ErrorCodes::ErrorNoDataAvailable) => thread::sleep(Duration::from_millis(5)),
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
    // deallocate buffer for digital offset compensation data
    drop(wait_buffer);

    // turn off digital offset compensation on all channels
    on_values[0] = false;
    device.digital_offset_compensation(&channel_indexes, &on_values, true)?;
    println!("Stopped digital offset compensation");

    // Check digital offset compensation success
    let liquid_junction_statuses = device.liquid_junction_statuses(&channel_indexes)?;
    println!("Liquid junction status: {}", liquid_junction_statuses[0]);

    let liquid_junction_voltages = device.liquid_junction_voltages(&channel_indexes)?;
    println!(
        "Compensated voltage: {}",
        describe_measurement(liquid_junction_voltages[0])
    );
    println!();

    // Protocol definition, square wave by means of 3 items.
    // Everytime a new protocol is defined, its prot_id should be incremental.
    let prot_id = 0u16;
    let items_num = 3u16;
    let sweeps_num = 1u16;
    let v_rest = E384Measurement {
        value: 0.0,
        prefix: E384UnitPfx_E384_PFX_NONE,
        unit: E384Unit_E384_UNIT_V,
    };
    let stop_protocol_flag = true;
    device.set_voltage_protocol_structure(
        prot_id,
        items_num,
        sweeps_num,
        v_rest,
        stop_protocol_flag,
    )?;

    let t0 = E384Measurement {
        value: 100.0,
        prefix: E384UnitPfx_E384_PFX_MILLI,
        unit: E384Unit_E384_UNIT_S,
    };
    let t_step = E384Measurement {
        value: 0.0,
        prefix: E384UnitPfx_E384_PFX_MILLI,
        unit: E384Unit_E384_UNIT_S,
    };
    let reps_num = 0u16; // 0 means "run indefinitely"
    let apply_steps = false;
    let v_half_flag = false;

    let v0 = E384Measurement {
        value: 0.0,
        prefix: E384UnitPfx_E384_PFX_MILLI,
        unit: E384Unit_E384_UNIT_V,
    };
    let v_step = E384Measurement {
        value: 0.0,
        prefix: E384UnitPfx_E384_PFX_MILLI,
        unit: E384Unit_E384_UNIT_V,
    };
    device.set_voltage_protocol_step(
        0, 1, reps_num, apply_steps, v0, v_step, t0, t_step, v_half_flag, &[],
    )?;

    let v0 = E384Measurement {
        value: 100.0,
        prefix: E384UnitPfx_E384_PFX_MILLI,
        unit: E384Unit_E384_UNIT_V,
    };
    device.set_voltage_protocol_step(
        1, 2, reps_num, apply_steps, v0, v_step, t0, t_step, v_half_flag, &[],
    )?;

    let v0 = E384Measurement {
        value: -100.0,
        prefix: E384UnitPfx_E384_PFX_MILLI,
        unit: E384Unit_E384_UNIT_V,
    };
    device.set_voltage_protocol_step(
        2, 0, reps_num, apply_steps, v0, v_step, t0, t_step, v_half_flag, &[],
    )?;

    device.start_protocol()?;

    let chan_idxs = [0u16];

    // Define how much data to save to file: 10s worth at the current sampling rate.
    let my_data_to_read = (10.0 * no_prefix_value(sampling_rate)) as u32;
    let mut acc_data_to_read: u32 = 0;

    device.purge_data()?;
    let mut fid = File::create("myLog.dat").expect("failed to create myLog.dat");
    let mut buffer = device.allocate_rx_buffer()?;

    let mut offset_modified = false;

    while acc_data_to_read <= my_data_to_read {
        if acc_data_to_read > my_data_to_read / 2 && !offset_modified {
            // after ~5s apply a voltage offset to make the output data more interesting
            offset_modified = true;

            let v_step = E384Measurement {
                value: 100.0,
                prefix: E384UnitPfx_E384_PFX_MILLI,
                unit: E384Unit_E384_UNIT_V,
            };
            device.set_voltage_hold_tuner(&chan_idxs, &[v_step], true)?;
        }

        // Read packets.
        match  buffer.next_message(-1) {
            Ok((rx_out, data)) => {
                if rx_out.dataLen > 0 {
                    acc_data_to_read +=
                        rx_out.dataLen / (voltage_channels_num + current_channels_num) as u32;
                    write_samples(&mut fid, data).expect("failed to write to myLog.dat");
                }
            },
            Err(ErrorCodes::ErrorNoDataAvailable) => thread::sleep(Duration::from_millis(5)),
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
    drop(buffer);

    println!("Ended voltage protocol");
    println!();

    println!("Disconnect the device");
    drop(device);

    Ok(())
}
