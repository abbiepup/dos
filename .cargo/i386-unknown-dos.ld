OUTPUT_FORMAT(binary)
ENTRY(_start)

MEMORY {
    RAM (rwx) : ORIGIN = 0x0100, LENGTH = 64K - 256
}

SECTIONS {
    .text : {
        KEEP(*(.rt));
        *(.text .text.*)
        . = ALIGN(4);
    } > RAM

    .rodata : {
        *(.rodata .rodata.*)
        . = ALIGN(4);
    } > RAM

    .data : {
        *(.data .data.*)
        . = ALIGN(4);
    } > RAM

    .bss (NOLOAD) : {
        PROVIDE(__bss_start = .);
        *(.bss .bss.*)
        *(COMMON)
        . = ALIGN(4);
        PROVIDE(__bss_size = SIZEOF(.bss));
    } > RAM

    .heap (NOLOAD) : {
        PROVIDE(__heap_start = .);
        PROVIDE(__heap_end = ORIGIN(RAM) + LENGTH(RAM));
        PROVIDE(__heap_size = __heap_end - __heap_start);
    } > RAM
}
