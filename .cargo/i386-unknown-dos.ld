OUTPUT_FORMAT(binary)
ENTRY(_start)

MEMORY {
    RAM (rwx) : ORIGIN = 0x0100, LENGTH = 64K
}

SECTIONS {
    .text : {
        KEEP(*(.rt));
        *(.text .text.*)
        . = ALIGN(2);
    } > RAM

    .data : {
        *(.rodata .rodata.*)
        *(.data .data.*)
        . = ALIGN(2);
    } > RAM

    .bss (NOLOAD) : {
        PROVIDE(__bss_start = .);
        *(.bss .bss.*)
        . = ALIGN(2);
        PROVIDE(__bss_end = .);
        PROVIDE(__bss_size = __bss_end - __bss_start);
    } > RAM

    .heap (NOLOAD) : {
        PROVIDE(__heap_start = .);
        PROVIDE(__heap_end = ORIGIN(RAM) + LENGTH(RAM));
        PROVIDE(__heap_size = __heap_end - __heap_start);
    } > RAM
}
