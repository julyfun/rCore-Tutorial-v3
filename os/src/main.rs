#![no_std]
#![no_main]
// mod console;
mod lang_items;
mod logging;
mod sbi;

use core::arch::global_asm;
use log::*;
global_asm!(include_str!("entry.asm"));

#[unsafe(no_mangle)] // 避免名字进行混淆
pub fn rust_main() -> ! {
    // 之前还会建立栈空间.

    clear_bss();

    crate::logging::init(); // 不能在清除 BSS 之前调用
    info!("v1.4");
    panic!("It should not reach here!");
}

// 将程序 .bss 段（未初始化的全局变量和静态变量）全部写入 0，以实现全局变量全 0 的效果）
fn clear_bss() {
    unsafe extern "C" {
        // 尝试从其他地方找到全局符号，给后面 for_each 使用
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) })
}
