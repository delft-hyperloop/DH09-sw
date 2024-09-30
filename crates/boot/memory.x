MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* TODO Adjust these memory regions to match your device memory layout */
  /* These values correspond to the LM3S6965, one of the few devices QEMU can emulate */
/*
  FLASH                             : ORIGIN = 0x08000000, LENGTH = 24K
  BOOTLOADER_STATE                  : ORIGIN = 0x08006000, LENGTH = 32K
  ACTIVE                            : ORIGIN = 0x0800E000, LENGTH = 512K
  DFU                               : ORIGIN = 0x0808E000, LENGTH = 516K
  RAM                        (rwx)  : ORIGIN = 0x20000000, LENGTH = 64K
*/


  FLASH                             : ORIGIN = 0x08000000, LENGTH = 512K
  BOOTLOADER_STATE                  : ORIGIN = 0x08080000, LENGTH = 8K
  ACTIVE                            : ORIGIN = 0x080C2000, LENGTH = 256K
  DFU                               : ORIGIN = 0x08202000, LENGTH = 260K
  RAM                         (rwx) : ORIGIN = 0x20000000, LENGTH = 128K
}


__bootloader_state_start = ORIGIN(BOOTLOADER_STATE) - ORIGIN(FLASH);
__bootloader_state_end = ORIGIN(BOOTLOADER_STATE) + LENGTH(BOOTLOADER_STATE) - ORIGIN(FLASH);

__bootloader_active_start = ORIGIN(ACTIVE) - ORIGIN(FLASH);
__bootloader_active_end = ORIGIN(ACTIVE) + LENGTH(ACTIVE) - ORIGIN(FLASH);

__bootloader_dfu_start = ORIGIN(DFU) - ORIGIN(FLASH);
__bootloader_dfu_end = ORIGIN(DFU) + LENGTH(DFU) - ORIGIN(FLASH);

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* You may want to use this variable to locate the call stack and static
   variables in different memory regions. Below is shown the default value */
/* _stack_start = ORIGIN(RAM) + LENGTH(RAM); */

/* You can use this symbol to customize the location of the .text section */
/* If omitted the .text section will be placed right after the .vector_table
   section */
/* This is required only on microcontrollers that store some configuration right
   after the vector table */
/* _stext = ORIGIN(FLASH) + 0x400; */

/* Example of putting non-initialized variables into custom RAM locations. */
/* This assumes you have defined a region RAM2 above, and in the Rust
   sources added the attribute `#[link_section = ".ram2bss"]` to the data
   you want to place there. */
/* Note that the section will not be zero-initialized by the runtime! */
/* SECTIONS {
     .ram2bss (NOLOAD) : ALIGN(4) {
       *(.ram2bss);
       . = ALIGN(4);
     } > RAM2
   } INSERT AFTER .bss;
*/