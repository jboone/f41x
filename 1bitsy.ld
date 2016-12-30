MEMORY
{
  PROGRAM (rx) : ORIGIN = 0x00000000, LENGTH = 1024k
  FLASH   (rx) : ORIGIN = 0x08000000, LENGTH = 1024K
  RAM     (rw) : ORIGIN = 0x20000000, LENGTH =  112K
  RAM2    (rw) : ORIGIN = 0x2001c000, LENGTH =   16K
}

ENTRY(_reset)

SECTIONS
{
  .text : /*ORIGIN(FLASH) :*/
  {
    /* Vector table */
    _VECTOR_TABLE = .;
    LONG(ORIGIN(RAM) + LENGTH(RAM));
    LONG(_reset + 1);
    KEEP(*(.rodata._EXCEPTIONS));
    _eexceptions = .;

    INCLUDE interrupts_text.ld

    /* Entry point: reset handler */
    _reset = .;
    *(.text._reset);

    *(.text.*);
    *(.rodata.*);
  } > PROGRAM AT > FLASH

  .bss : ALIGN(4)
  {
    _sbss = .;
    *(.bss.*);
    _ebss = ALIGN(4);
  } > RAM

  .data : ALIGN(4)
  {
    _sdata = .;
    *(.data.*);
    _edata = ALIGN(4);
  } > RAM AT > FLASH

  _sidata = LOADADDR(.data);
  /*_heap_start = .;*
  /*_heap_end = ORIGIN(RAM) + LENGTH(RAM);*/
  /DISCARD/ :
  {
    *(.ARM.exidx.*)
    *(.note.gnu.build-id.*)
  }
}

/* HACK to make these symbols overrideable by _dependencies_ (they were
   already overridable by the top crate), we declare them as undefined
   (EXTERN) here. */
EXTERN(__aeabi_memclr4);
EXTERN(_default_exception_handler);
EXTERN(_init);

/* Exceptions */
PROVIDE(_nmi = _default_exception_handler);
PROVIDE(_hard_fault = _default_exception_handler);
PROVIDE(_memmanage_fault = _default_exception_handler);
PROVIDE(_bus_fault = _default_exception_handler);
PROVIDE(_usage_fault = _default_exception_handler);
PROVIDE(_svcall = _default_exception_handler);
PROVIDE(_pendsv = _default_exception_handler);
PROVIDE(_systick = _default_exception_handler);

ASSERT(_eexceptions - ORIGIN(PROGRAM) == 0x40, "exceptions not linked where expected");

INCLUDE interrupts_sym.ld
