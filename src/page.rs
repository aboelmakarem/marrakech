// Marrakech Operating System
// Ahmed Hussein (amhussein4@gmail.com)
// 2/21/2024

use crate::uart;

extern "C"
{
	static HEAP_START: usize;
	static HEAP_SIZE: usize;
}

// The page data is encoded in a flag that tells whether a physical memory page is taken and 
// if it is, whether it is the last page in a block of allocated pages. 

pub struct Page
{
	// bit 0: is page allocated ?
	// bit 1: is last page in allocation block ?
	flags: u8
}

impl Page
{
	pub fn new() -> Self
	{
		Page{flags : 0}
	}

	pub fn allocated(&self) -> bool
	{
		self.flags & 0x01 != 0
	}

	pub fn last(&self) -> bool
	{
		self.flags & 0x02 != 0
	}

	pub fn clear(&mut self)
	{
		self.flags = 0;
	}

	pub fn allocate(&mut self)
	{
		self.flags |= 0x01;
	}

	pub fn mark_last(&mut self)
	{
		self.flags |= 0x02;
	}

	pub fn free(&mut self)
	{
		self.clear();
	}

	pub fn mark_not_last(&mut self)
	{
		self.flags &= 0xfd;
	}
}

// PAGE_ALLOCATION_START is the memory address at which 
// page allocation starts
static mut PAGE_ALLOCATION_START: usize = 0;
const PAGE_SIZE: usize = 4096;

// Returns the multiple of block that lies in the 
// interval [value,value + block).
fn align_to(value: usize,block: usize) -> usize
{
	block*((value + block - 1)/block)
}

// Initialize the page index, which is an array of page 
// flags that indicate whether physical memory pages 
// are allocated and where they lie in their allocation 
// blocks.
pub fn initialize_page_index()
{
	unsafe
	{
		// Calculate total number of pages.
		let page_count = HEAP_SIZE/PAGE_SIZE;
		// The page index starts at the beginning 
		// of the heap
		let index_ptr = HEAP_START as *mut Page;
		// Clear all pages in the index
		for i in 0..page_count
		{
			(*(index_ptr.add(i))).clear();
		}
		// Page allocation begins right after the 
		// page index ends, however, because pages 
		// have to start at a PAGE_SIZE boundary, the 
		// remaining space up to the next boundary 
		// is not used for pages. 
		let index_size = page_count*core::mem::size_of::<Page>();
		PAGE_ALLOCATION_START = align_to(HEAP_START + index_size,PAGE_SIZE);
	}
}

// Allocates a contiguous block of pages of size block_size. 
// This function assumes that the page index has been initialized 
// and PAGE_ALLOCATION_START has been set.
pub fn allocate_page_block(block_size: usize) -> *mut u8
{
	if block_size == 0
	{
		return core::ptr::null_mut();
	}
	unsafe
	{
		let page_count = HEAP_SIZE/PAGE_SIZE;
		let index_ptr = HEAP_START as *mut Page;
		// Go over the page index and look for a free 
		// page that can be the start of the block. For 
		// this, we only need to iterate up to 
		// (page_count - block_size) because no blocks 
		// of size block_size can be found after that. 
		for i in 0..(page_count - block_size)
		{
			let mut found = false;
			if !((*index_ptr.add(i)).allocated())
			{
				// found a free page, check to see if 
				// it is followed by block_size free 
				// pages
				found = true;
				for j in (i + 1)..(i + block_size)
				{
					if (*index_ptr.add(j)).allocated()
					{
						// found an allocated page, page(i) 
						// cannot be the start of the block
						found = false;
						break;
					}
				}
			}
			// If control reaches this, then there is a 
			// free block of pages of size block_size that 
			// starts at page i
			if found
			{
				// Mark all block pages as allocated
				for j in i..(i + block_size)
				{
					(*index_ptr.add(j)).allocate();
				}
				// Mark last block page as last
				(*index_ptr.add(i + block_size - 1)).mark_last();
				// Form the address of the start page
				let address = PAGE_ALLOCATION_START + i*PAGE_SIZE;
				// Cast it to a byte pointer and return it
				return address as *mut u8;
			}
		}
	}
	// If control reaches this point, then no contiguous 
	// blocks of the given size were found. Return a null 
	// pointer. 
	core::ptr::null_mut()
}

// Allocates a contiguous block of pages of size block_size 
// and zeroes its contents. 
pub fn zero_allocate_page_block(block_size: usize) -> *mut u8
{
	// Allocate a page block
	let block_start: *mut u8 = allocate_page_block(block_size);
	unsafe
	{
		if !block_start.is_null()
		{
			// Write zeros to the pages in blocks of 
			// 8 bytes rather than byte by byte
			let block_ptr = block_start as *mut u64;
			let total_size = (block_size*PAGE_SIZE)/8;
			for i in 0..total_size
			{
				(*block_ptr.add(i)) = 0;
			}
		}
	}
	block_start
}

// Frees a block of pages given the start address of the first
// page. 
pub fn free_block(address: *mut u8) -> bool
{
	if address.is_null()
	{
		return false;
	}
	unsafe
	{
		// Get the index of the page from its address
		let int_address = address as usize;
		if int_address < PAGE_ALLOCATION_START
		{
			return false;
		}
		if int_address >= (HEAP_START + HEAP_SIZE)
		{
			return false;
		}
		let index = (int_address - PAGE_ALLOCATION_START)/PAGE_SIZE;
		// The page index starts at the HEAP_START
		let mut page = (HEAP_START + index) as *mut Page;
		// Free all pages up to the last page
		while (*page).allocated() && !(*page).last()
		{
			(*page).free();
			page = page.add(1);
		}
		// The last page has to be allocated and marked as last
		if !(*page).allocated() || !(*page).last()
		{
			return false;
		}
		// Free the last page
		(*page).free();
	}
	true
}

fn page_address(index: usize) -> usize
{
	unsafe
	{
		PAGE_ALLOCATION_START + index*PAGE_SIZE
	}
}

// Prints the allocated ranges of pages
pub fn print_page_allocations()
{
	let mut uart0: uart::UART = uart::UART::new(0x10000000);
    uart0.init();
    uart0.writeln("Page Allocations");
	unsafe
	{
		// Go over all pages in the page index until an allocated page 
		// is found. The page index starts at the heap start.
		let page = HEAP_START as *const Page;
		let page_count = HEAP_SIZE/PAGE_SIZE;
		let mut i = 0;
		while i < page_count
		{
			if (*page.add(i)).allocated()
			{
				let start_index = i;
				loop
				{
					if !(*page.add(i)).last()
					{
						// Verify that page is indeed allocated
						if !(*page.add(i)).allocated()
						{
							uart0.write("invalid page allocation found");
							break;
						}
						i += 1;
					}
					else
					{
						break;
					}
				}
				let end_index = i;
				let start_address = page_address(start_index);
				let end_address = page_address(end_index);
				uart0.write("page block allocated :");
				uart0.write_address(start_address);
				uart0.write("-->");
				uart0.write_address(end_address);
				uart0.writeln(".");
			}
			i += 1;
		}
	}
}
