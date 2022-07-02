pub const MMIO_BASE: usize = 0x3F000000;
pub const MMIO_SIZE: usize = 0x01000000;
pub const GPFSEL0: usize = MMIO_BASE + 0x00200000;
pub const GPFSEL1: usize = MMIO_BASE + 0x00200004;
pub const GPFSEL2: usize = MMIO_BASE + 0x00200008;
pub const GPFSEL3: usize = MMIO_BASE + 0x0020000c;
pub const GPFSEL4: usize = MMIO_BASE + 0x00200010;
pub const GPFSEL5: usize = MMIO_BASE + 0x00200014;
pub const GPSET0: usize = MMIO_BASE + 0x0020001c;
pub const GPSET1: usize = MMIO_BASE + 0x00200020;
pub const GPCLR0: usize = MMIO_BASE + 0x00200028;
pub const GPLEV0: usize = MMIO_BASE + 0x00200034;
pub const GPLEV1: usize = MMIO_BASE + 0x00200038;
pub const GPEDS0: usize = MMIO_BASE + 0x00200040;
pub const GPEDS1: usize = MMIO_BASE + 0x00200044;
pub const GPHEN0: usize = MMIO_BASE + 0x00200064;
pub const GPHEN1: usize = MMIO_BASE + 0x00200068;
pub const GPPUD: usize = MMIO_BASE + 0x00200094;
pub const GPPUDCLK0: usize = MMIO_BASE + 0x00200098;
pub const GPPUDCLK1: usize = MMIO_BASE + 0x0020009c;
