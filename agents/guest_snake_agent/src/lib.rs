#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[export_name = "decide"]
pub extern "C" fn decide(obs: u64) -> u64 {
    let hx = (obs >> 24) & 0xFF;
    let hy = (obs >> 16) & 0xFF;
    let ax = (obs >> 8) & 0xFF;
    let ay = obs & 0xFF;

    if hx < ax {
        3
    } else if hx > ax {
        2
    } else if hy < ay {
        1
    } else {
        0
    }
}
