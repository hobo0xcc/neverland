#![feature(panic_info_message, start, naked_functions)]
#![no_std]
#![no_main]

extern crate alloc;

use copland_os::*;
use core::arch::asm;
use log::info;

// It will merely jump to `boot`. Don't do anything else.
#[no_mangle]
#[start]
#[link_section = ".entry"]
#[naked]
pub unsafe extern "C" fn _entry() -> ! {
    #[cfg(target_arch = "riscv64")]
    asm!("j boot", options(noreturn));
    #[cfg(target_arch = "aarch64")]
    asm!("b boot", options(noreturn));
}

// Boot assembly for each ISA
#[no_mangle]
#[naked]
pub unsafe extern "C" fn boot() -> ! {
    #[cfg(target_arch = "riscv64")]
    asm!(include_str!("arch/riscv64/boot.S"), options(noreturn));
    #[cfg(target_arch = "aarch64")]
    asm!(include_str!("arch/aarch64/boot.S"), options(noreturn));
}

#[no_mangle]
#[cfg(target_arch = "riscv64")]
pub unsafe extern "C" fn main() -> ! {
    KERNEL_LOCK.lock();

    allocator::init_allocator();
    logger::init_logger();

    println!("PRESENT DAY\n  PRESENT TIME");

    info!("Arch: RISC-V");
    info!("Core: {}", crate::arch::riscv64::riscv::STATE.cpuid());

    {
        use copland_os::arch::riscv64::*;
        vm::VM_MANAGER.init();
    }

    task::TASK_MANAGER.init();

    let id = task::TASK_MANAGER.create_task("init", init as usize);
    task::TASK_MANAGER.ready_task(id);
    task::TASK_MANAGER.schedule();

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
    info!("Core: {}", crate::arch::aarch64::arm::STATE.cpuid());

    {
        use copland_os::arch::aarch64::*;
        vm::VM_MANAGER.init();
    }

    task::TASK_MANAGER.init();

    let id = task::TASK_MANAGER.create_task("init", init as usize);
    task::TASK_MANAGER.ready_task(id);
    task::TASK_MANAGER.schedule();

    loop {}
}

#[cfg(target_arch = "riscv64")]
pub unsafe extern "C" fn init() {
    use copland_os::arch::riscv64;
    use copland_os::device::common::virtio;

    info!("init");

    riscv64::plic::PLIC_MANAGER.init_irq(riscv64::plic::PlicIRQ::VirtIO0);
    virtio::block::VIRTIO_BLOCK.init(riscv64::address::_virtio_start as usize);

    let root_dir = fs::fat32::FILE_SYSTEM.root_dir();
    for e in root_dir.iter().map(|e| e.unwrap()) {
        println!("{}", e.file_name());
    }

    loop {
        task::TASK_MANAGER.schedule();
    }
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "C" fn init() {
    use crate::device::raspi3b::framebuffer::*;
    use crate::device::raspi3b::mailbox::*;
    use crate::device::raspi3b::*;
    use alloc::alloc::{alloc_zeroed, Layout};

    info!("init");

    MBOX.mbox[0] = 8 * 4;
    MBOX.mbox[1] = MBOX_REQUEST;
    MBOX.mbox[2] = MBOX_TAG_GETSERIAL;
    MBOX.mbox[3] = 8;
    MBOX.mbox[4] = 8;
    MBOX.mbox[5] = 0;
    MBOX.mbox[6] = 0;
    MBOX.mbox[7] = MBOX_TAG_LAST;

    if MBOX.mbox_call(MBOX_CH_PROP) {
        println!(
            "serial number: {}",
            ((MBOX.mbox[6] as usize) << 32) | MBOX.mbox[5] as usize
        );
    }

    let width = FRAMEBUFFER.width as usize;
    let height = FRAMEBUFFER.height as usize;
    for y in 0..height {
        for x in 0..width {
            FRAMEBUFFER.fb.add(y * width + x).write(0xffffffff);
        }
    }

    sandbox::fb_char::fb_char();
    if sd::SDCARD.sd_init() != sd::SDError::SD_OK.bits() {
        panic!("SDError");
    }
    let buf = {
        let layout = Layout::from_size_align(512, 512).unwrap();
        alloc_zeroed(layout)
    };

    for i in 0..512 {
        if i % 8 == 0 {
            println!();
        }
        let ch = buf.add(i).read();
        print!("{:02x} ", ch);
    }
    println!();

    loop {
        task::TASK_MANAGER.schedule();
    }
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
