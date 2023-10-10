#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(sketch_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use sketch_os::clear;
use sketch_os::println;

//don't mangle this function's name (basically, don' fuck it up)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("hey there. this is a test of the vga buffer on this operating system i am coding, called 'sketch_os'. i'm doing it for my DofE, which is basically a large program that adults (big people) put kids through to make thenm better? i'm not reallys ure of the point.hey there. this is a test of the vga buffer on this operating system i am coding, called 'sketch_os'. i'm doing it for my DofE, which is basically a large program that adults (big people) put kids through to make thenm better? i'm not reallys ure of the point.hey there. this is a test of the vga buffer on this operating system i am coding, called 'sketch_os'. i'm doing it for my DofE, which is basically a large program that adults (big people) put kids through to make thenm better? i'm not reallys ure of the point.hey there. this is a test of the vga buffer on this operating system i am coding, called 'sketch_os'. i'm doing it for my DofE, which is basically a large program that adults (big people) put kids through to make thenm better? i'm not reallys ure of the point. hey there. this is a test of the vga buffer on this operating system i am coding, called 'sketch_os'. i'm doing it for my DofE, which is basically a large program that adults (big people) put kids through to make thenm better? i'm not reallys ure of the point.hey there. this is a test of the vga buffer on this operating system i am coding, called 'sketch_os'. i'm doing it for my DofE, which is basically a large program that adults (big people) put kids through to make thenm better? i'm not reallys ure of the point.hey there. this is a test of the vga buffer on this operating system i am coding, called 'sketch_os'. i'm doing it for my DofE, which is basically a large program that adults (big people) put kids through to make thenm better? i'm not reallys ure of the point.hey there. this is a test of the vga buffer on this operating system i am coding, called 'sketch_os'. i'm doing it for my DofE, which is basically a large program that adults (big people) put kids through to make thenm better? i'm not reallys ure of the point.hey there. this is a test of the vga buffer on this operating system i am coding, called 'sketch_os'. i'm doing it for my DofE, which is basically a large program that adults (big people) put kids through to make thenm better? i'm not reallys ure of the point.hey there. this is a test of the vga buffer on this operating system i am coding, called 'sketch_os'. i'm doing it for my DofE, which is basically a large program that adults (big people) put kids through to make thenm better? i'm not reallys ure of the point.hey there. this is a test of the vga buffer on this operating system i am coding, called 'sketch_os'. i'm doing it for my DofE, which is basically a large program that adults (big people) put kids through to make thenm better? i'm not reallys ure of the point.hey there. this is a test of the vga buffer on this operating system i am coding, called 'sketch_os'. i'm doing it for my Dof");

    clear!();

    #[cfg(test)]
    test_main();

    #[allow(clippy::empty_loop)]
    loop {}
}

//call this on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    sketch_os::test_panic_handler(info)
}
