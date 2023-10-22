/** 実行開始地点 */
ENTRY(boot)

SECTIONS {
    /** ベースアドレス  */
    . = 0x80200000;

    /** 実行可能なコード */
    .text :{
        KEEP(*(.text.boot));
        *(.text .text.*);
    }

    /** 読み取り専用データ:定数など */
    .rodata : ALIGN(4) {
        *(.rodata .rodata.*);
    }

    /** 読み書き可能なデータ:初期値を持つグローバル変数 */
    .data : ALIGN(4) {
        *(.data .data.*);
    }

    /** 未初期化データ: 初期値が0のグローバル変数 */
    .bss : ALIGN(4) {
        __bss = .;
        *(.bss .bss.* .sbss .sbss.*);
        __bss_end = .;
    }

    . = ALIGN(4);
    . += 128 * 1024; /* 128KB */
    __stack_top = .;
}