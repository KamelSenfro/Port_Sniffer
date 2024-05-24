use bpaf::Bpaf;
use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;

// Max IP Port.
const MAX: u16 = 65535;

// Address fallback.
const IPFALLBACK: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

// CLI Arguments.
#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub struct Arguments {
    // Address argument.  Accepts -a and --address and an IpAddr type. Falls back to the above constant.
    #[bpaf(long, short, argument("Address"), fallback(IPFALLBACK))]
    /// The address that you want to sniff.  Must be a valid ipv4 address.  Falls back to 127.0.0.1
    pub address: IpAddr,
    #[bpaf(
        long("start"),
        short('s'),
        guard(start_port_guard, "Must be greater than 0"),
        fallback(1u16)
    )]
    /// The start port for the sniffer. (must be greater than 0)
    pub start_port: u16,
    #[bpaf(
        long("end"),
        short('e'),
        guard(end_port_guard, "Must be less than or equal to 65535"),
        fallback(MAX)
    )]
    /// The end port for the sniffer. (must be less than or equal to 65535)
    pub end_port: u16,
    #[bpaf(long, short, argument("Timeout"), fallback(Duration::from_secs(1)))]
    /// The timeout for each connection attempt. (must be greater than 0)
    pub timeout: Duration,
    #[bpaf(long, short, argument("Threads"), fallback(10))]
    /// The number of concurrent threads to use for scanning. (must be greater than 0)
    pub threads: usize,
}

fn start_port_guard(input: &u16) -> bool {
    *input > 0
}

fn end_port_guard(input: &u16) -> bool {
    *input <= MAX
}

// Scan the port.
fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, timeout: Duration) {
    // Attempts Connects to the address and the given port.
    match TcpStream::connect_timeout(&format!("{}:{}", addr, start_port), timeout) {
        // If the connection is successful, print out a . and then pass the port through the channel.
        Ok(_) => {
            print!(".");
            io::stdout().flush().unwrap();
            tx.send(start_port).unwrap();
        }
        // If the connection is unsuccessful, do nothing. Means port is not open.
        Err(_) => {}
    }
}

fn main() {
    // collect the arguments.
    let opts = arguments().run();

    // Initialize the channel.
    let (tx, rx) = channel();

    // Create a thread pool for scanning.
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(opts.threads)
        .build()
        .unwrap();

    // Iterate through all of the ports (based on user input) and spawn a thread for each.
    for i in opts.start_port..opts.end_port {
        let tx = tx.clone();

        pool.spawn(move || scan(tx, i, opts.address, opts.timeout));
    }

    // Create the vector for all of the outputs.
    let mut out = vec![];

    // Drop the tx clones.
    drop(tx);

    // Wait for all of the outputs to finish and push them into the vector.
    for p in rx {
        out.push(p);
    }

    println!("");
    out.sort();
    for v in out {
        // Iterate through the outputs and print them out as being open.
        println!("{} is open", v);
    }
}
