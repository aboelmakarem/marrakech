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
	# Initialize all control registers
	# Set machine mode trap delegations, delegate everything to 
	# supervisor mode
	li t1,0xffff
	csrw mideleg,t1
	csrw medeleg,t1
	# Set stack pointer
	la sp,stack
	# Set machine mode status
	# bits 11 and 12: machine privilege level
	# bit 7: machine mode past interrupt enabled
	# bit 3: machine mode current interrupt enabled
	li t2,0x1888
	csrw mstatus,t2
	# Overwrite the address in mepc by the address of the 
	# kernel entry point and use mret to return to it
	la t0,kernel_main
	csrw mepc,t0
	# Set the machine mode trap vector location
	la t1,trap_location
	csrw mtvec,t1
	# Enable all traps in mie (timer, software and external)
	# for machine mode, this requires setting bits 3, 7 and 11
	# in mie register
	li t2,0x0888
	csrw mie,t2
	# Upon executing mret below, the control will go to the 
	# kernel and will never come back. Set the return address 
	# to the sleep section below to have something to return 
	# to but it will never be used. 
	la ra,non_zero_sleep
	mret

non_zero_sleep:
	# Sleep until woken up by an interrupt from another core
	wfi
	j	non_zero_sleep

