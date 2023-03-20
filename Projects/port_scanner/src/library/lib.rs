#[allow(unused_imports)]
use std::io::{Error, ErrorKind, Read};
use std::net::{Shutdown, TcpStream};
use colored::Colorize;
pub struct Target {
    pub ip: String,
    pub start: u16,
    pub finish: u16,
}
impl Default for Target {
    fn default() -> Target {
        Target {
            ip: String::from("127.0.0.1"),
            start: 1,
            finish: 65535,
        }
    }
}

impl Target {
    pub fn parse_target(address: &str, range: &str) -> Target {
        match range {
            //Check to see if the range is "all"
            "all" => Target {
                ip: String::from(address),
                start: 1,
                finish: 65535,
            },
            //Check for a defined range of ports
            range if range.contains("-") => {
                let range: Vec<&str> = range.split("-").collect();
                let start = range[0].parse::<u16>().unwrap();
                let finish = range[1].parse::<u16>().unwrap();
                Target {
                    ip: String::from(address),
                    start,
                    finish,
                }
            }
            //If the range is not "all", a range of ports, or a single port, then return the default target
            _ => {
                //Check to see if the range is a single port
                if range.parse::<u16>().is_ok() {
                    let port = range.parse::<u16>().unwrap();
                    Target {
                        ip: String::from(address),
                        start: port,
                        finish: port,
                    }
                //If the range is not a single port and it doesn't match any of the other cases, then return the default target
                } else {
                    println!("{}","[!] Invalid port range. Using 1-65535".red().italic());
                    Target::default()
                }
            }
        }
    }
}

pub fn scan(target: Target) {
    let mut open_ports: Vec<u16> = Vec::new();
    for port in target.start..=target.finish {
        let stream = TcpStream::connect(format!("{}:{}", target.ip, port));
        match stream {
            Ok(stream) => {

                //Non functional code to read a ports banner.

                // let mut banner: Vec<u8> = vec![0; 1024];

                // let mut rx_bytes = [0u8; 1024];
                
                // let bytes_read = stream.read(&mut rx_bytes).unwrap();

                // if bytes_read == 0 {
                //     break;
                // }
                // banner.extend_from_slice(&rx_bytes);

                // String::from_utf8(banner)
                //     .map(|msg| println!("{}", msg))
                //     .map_err(|_| {
                //         Error::new(
                //             ErrorKind::InvalidData,
                //             "Couldn't parse received string as utf8",
                //         )
                //     })
                //     .ok();

                println!("{} {}: Open","[+]".green(), port);
                stream.shutdown(Shutdown::Both).unwrap();
                open_ports.push(port);
            }
            Err(_) => {
                ();
            }
        }
    }
    println!("{}{}","[*] Total open Ports: ".green(), open_ports.len());
}
