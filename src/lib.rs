pub struct RString(String);

fn to_rstring(s: impl Into<String>) -> Box<RString> {
    Box::new(RString(s.into()))
}

/// # Safety
/// None whatsoever.
#[no_mangle]
pub unsafe extern "C" fn rstring_data(s: &RString) -> *const u8 {
    s.0.as_ptr()
}

/// # Safety
/// None whatsoever.
#[no_mangle]
pub unsafe extern "C" fn rstring_len(s: &RString) -> usize {
    // correctness: len() on Rust strings returns the number of bytes, not characters/graphemes
    s.0.len()
}

/// # Safety
/// None whatsoever.
#[no_mangle]
pub unsafe extern "C" fn rstring_free(s: *mut RString) {
    let s = Box::from_raw(s);
    drop(s);
}

/// # Safety
/// None whatsoever.
#[no_mangle]
pub unsafe extern "C" fn do_whatever(c: *const u8, len: usize) {
    let bytes = std::slice::from_raw_parts(c, len);
    let s = std::str::from_utf8(bytes).unwrap();
    do_whatever_impl(s);
}

fn do_whatever_impl(s: &str) {
    println!("Hello from Rust or whatever. Your string is: {}", s);
}

/// # Safety
/// None whatsoever.
#[no_mangle]
pub unsafe extern "C" fn returns_whatever() -> Box<RString> {
    let s = String::from("Yeah allocations, sure");
    to_rstring(s)
}
