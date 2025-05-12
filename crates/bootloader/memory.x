MEMORY {
  BOOTLOADER       : ORIGIN = 0x0800_0000, LENGTH = 32K
  BOOTLOADER_STATE : ORIGIN = 0x0800_8000, LENGTH = 1K
  DFU              : ORIGIN = 0x0800_8400, LENGTH = 64K
  ACTIVE           : ORIGIN = 0x0801_8400, LENGTH = 1903K

  RAM              : ORIGIN = 0x2400_0000, LENGTH = 512K
  RAM_D3           : ORIGIN = 0x3800_0000, LENGTH = 64K
}

SECTIONS {
  .text : {
    KEEP(*(.vector*))
    *(.text*)
    *(.rodata*)
    . = ALIGN(4);
  } > BOOTLOADER

  .boot_state : {
    KEEP(*(.boot_state*))
  } > BOOTLOADER_STATE

}
