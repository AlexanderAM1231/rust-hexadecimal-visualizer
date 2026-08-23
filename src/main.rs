fn hex_dump(data: &[u8]) {
    for (i, chunk) in data.chunks(16).enumerate() {
        print!("{:08x}: ", i * 16);
        for byte in chunk {
            print!("{:02x} ", byte);
        }
        println!();
    }
}

fn main() {
    let sample = b"Hello, Binary Hex Dump World!";
    hex_dump(sample);
}
