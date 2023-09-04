use clap::Parser;
use std::io::{self, Write};
use std::{
    net::{IpAddr, TcpStream},
    sync::mpsc::{channel, Sender},
    thread,
};

const MAX: u16 = 65535;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// The IPaddress
    #[arg(short, long)]
    ipaddr: IpAddr,

    /// How many threads
    #[arg(short, long, default_value_t = 4)]
    threads: u16,
}

fn main() {
    let args = Arguments::parse();

    let (tx, rx) = channel();
    for i in 0..args.threads {
        let tx = tx.clone();
        thread::spawn(move || {
            send(tx, i, args.ipaddr, args.threads);
        });
    }

    drop(tx);

    let mut out: Vec<String> = vec![];
    for p in rx {
        out.push(p);
    }

    println!("");
    out.sort();
    for v in out {
        println!("{} is open", v);
    }
}

fn send(tx: Sender<String>, start_port: u16, ipaddr: IpAddr, num_threads: u16) {
    let mut port = start_port + 1;
    loop {
        match TcpStream::connect((ipaddr, port)) {
            Ok(_) => {
                print!(".");
                // this is important so we can use IO inside our thread
                // this allows us to constructs a handle to the standard output
                // of the current process, by calling flush it sends all these print statements
                // to what is essentially a mutex of shared data
                io::stdout().flush().unwrap();
                tx.send(port.to_string()).unwrap();
            }
            Err(_) => {}
        }

        if (MAX - port) <= num_threads {
            break;
        }

        port += num_threads;
    }
}
