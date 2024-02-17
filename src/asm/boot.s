# Marrakech OS
# Ahmed Hussein (amhussein4@gmail.com)
# February 13th 2024

# Generate RISC-V 32-bit, not compressed, instructions
.option norvc

# Define a special text section for the boot code
.section .text.boot

.global start
start:
	# Store the hart ID in t0
	csrr t0,mhartid
	# If this is not core 0, sleep
	bnez t0,non_zero_sleep
	# Clear SATP, which is the page table base address
	csrw satp,zero
	# push the current options stack
	.option push
	# Do not relax instructions when loading global pointer. 
	# This way, addresses in instructions are not calculated 
	# based on a datum value in global pointer which still 
	# has not been set yet. 
	.option norelax
	# load the address at global_pointer to register gp
	# global_pointer is an address that is defined in the 
	# linker script. The global_pointer points to the end 
	# of the text section of the program
	la gp,global_pointer
	# pop back the options stack (get rid of last option)
	.option pop
	# Zero-out all BSS section. The symbols bss_start and 
	# bss_end are defined in the linker script and they 
	# point to the beginning and end of bss section
	la a0,bss_start
	la a1,bss_end
	bgeu a0,a1,configure_hart
	# loop over all bytes in BSS and set them to zero
bss_clear_loop:
	# write zero to the double word pointed to by the 
	# address in a0
	sd zero,(a0)
	# move to the next double word
	addi a0,a0,8
	# iterate
	bltu a0,a1,bss_clear_loop

configure_hart:
	

non_zero_sleep:
	# Sleep until woken up by an interrupt from another core
	wfi
	j	non_zero_sleep

