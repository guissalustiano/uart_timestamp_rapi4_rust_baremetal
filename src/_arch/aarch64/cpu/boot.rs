// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2021-2022 Andre Richter <andre.o.richter@gmail.com>

//! Architectural boot code.
//!
//! # Orientation
//!
//! Since arch modules are imported into generic modules using the path attribute, the path of this
//! file is:
//!
//! crate::cpu::boot::arch_boot

use core::arch::asm;
use core::ptr;
use crate::time::ARCH_TIMER_COUNTER_FREQUENCY;

use aarch64_cpu::registers::MPIDR_EL1;
use tock_registers::interfaces::Readable;

extern "C" {
    static mut _bss_begin: u8;
    static mut _bss_end: u8;

    static mut _stack_end: u8;
}


#[naked]
#[link_section = ".text._start"]
#[no_mangle]
pub unsafe extern "C" fn _init_rust() -> ! {
     // Init SP and zero out the .bss section
     asm!("
          adrp x0, {stack_end}
          adr x0, {stack_end}
          mov sp, x0

          adrp x0, {bss_begin}
          adr x0, {bss_begin}

          adrp x1, {bss_end}
          adr x1, {bss_end}

          7:
            cmp	x0, x1
            b.eq	{func_start_rust}
            stp	xzr, xzr, [x0], #16
            b	7b

          b {func_start_rust}
      ", 
      stack_end = sym _stack_end,
      bss_begin = sym _bss_begin,
      bss_end = sym _bss_end,
      func_start_rust = sym _start_rust,
      options(noreturn));
}

#[allow(dead_code)]
#[link_section = ".text._start"]
pub unsafe fn _start_rust() -> ! {
    // Stop all other cores except core 0
    if get_cpu_id() != 0 {
        aarch64_cpu::asm::wfe();
    }

    // Zero out the .bss section
    let count = &_bss_begin as *const u8 as usize - &_bss_end as *const u8 as usize;
    ptr::write_bytes(&mut _bss_begin as *mut u8, 0, count);


	// Read the CPU's timer counter frequency and store it in ARCH_TIMER_COUNTER_FREQUENCY.
     asm!("
          adrp {0:x}, {1}
          add {0:x}, {0:x}, #:lo12:{1}
          mrs {2:x}, cntfrq_el0
          str {2:w}, [{0:x}]

      ", 
      out(reg) _, 
      sym ARCH_TIMER_COUNTER_FREQUENCY,
      out(reg) _,
      );

     crate::kernel_init()
}

#[inline(always)]
fn get_cpu_id() -> u64 {
    MPIDR_EL1.get() & 0b11
}
