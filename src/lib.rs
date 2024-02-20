
#![no_std]
pub mod uart;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> !
{
	loop {}
}

#[no_mangle]
extern "C"
fn kernel_main()
{
    // Create a new UART communicator and use it to write strings 
    // to display. 
    let mut uart0: uart::UART = uart::UART::new(0x10000000);
    uart0.init();
    uart0.write("welcome to marrakech");
    loop
    {
        let c: u8 = uart0.get();
        if c != 0
        {
            uart0.put(10);
            uart0.put(c);
        }
    }
}
