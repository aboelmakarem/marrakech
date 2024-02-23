
#![no_std]
pub mod uart;
pub mod page;

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
    uart0.writeln("welcome to marrakech");
    page::initialize_page_index();
    let b1 = page::allocate_page_block(10);
    let b2 = page::allocate_page_block(5);
    let b3 = page::allocate_page_block(20);
    uart0.writeln("before");
    page::print_page_allocations();
    page::free_block(b2);
    uart0.writeln("after");
    page::print_page_allocations();
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
