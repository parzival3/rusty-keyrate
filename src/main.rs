use core::ffi::c_void;
use clap::Parser;

use windows::{
    Win32::UI::Accessibility::FILTERKEYS,
    Win32::UI::WindowsAndMessaging::SPI_SETFILTERKEYS,
    Win32::UI::WindowsAndMessaging::SystemParametersInfoW
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Delay in ms
    #[arg(short, long)]
    delay: u32,

    /// Repeat in ms
    #[arg(short, long)]
    repeat: u32,
}

const FKF_FILTERKEYSON: u32 = 0x00000001;
const FKF_AVAILABLE: u32 = 0x00000002;

fn system_parameter_info(mut keys: FILTERKEYS) -> bool {
    let ptr: *mut FILTERKEYS = &mut keys;
    let voidptr = ptr as *mut c_void;
    unsafe {
        SystemParametersInfoW(SPI_SETFILTERKEYS, 0, Some(voidptr), windows::Win32::UI::WindowsAndMessaging::SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0)).as_bool()
    }
}

fn main() {
    let args = Args::parse();
    println!("Setting delay to {} and repeat to {}", args.delay, args.repeat);
    let filter_keys = FILTERKEYS { cbSize: std::mem::size_of::<FILTERKEYS>() as u32, dwFlags: FKF_FILTERKEYSON | FKF_AVAILABLE, iWaitMSec: 0, iDelayMSec: args.delay, iRepeatMSec: args.repeat, iBounceMSec: 0 };
    if !system_parameter_info(filter_keys) {
        eprintln!("Failed to set the key parameters");
    }
}
