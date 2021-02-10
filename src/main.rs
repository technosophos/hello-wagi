fn main() {
    println!("Content-Type: text/plain\n\n Hello from main()");
}

// Use no_mangle so we can call this from WAGI or other external tools.
#[no_mangle]
/// A provider function that can be called directly
pub fn hello() {
    println!("Content-Type: text/plain\n\n Hello")
}

#[no_mangle]
/// Another provider function that can be called directly.
pub fn goodbye() {
    println!("Content-Type: text/plain\n\n Goodbye")
}

// This maps a few routes:
// '/hello' will result in the `hello()` function being called.
// '/goodbye' and all subpaths of '/goodbye' will call the `goodbye()` function.
//
// Note that when compiled, the `main` function is named `_start()`. So if you want
// to map to that function, it is `/main _start`.
#[no_mangle]
pub fn _routes() {
    println!("/hello hello");
    println!("/goodbye/... goodbye");
    println!("/main _start");
}
