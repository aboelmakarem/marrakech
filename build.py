
import os
import sys

# Define build target and type
build_target = "riscv64gc-unknown-none-elf"
build_type = "release"

# Define compilers, assemblers, linkers and flags
assembler = "riscv64-elf-as"
c_compiler = "riscv64-elf-gcc"
c_flags = "-ffreestanding -nostdlib"
linker = "riscv64-elf-ld"

# Define locations of source files
asm_path = "./src/asm"
c_path = "./src/c"
link_script = "./src/link_scripts/link.ld"

# Define output and output location for object files
o_path = "./objects"
os_lib_path = "./target/" + build_target + "/" + build_type + "/libmarrakech.a"
output = "marrakech.elf"

# Collect all sources
asm_sources = []
c_sources = []
if(os.path.exists(asm_path)):
	files = os.listdir(asm_path)
	asm_sources = [file for file in files if ".s" in file]

if(os.path.exists(c_path)):
	files = os.listdir(c_path)
	c_sources = [file for file in files if ".c" in file]

# Create object output directory
if(not os.path.exists(o_path)):
	command_string = "mkdir " + o_path
	os.system(command_string)

def compile_asm():
	for source in asm_sources:
		raw_name = source[:-2]
		source_path = asm_path + "/" + raw_name + ".s"
		target_path = o_path + "/" + raw_name + ".o"
		command_string = assembler + " " + source_path + " -o " + target_path
		print(command_string)
		os.system(command_string)

def compile_c():
	for source in c_sources:
		raw_name = source[:-2]
		source_path = c_path + "/" + raw_name + ".c"
		target_path = o_path + "/" + raw_name + ".o"
		command_string = c_compiler + " " + c_flags + " -c " + source_path + " -o " + target_path
		print(command_string)
		os.system(command_string)

def compile_rust():
	command_string = "cargo build --" + build_type
	print(command_string)
	os.system(command_string)

def link():
	objects = ""
	for source in asm_sources:
		raw_name = source[:-2]
		objects = objects + " " + o_path + "/" + raw_name + ".o"

	for source in c_sources:
		raw_name = source[:-2]
		objects = objects + " " + o_path + "/" + raw_name + ".o"

	command_string = linker + " -T" + link_script + " " + objects + " " + os_lib_path + " -o " + output
	print(command_string)
	os.system(command_string)

def clean():
	command_string = "cargo clean"
	print(command_string)
	os.system(command_string)
	command_string = "rm " + o_path + "/*.o"
	os.system(command_string)

def build():
	compile_asm()
	compile_c()
	compile_rust()
	link()

build()
