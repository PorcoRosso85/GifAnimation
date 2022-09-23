#[link(wasm_import_module = "mod")]
extern {
  fn foo();
}

#[no_mangle]
pub extern fn bar() {
}

#[link_section = "hello"]
pub static SECTION: [u8; 24] = *b"This is a custom section";



#[link(wasm_import_module = "mod")]
extern {
  fn foo();
}

#[no_mangle]
pub extern fn bar() {}


#[link_section = "hello"]
pub static SECTION: [u8; 24] = *b"This is a custom section";