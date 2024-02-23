# Marrakech OS
# Ahmed Hussein (amhussein4@gmail.com)
# February 13th 2024

# Generate RISC-V 32-bit, not compressed, instructions
.option norvc

# Define a special text section for the boot code
.section .text.boot

# Make start symbol global
.global start

start:
	# Initialize all machine and supervisor mode status registers
	# for all harts in the system
	# 1. zero (m/s)tval, (m/s)cause and (m/s)scratch
	csrw mtval,zero
	csrw stval,zero
	csrw mcause,zero
	csrw scause,zero
	csrw mscratch,zero
	csrw sscratch,zero
	# 2. Do not modify any of the event counters or the type of 
	# the events they count. The following registers
	# mcycle, minstret, mhpmcounter3 --> mhpmcounter31, 
	# mhpmevent3 --> mhpmevent31, cycle, time, instret, 
	# hpmcounter3 --> hpmcounter31
	# 3. Allow all counters to run by disabling all counter 
	# inhibitions
	#csrw mcountinhibit, zero
	# 4. Enable supervisor mode to read time and instret 
	# counters only
	li t0,0x07
	csrw mcounteren,t0
	# 5. Enable user mode to read time, cycle and instret 
	# counters only
	csrw scounteren,t0
	# 6. Clear and forget about all pending interrupts
	csrw mip,zero
	csrw sip,zero
	# 7. Clear page table base address register
	csrw satp,zero
	# 8. Delegate all exceptions and interrupts to supervisor mode 
	# for all harts
	li t1,0xffff
	csrw mideleg,t1
	csrw medeleg,t1
	# 9. Set the machine mode trap vector location and use the 
	# same location for supervisor mode trap vector
	la t2,trap_location
	csrw mtvec,t2
	csrw stvec,t2
	# 10. Set machine and supervisor mode return addresses to zero for now
	# this will be updated below for hart 0
	csrw mepc,zero
	csrw sepc,zero

	# Hart 0 will be designated to take all external 
	# interrupts in addition to its timer and software 
	# interrupts while other harts will handle their software 
	# and timer interrupts. This will be set on a per-hart 
	# basis below.

	# Store the hart ID in t0
	csrr t0,mhartid
	# If this is not core 0, sleep
	bnez t0,non_zero_sleep
	# The following runs on hart 0 only
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
	# Set stack pointer
	la sp,kernel_stack_end
	# Set machine mode status
	# bits 11 and 12: machine privilege level
	# bit 7: machine mode past interrupt enabled
	# bit 3: machine mode current interrupt enabled
	li t2,0x1888
	csrw mstatus,t2
	# Set supervisor mode status to user-level and no interrupt 
	# handling for now
	csrw sstatus,zero
	# Overwrite the address in mepc by the address of the 
	# kernel entry point and use mret to return to it
	la t0,kernel_main
	csrw mepc,t0
	# Enable all traps in mie (timer, software and external)
	# for machine mode, this requires setting bits 3, 7 and 11
	# in mie register
	li t2,0x0888
	csrw mie,t2
	csrw sie,t2
	# Upon executing mret below, the control will go to the 
	# kernel and will never come back. Set the return address 
	# to the sleep section below to have something to return 
	# to but it will never be used. 
	la ra,non_zero_sleep
	mret

non_zero_sleep:
	# Set machine mode status to user privilege level and disable all 
	# previous and current interrupt handling for non-zero harts. Apply 
	# the same settings to supervisor mode. 
	csrw mstatus,zero
	csrw sstatus,zero
	# Enable software exceptions and disable all
	# external and timer interrupts for non-zero harts 
	li t2,0x06
	csrw mie,t2
	csrw sie,t2
	# zero machine mode return address
	csrw mepc,zero
	# Sleep until woken up by an interrupt from another core
	wfi
	j	non_zero_sleep

