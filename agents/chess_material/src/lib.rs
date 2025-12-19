#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn decide(_obs: u64) -> u64 {
    // Extremely naive move: pawn push
    let from: u8 = 8;
    let to: u8 = 16;
    pack_move(from, to, 0)
}

fn pack_move(from: u8, to: u8, promo: u8) -> u64 {
    (from as u64)
        | ((to as u64) << 6)
        | ((promo as u64) << 12)
}
