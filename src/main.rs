fn main() {
    println!(
        r#"Status: 200
Content-Type: text/plain

Hello World!
"#
    );
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
