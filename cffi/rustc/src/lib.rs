// Very simple starter for ffi from C -> Rust -> C. I want to play around with passing structs and
// how to pass data between the two languages. Hopefully this will give a good reference point for
// more challenging ffi scenarios

#[no_mangle]
pub extern "C" fn hello_from_rust() {
    println!("hello from Rust!");
}
