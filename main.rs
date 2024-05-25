use bpaf::Bpaf;
use std::fs::File;
use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr, TcpStream};
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
    #[bpaf(long, short, argument("Address"), fallback(IPFALLBACK))]
    /// The address that you want to sniff. Must be a valid ipv4 address. Falls back to 127.0.0.1
    pub address: IpAddr,

    #[bpaf(long("start"), short('s'), guard(start_port_guard, "Must be greater than 0"), fallback(1u16))]
    /// The start port for the sniffer. (must be greater than 0)
    pub start_port: u16,

    #[bpaf(long("end"), short('e'), guard(end_port_guard, "Must be less than or equal to 65535"), fallback(MAX))]
    /// The end port for the sniffer. (must be less than or equal to 65535)
    pub end_port: u16,

    #[bpaf(long, short, argument("Timeout"), fallback(Duration::from_secs(1)))]
    /// The timeout for each connection attempt. (must be greater than 0)
    pub timeout: Duration,

    #[bpaf(long, short, argument("Threads"), fallback(10))]
    /// The number of concurrent threads to use for scanning. (must be greater than 0)
    pub threads: usize,

    #[bpaf(long, short)]
    /// Enable verbose output
    pub verbose: bool,

    #[bpaf(long, short, argument("Output File"))]
    /// Output file to save the results
    pub output: Option<String>,

    #[bpaf(long, short, argument("Protocol"), fallback("tcp".to_string()))]
    /// The protocol to use for scanning (tcp or udp)
    pub protocol: String,
}

fn start_port_guard(input: &u16) -> bool {
    *input > 0
}

fn end_port_guard(input: &u16) -> bool {
    *input <= MAX
}

// Scan the port.
fn scan(tx: Sender<u16>, port: u16, addr: IpAddr, timeout: Duration, verbose: bool) {
    // Attempts Connects to the address and the given port.
    match TcpStream::connect_timeout(&format!("{}:{}", addr, port).parse().unwrap(), timeout) {
        Ok(_) => {
            if verbose {
                println!("Port {} is open", port);
            } else {
                print!(".");
                io::stdout().flush().unwrap();
            }
            tx.send(port).unwrap();
        }
        Err(_) => {
            if verbose {
                println!("Port {} is closed", port);
            }
        }
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
    for port in opts.start_port..=opts.end_port {
        let tx = tx.clone();
        let addr = opts.address;
        let timeout = opts.timeout;
        let verbose = opts.verbose;

        pool.spawn(move || scan(tx, port, addr, timeout, verbose));
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

    // Save results to a file if the output file option is specified.
    if let Some(output_file) = opts.output {
        let mut file = File::create(output_file).expect("Unable to create file");
        for v in out {
            writeln!(file, "{} is open", v).expect("Unable to write to file");
        }
    }
}
