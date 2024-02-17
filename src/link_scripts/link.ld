# Marrakech OS
# Ahmed Hussein (amhussein4@gmail.com)
# February 16th 2024

# Define the architecture we are generating this for
OUTPUT_ARCH("riscv")

# Set the entry point to "start" which is defined in boot.s
ENTRY(start)

# Define the memory section which tells the linker where the 
# memory starts, how big it is and the memory permissions. 
MEMORY
{
	# Memory is writable (w), executable (x), allocatable (a), 
	# not read only (!r) or initialized (!i). It starts at 
	# location 0x80000000 because this is where RISC-V boards 
	# and emulators load the boot code to and it is of size 
	# 2048 MB = 2 GB. The memory name is "ram"
	ram (wxa!ri) : ORIGIN = 0x80000000,LENGTH = 2048M
}

# Define program headers and specify that they need to be 
# loaded from target file to memory
PHDRS
{
	text PT_LOAD;
	data PT_LOAD;
	bss PT_LOAD;
}

# Define all program sections
SECTIONS
{
	# Program code
	.text : 
	{
		PROVIDE(text_start = .);
		# put the boot text first before anything else
		*(.text.boot) *(.text .text.*)
		PROVIDE(text_end = .);
		# > ram: Put this section in the "text" section of virtual 
		# memory called "ram"
		# AT>ram: When loading the output, load it to the physical 
		# memory called ram, similar to the virtual memory. Since 
		# we know that the entry will be set at a given address, 
		# this means that both virtual and physical memory addresses 
		# of any byte in the output match which reduces the work 
		# required in memory access translation and improves overall
		# OS performance. 
	} > ram AT>ram :text
	# Define global pointer to be the first byte in the data 
	# section. This will be used in the assembly code to set the 
	# global pointer register and refer to instructions by their 
	# offset from global pointer. 
	PROVIDE(global_pointer = .);

	# Program read-only data
	.rodata : 
	{
		PROVIDE(rodata_start = .);
		*(.rodata .rodata.*)
		PROVIDE(rodata_end = .);
		# Place read-only data in text section along with the 
		# program code
	} > ram AT>ram :text

	# Program initialized read/write data
	.data : 
	{
		PROVIDE(data_start = .);
		# put the boot text first before anything else
		*(.text.boot) *(.text .text.*)
		PROVIDE(data_end = .);
	}

	# Program uninitialized data
	.bss : 
	{
		PROVIDE(text_start = .);
		# put the boot text first before anything else
		*(.text.boot) *(.text .text.*)
		PROVIDE(text_end = .);
		
	}
}
