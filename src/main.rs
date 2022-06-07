#![feature(panic_info_message, start)]
#![no_std]
#![no_main]

extern crate alloc;
extern crate copland_os;

use alloc::alloc::{alloc_zeroed, Layout};
use copland_os::*;
use core::arch::asm;
use log::info;

#[cfg(target_arch = "riscv64")]
pub unsafe extern "C" fn init() {
    use copland_os::arch::riscv64;
    use copland_os::device::common::virtio;

    info!("init");

    riscv64::plic::PLIC_MANAGER
        .lock()
        .init_irq(riscv64::plic::PlicIRQ::VirtIO0);
    virtio::block::VIRTIO_BLOCK
        .lock()
        .init(riscv64::address::_virtio_start as usize);

    let buf = alloc_zeroed(Layout::from_size_align(512, 512).unwrap());
    virtio::block::VIRTIO_BLOCK
        .lock()
        .block_op(buf, 0, virtio::block::BlockOpType::Read);

    loop {
        task::TASK_MANAGER.lock().schedule();
    }
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "C" fn init() {
    info!("init");
    loop {
        task::TASK_MANAGER.lock().schedule();
    }
}

#[no_mangle]
#[cfg(target_arch = "riscv64")]
pub unsafe extern "C" fn main() -> ! {
    KERNEL_LOCK.lock();

    allocator::init_allocator();
    logger::init_logger();

    println!("PRESENT DAY\n  PRESENT TIME");

    info!("Arch: RISC-V");
    info!(
        "Hart: {}",
        crate::arch::riscv64::riscv::STATE.lock().cpuid()
    );

    {
        use copland_os::arch::riscv64::*;
        vm::VM_MANAGER.lock().init();
    }

    task::TASK_MANAGER.lock().init();

    let id = task::TASK_MANAGER.lock().create_task("init", init as usize);
    task::TASK_MANAGER.lock().ready_task(id);
    task::TASK_MANAGER.lock().schedule();

    loop {}
}

#[no_mangle]
#[cfg(target_arch = "aarch64")]
pub unsafe extern "C" fn main() -> ! {
    KERNEL_LOCK.lock();

    allocator::init_allocator();
    logger::init_logger();

    println!("PRESENT DAY\n  PRESENT TIME");

    info!("Arch: AArch64");
    info!("Hart: {}", crate::arch::aarch64::arm::STATE.lock().cpuid());

    {
        use copland_os::arch::aarch64::*;
        vm::VM_MANAGER.lock().init();
    }

    task::TASK_MANAGER.lock().init();

    let id = task::TASK_MANAGER.lock().create_task("init", init as usize);
    task::TASK_MANAGER.lock().ready_task(id);
    task::TASK_MANAGER.lock().schedule();

    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn boot() -> ! {
    #[cfg(target_arch = "riscv64")]
    asm!(include_str!("arch/riscv64/boot.S"));
    #[cfg(target_arch = "aarch64")]
    asm!(include_str!("arch/aarch64/boot.S"));
    loop {}
}

#[no_mangle]
#[start]
#[link_section = ".entry"]
pub unsafe extern "C" fn _entry() -> ! {
    #[cfg(target_arch = "riscv64")]
    asm!("j boot", options(noreturn));
    #[cfg(target_arch = "aarch64")]
    asm!("b boot", options(noreturn));
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    print!("Panic: ");
    if let Some(location) = info.location() {
        println!(
            "line: {}, file: {}: {}",
            location.line(),
            location.file(),
            info.message().unwrap()
        );
    } else {
        println!("No information available");
    }
    loop {}
}
