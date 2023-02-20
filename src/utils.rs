pub const VERSION: i8 = 1;
pub const DIFFICULTY: u32 = 4;

pub fn print_bytes(bytes: &[u8]) {
    for b in bytes {
        print!("{:02X} ", b);
    }
    println!("");
}