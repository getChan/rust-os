#![no_std] // Rust 표준 라이브러리를 링크하지 않도록 합니다
#![no_main] // Rust 언어에서 사용하는 실행 시작 지점 (main 함수)을 사용하지 않습니다
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"] // cargo test에서 생성하는 함수의 이름을 test_main으로 변경

use core::panic::PanicInfo;

use os::println;

#[no_mangle] // 이 함수의 이름을 mangle하지 않습니다
pub extern "C" fn _start() -> ! {
    // 링커는 기본적으로 '_start' 라는 이름을 가진 함수를 실행 시작 지점으로 삼기에,
    // 이 함수는 실행 시작 지점이 됩니다
    println!("Hello World{}", "!");
    #[cfg(test)]
    test_main();

    loop {}
}

// 패닉 발생 시 해당 함수 호출
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    os::test_panic_handler(_info);
}
