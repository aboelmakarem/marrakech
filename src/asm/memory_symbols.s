# Marrakech OS
# Ahmed Hussein (amhussein4@gmail.com)
# February 22nd 2024

# Import and expose memory symbols from linker script.
.section .rodata

# Each symbol to be exported is written as a double 
# word at a memory location that is marked by a 
# similar global symbol. 
.global TEXT_START
TEXT_START:
	.dword text_start

.global TEXT_END
TEXT_END:
	.dword text_end

.global RODATA_START
RODATA_START:
	.dword rodata_start

.global RODATA_END
RODATA_END:
	.dword rodata_end

.global DATA_START
DATA_START:
	.dword data_start

.global DATA_END
DATA_END:
	.dword data_end

.global BSS_START
BSS_START:
	.dword bss_start

.global BSS_END
BSS_END:
	.dword bss_end

.global KERNEL_STACK_START
KERNEL_STACK_START:
	.dword kernel_stack_start

.global KERNEL_STACK_END
KERNEL_STACK_END:
	.dword kernel_stack_end

.global HEAP_START
HEAP_START:
	.dword heap_start

.global HEAP_END
HEAP_END:
	.dword heap_end

.global HEAP_SIZE
HEAP_SIZE:
	.dword heap_size

