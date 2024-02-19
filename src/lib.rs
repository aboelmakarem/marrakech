
#![no_std]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> !
{
	loop {}
}

#[no_mangle]
extern "C"
fn kernel_main() -> !
{
    loop {}
}
