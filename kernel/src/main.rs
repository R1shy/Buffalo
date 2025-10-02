#![no_std]
#![no_main]

use bootloader_api::{entry_point, BootloaderConfig};
use core::panic::PanicInfo;


pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.kernel_stack_size = 90 * 1024;
    config
};

entry_point!(main, config = &BOOTLOADER_CONFIG);

fn main(bootinfo: &'static mut bootloader_api::BootInfo) -> ! {
    if let Some(framebuffer) = bootinfo.framebuffer.as_mut() {
        for byte in framebuffer.buffer_mut() {
            *byte = 0x45; 
        }
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
