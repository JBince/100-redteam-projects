mod library;
use library::lib;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <ip> <range>", args[0]);
        println!("\tIP: The IP address to scan. Expressed as IPv4");
        println!("\tRange: The range of ports to scan. Expressed as a single port, a range of ports, or 'all' to scan all ports");
        println!("\tExample: {} 192.168.145.1 1-100", args[0]);
        std::process::exit(1);
    }
    let target = lib::Target::parse_target(&args[1], &args[2]);
    lib::scan(target);
}
