#![no_std]
#![no_main]
mod console;
mod lang_items;
mod sbi;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

#[unsafe(no_mangle)] // 避免名字进行混淆
pub fn rust_main() -> ! {
    clear_bss();
    println!("1.2");
    loop {}
}

fn clear_bss() {
    unsafe extern "C" {
        // 尝试从其他地方找到全局符号
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) })
}
