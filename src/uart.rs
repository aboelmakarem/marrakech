// Marrakech Operating System
// Ahmed Hussein (amhussein4@gmail.com)
// 2/15/2024


// Takes a number and a quartet index and returns the hexdemical digit (0 - F) 
// that corresponds to the chosen quartet. A quartet is a group of 4 bits. The 
// quartets are indexed starting from the least significant side (index 0) up 
// to the most significant side where the index increases by 1 every 4 bits. 
fn hex_quartet_digit(number: usize,index: usize) -> u8
{
	let shift = 4*index;
	let quartet = (number >> shift) & 0xf;
	let digit = match quartet
	{
		0 => b'0',
		1 => b'1',
		2 => b'2',
		3 => b'3',
		4 => b'4',
		5 => b'5',
		6 => b'6',
		7 => b'7',
		8 => b'8',
		9 => b'9',
		10 => b'a',
		11 => b'b',
		12 => b'c',
		13 => b'd',
		14 => b'e',
		15 => b'f',
		_ => 0
	};
	digit
}

// A struct to encapsulate all UART communication related data
pub struct UART
{
	base_address: usize
}

impl UART
{
	pub fn new(address: usize) -> Self
	{
		UART{base_address : address}
	}
	pub fn init(&mut self)
	{
	    let ptr = self.base_address as *mut u8;
	    unsafe
	    {
		    // 1. Set the UART word length to 8 bits:
		    //      Set the first 2 bits of the LCR register to 11 
		    //      binary (or 3 decimal). 
		    //      The LCR register is at UART base address + 3.
		    let lcr: u8 = 3;
		    ptr.add(3).write_volatile(lcr);
		    // 2. Enable FIFO communications:
		    //      Set the first bit of the FCR register to 1
		    //      The FCR register is at UART base address + 2
		    ptr.add(2).write_volatile(1);
		    // 3. Enable UART receiver interrupts:
		    //      Set the first bit of the IER register to 1
		    //      IER register is at UART base address + 1
		    ptr.add(1).write_volatile(1);
		    // 4. Set UART channel BAUD rate:
		    //      Write the BAUD rate divisor to the divisor 
		    //      register (LS and MS). The divisor is equal 
		    //      to the ceil of the division of the base 
		    //      clock rate of 22.729 Mhz by 16 times the 
		    //      required BAUD rate. 
		    //      D = ceil(22.729 Mhz/(16 BAUD))
		    //      Here, we use a BAUD rate of 2400. Hence, 
		    //      D = ceil(22729000/(16*2400)) = 592
		    let divisor:u16 = 592;
		    let divisor_low_byte: u8 = (divisor & 0xff) as u8;
		    let divisor_high_byte: u8 = (divisor >> 8) as u8;
		    // Before setting the LS and MS registers, we need 
		    // to enable DLAB access by setting the top bit 
		    // of LCR register to 1.
		    ptr.add(3).write_volatile(lcr | (1 << 7));
		    // Write divisor, the LS and MS registers are at 
		    // UART base address + 0 and 1 respectively
		    ptr.add(0).write_volatile(divisor_low_byte);
		    ptr.add(1).write_volatile(divisor_high_byte);
		    // Finally, disable DLAB access so that the registers 
		    // at UART base + 0 and 1 can be used for UART 
		    // communication and interrupt control. Disabling is 
		    // done by setting the top bit of LCR to zero.
		    ptr.add(3).write_volatile(lcr);
		}
	}
	pub fn put(&mut self,byte: u8)
	{
	    let ptr = self.base_address as *mut u8;
	    unsafe
	    {
	    	ptr.add(0).write_volatile(byte);
	    }
	}
	pub fn get(&mut self) -> u8
	{
	    let ptr = self.base_address as *mut u8;
	    // See if there is data ready to be read by examining 
	    // the low bit of the LSR register which is at UART 
	    // base address + 5
	    unsafe
	    {
		    if ptr.add(5).read_volatile() & 0x01 == 0
		    {
		        return 0;
		    }
		    ptr.add(0).read_volatile()
		}
	}
	pub fn write(&mut self,string: &str)
	{
		for character in string.bytes()
		{
			self.put(character);
		}
	}
	pub fn writeln(&mut self,string: &str)
	{
		self.write(string);
		self.put(b'\n');
	}
	pub fn write_address(&mut self,address: usize)
	{
		// Write a number of hexdecimal digits that 
		// depends on the give address
		self.put(b' ');
		self.put(b'0');
		self.put(b'x');
		if address < 65536
		{
			// Write a 4-digit address 0xWWWW
			for i in (0..4).rev()
			{
				self.put(hex_quartet_digit(address,i));
			}
		}
		else if address < 4294967296
		{
			// Write an 8-digit address 0xVVVVWWWW
			for i in (0..8).rev()
			{
				self.put(hex_quartet_digit(address,i));
			}
		}
		else
		{
			// Write a 16-digit address 0xUUUUUUUUVVVVWWWW
			for i in (0..16).rev()
			{
				self.put(hex_quartet_digit(address,i));
			}
		}
		self.put(b' ');
	}
}

