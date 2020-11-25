fn main() {
    println!("Starting");
    let mut data: Vec<u8> = Vec::new();
    loop {
        data.push(0);
        if data.len() % 1_000_000 == 0 {
            println!("{} bytes on stack (pid: {})", data.len(), std::process::id());
        }
    }
}
