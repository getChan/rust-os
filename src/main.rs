#![no_std]

use core::panic::PanicInfo;

// 패닉 발생 시 해당 함수 호출
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop {
    }
}

fn main() {
}
