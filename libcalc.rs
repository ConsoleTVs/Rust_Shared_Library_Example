#![crate_type = "lib"]
#[no_mangle]
pub extern "C" fn calc(a: i32, b: i32) -> i32
{
    println!("Calculating from rust...");
    return a + b;
}
