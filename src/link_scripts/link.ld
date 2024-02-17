


SECTIONS
{
	.text : 
	{
		PROVIDE(text_start = .);
		# put the boot text first before anything else
		*(.text.boot) *(.text .text.*)
		PROVIDE(text_end = .);
	}
	PROVIDE(global_pointer = .);

}
