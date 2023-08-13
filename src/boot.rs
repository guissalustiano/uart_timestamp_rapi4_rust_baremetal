use core::arch::asm;
use core::ptr;
use crate::time::ARCH_TIMER_COUNTER_FREQUENCY;

use aarch64_cpu::registers::MPIDR_EL1;
use tock_registers::interfaces::Readable;

// Defined from linked script
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

          b {func_start_rust}
      ", 
      stack_end = sym _stack_end,
      func_start_rust = sym _start_rust,
      options(noreturn));
}

#[inline(always)]
fn get_cpu_id() -> u64 {
    MPIDR_EL1.get() & 0b11
}

#[inline(always)]
fn hald_minors_cpus() {
    if get_cpu_id() != 0 {
        loop {
            aarch64_cpu::asm::wfe();
        }
    }
}

#[inline(always)]
pub unsafe fn zero_bss() {
     asm!("
          adrp {0}, {bss_begin}
          adr {0}, {bss_begin}

          adrp {1}, {bss_end}
          adr {1}, {bss_end}

          7:
            cmp	{0}, {1}
            b.eq	9f
            stp	xzr, xzr, [{0}], #16
            b	7b
         9:
      ", 
      out(reg) _,
      out(reg) _,
      bss_begin = sym _bss_begin,
      bss_end = sym _bss_end,
  );
}

#[inline(always)]
pub unsafe fn setup_timer() {
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
}


#[allow(dead_code)]
pub unsafe fn _start_rust() -> ! {
    hald_minors_cpus();
    zero_bss();
    setup_timer();

    crate::kernel_init()
}

