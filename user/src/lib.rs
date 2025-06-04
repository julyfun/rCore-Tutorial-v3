#![no_std]
#![feature(linkage)]

use core::panic;

#[macro_use]
pub mod console; // os/ 复制过来的
mod lang_items;
mod syscall;

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
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

#[linkage = "weak"] // 使得在 bin 下就算找不到 main 也能编译通过 [? 有啥意义]
#[unsafe(no_mangle)]
fn main() -> i32 {
    panic!("No main function defined");
}

use syscall::*;

// fd: 文件描述符.
// 胖指针 buf 包含 as_ptr() 和 len().
// 也被 console.rs 使用.
pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}

pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}
