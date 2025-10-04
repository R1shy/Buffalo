#![no_std]
#![no_main]


mod framebuffer;

use bootloader_api::{entry_point, info::FrameBufferInfo, BootloaderConfig};
use bootloader_x86_64_common::logger::LockedLogger;
use conquer_once::spin::OnceCell;
use core::panic::PanicInfo;




pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.kernel_stack_size = 90 * 1024;
    config
};


pub(crate) static LOGGER: OnceCell<LockedLogger> = OnceCell::uninit();
pub(crate) fn init_logger(buffer: &'static mut [u8], info: FrameBufferInfo) {
    let logger = LOGGER.get_or_init(move || LockedLogger::new(buffer, info, true, false));
    log::set_logger(logger).expect("Logger already set");
    log::set_max_level(log::LevelFilter::Trace);
}


entry_point!(main, config = &BOOTLOADER_CONFIG);


fn main(bootinfo: &'static mut bootloader_api::BootInfo) -> ! {
    let frame_buffer_optional = &mut bootinfo.framebuffer;
    let frame_buffer_option = frame_buffer_optional.as_mut();
    let frame_buffer_struct = frame_buffer_option.unwrap();
    let frame_buffer_info = frame_buffer_struct.info().clone();
    let raw_frame_buffer = frame_buffer_struct.buffer_mut();
    init_logger(raw_frame_buffer, frame_buffer_info);
    
    kernel::init();


    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {

    log::error!("@ {}\nBECAUSE: {}", 
        info.location().expect("REASON"),
        info.message());
    loop {}
}
