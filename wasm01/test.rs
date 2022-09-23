#![no_main]
#![no_std]

#[link(wasm_import_module = "imports")]
extern {
  fn console_log(x: i32);
}

#[no_mangle]
pub extern fn sayFive() {
  unsafe { console_log(5); }
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> !{ loop {} }








#[link(wasm_import_module = "mod")]
extern { fn foo(); }

#[no_mangle]
pub extern fn bar() {}




#[link_section = "hello"]
pub static SECTION: [u8; 24] = *b"This is a custome section";
