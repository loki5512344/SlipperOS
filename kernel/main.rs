#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod lib;
mod panic;

extern crate riscv;

use riscv::register::sstatus;

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    unsafe { sstatus::set_spp(sstatus::SPP::Supervisor) }

    crate::lib::uart_init();
    crate::lib::print_log("BOOT", "SlipperOS v0.1 booting...");

    crate::lib::print_seal();

    crate::lib::plic_init();
    crate::lib::print_log("BOOT", "plic ok");

    crate::lib::clint_init();
    crate::lib::print_log("BOOT", "clint ok");

    crate::lib::mm_init();
    crate::lib::print_log("MM", "page allocator ready");

    crate::lib::virtio_scan();
    crate::lib::print_log("VIRTIO", "scan complete");

    crate::lib::sched_init();
    crate::lib::print_log("PROC", "scheduler started");

    crate::lib::print_log("SLIP", "ready");
    crate::lib::shell_start();
}
