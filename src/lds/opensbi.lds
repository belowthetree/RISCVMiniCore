/*
 opensbi.lds
 QEMU 链接程序
 2024年2月22日 zgg
*/

OUTPUT_ARCH( "riscv" )

. = 0x80200000;
ENTRY( _start )

MEMORY
{
  ram   (wxa) : ORIGIN = 0x80200000, LENGTH = 512M
}

PHDRS
{
  text PT_LOAD;
  data PT_LOAD;
  bss PT_LOAD;
}

SECTIONS
{
  .text : {
    PROVIDE(_text_start = .);
	
    *(.text.init) *(.text .text.*)

    PROVIDE(_text_end = .);
	
  } >ram AT>ram :text
   
   PROVIDE(_global_pointer = .);
   
  .rodata : {
    PROVIDE(_rodata_start = .);
    *(.rodata .rodata.*)
    PROVIDE(_rodata_end = .);
	
  } >ram AT>ram :text

  .data : {
	
    . = ALIGN(4096);
    PROVIDE(_data_start = .);
	
    *(.sdata .sdata.*) *(.data .data.*)
    PROVIDE(_data_end = .);
  } >ram AT>ram :data

  .bss : {
    PROVIDE(_bss_start = .);
    *(.sbss .sbss.*) *(.bss .bss.*)
    PROVIDE(_bss_end = .);
  } >ram AT>ram :bss

  PROVIDE(_memory_start = ORIGIN(ram));
  
  PROVIDE(_stack_start = _bss_end);
  PROVIDE(_stack_end = _stack_start + 0x80000);
  PROVIDE(_trap_stack_start = _stack_end);
  PROVIDE(_trap_stack_end = _trap_stack_start + 0x160000);
  PROVIDE(_trap_stack_start2 = _trap_stack_end);
  PROVIDE(_trap_stack_end2 = _trap_stack_start2 + 0x160000);
  PROVIDE(_memory_end = ORIGIN(ram) + LENGTH(ram));

  PROVIDE(_heap_start = _trap_stack_end2);
  PROVIDE(_heap_size = _memory_end - _heap_start);
}
