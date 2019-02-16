use std::io::Write;
use std::net::TcpListener;

use std::env;
use std::sync::Arc;
use std::thread;

struct IrcServer {
    resp: Vec<u8>,
}

unsafe impl Send for IrcServer {}

impl IrcServer {
    fn new(dns: &str, port: &str) -> IrcServer {
        let mut resp = Vec::new();
        write!(
            &mut resp,
            ":{} ERROR Please use TLS (SSL) to connect to this IRC network on port {}\r\n",
            dns, port
        )
        .expect("couldn't create response from dns/port"); // how could this error ever happen?
        IrcServer { resp }
    }
}

struct Args<'a> {
    args: &'a Vec<String>,
}

impl<'a> Args<'a> {
    fn new(args: &'a Vec<String>) -> Args {
        Args { args }
    }
    fn get_str(&self, index: usize, def: &'a str) -> &'a str {
        match self.args.get(index) {
            Some(ret) => ret,
            None => def,
        }
    }
}

fn main() {
    let raw_args = env::args().collect();
    let args = Args::new(&raw_args);
    if args.get_str(1, "").contains("-h") {
        println!(
            "usage: {} [-h] [host, [::]:6667] [port, 6697] [dns, irc.local]",
            args.get_str(0, "secureircd")
        );
        return;
    }
    let host = args.get_str(1, "[::]:6667");

    let irc_server = Arc::new(IrcServer::new(
        args.get_str(3, "irc.local"),
        args.get_str(2, "6697"),
    ));

    let listener = TcpListener::bind(&host).expect("could not bind to host");
    println!("Listening for connections on {}", &host);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let irc_server = irc_server.clone();
                thread::spawn(move || {
                    stream
                        .write(&irc_server.resp)
                        .expect("error handling connection")
                });
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}
