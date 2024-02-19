# Marrakech OS
# Ahmed Hussein (amhussein4@gmail.com)
# February 13th 2024

.global trap_location

# This is the only entry to the trap vector
trap_location:
	mret
