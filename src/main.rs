extern crate rand;
// #[macro_use]
extern crate structopt;
extern crate reqwest;

use rand::prelude::*;
// use std::error::Error;
// use std::io::{self,Read,Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::process;
use std::thread::sleep;
use std::time::Duration;
// use std::{thread, time};
// TODO: load up the filesystem -- test space allocation and I/O utilization
// use std::fs::File;
// use std::io::BufReader;
// use std::path::PathBuf;
// use std::time::Duration;
// TODO: multicore load
// use std::thread;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "chaosmarmoset")]
struct Opt {
    /// Activate debug mode (TODO: actually emit some stuff)
    #[structopt(short = "d", long = "debug")]
    debug: bool,

    /// mode: CPU, Disk, Network Sink, Web Client, Memory, or Logspam
    #[structopt(raw(required = "true"), short = "m", long = "mode")]
    mode: String,

    /// Memory usage in MB (required only for set-memory mode)
    #[structopt(short = "u", long = "usage")]
    usage: Option<u32>,

    /// Network port, for network modes
    #[structopt(short = "p", long = "port")]
    port: Option<u16>,

    /// URL to fetch, in web-client mode
    #[structopt(short = "U", long = "url")]
    url: Option<String>,

    /// Sleep in milliseconds(between fetches for web client; between logs for logspammer)
    #[structopt(short = "s", long = "sleep")]
    sleep: Option<u64>,
}

fn main() {
    let opt = Opt::from_args();
    let mode = opt.mode;

    if mode == "cpu" {
        // Max one CPU core
        // TODO: max more than one core
        load_cpu();
        process::exit(0);
    } else if mode == "max-memory" {
        // Just keep allocating memory till we die
        // TODO: allocate more slowly
        use_memory_quickly();
        process::exit(0);
    } else if mode == "set-memory" {
        // Allocate a fixed amount of memory and sleep forever
        let usage = opt.usage;
        match usage {
            Some(m) => use_fixed_memory(m),
            None    => panic!("Specify memory usage in MB, e.g. --usage 1000"),
        }
        process::exit(0);
    } else if mode == "network-sink" {
        // Allocate a fixed amount of memory and sleep forever
        let port = opt.port;
        match port {
            Some(p) => network_sink_listen(p),
            None    => panic!("Specify a listen port, e.g. --port 443"),
        }
        process::exit(0);
    } else if mode == "web-client" {
        // Periodically request the given URL, print number of lines in response body
        let url = opt.url;
        let sleepms;
        match opt.sleep {
            Some(ms) => sleepms = Duration::from_millis(ms),
            None     => sleepms = Duration::from_millis(0),
        }
        // FIXME: not the right way to do this!
        let x;
        match url {
            Some(u) => x = web_client(u, sleepms),
            None    => panic!("Specify a URL, e.g. --url http://www.didyoutryturningitoffandon.com/"),
        }
        process::exit(0);
    } else if mode == "log-spam" {
        // Spam stdout.  Did you know that EKS didn't used to rotate logfiles by default?
        // No?  Neither did I!!!
        let sleepms;
        match opt.sleep {
            Some(ms) => sleepms = Duration::from_millis(ms),
            None     => sleepms = Duration::from_millis(0),
        }
        spam_stdout(sleepms);
    } else {
        // TODO: fill up the disk
        // TODO: thrash the disk (lots of reads, or writes)
        // TODO: be a server or client, with variable performance.
        println!("Unfortunately '{}' is not (yet) implemented.", mode);
        process::exit(1);
    }
}

fn count_newlines(s: &str) -> usize {
    s.as_bytes().iter().filter(|&&c| c == b'\n').count()
}

fn load_cpu() {
    println!("OK trying to cook one CPU core...");
    let mut i: u32 = 0;
    loop {
        i = i + 1;
        if i >= (std::u32::MAX - 1) {
            println!("loop");
            i = 0;
        }
    }
}

// Loops forever, aggressively allocating more and more memory.
fn use_memory_quickly() {
    println!("Beware, this chews up your memory pretty quick...");
    let mut v: Vec<u32> = vec![];
    let mut i: u32 = 0;
    loop {
        i = i + 1;
        // Rough estimage of memory used based on 4 bytes per u32 entry -- prints every 400 MB
        if i % (100*(1<<20)) == 0 {
            println!("{}MB...", i / (1<<20) * 4);
        }
        v.push(i);
        if i >= std::u32::MAX {
            i = 0;
        }
    }
}

// Allocate a fixed amount of memory and then just sit around accessing it
// occasionally.
// FIXME: I couldn't find a way to just grab a big chunk of heap all in one go.
fn use_fixed_memory(usage: u32) {
    println!("Allocating: {}MB of memory", usage);
    let len:usize = (usage as usize * (1<<20) / 8) as usize;
    let mut buffer: Vec<u64> = Vec::with_capacity(len);
    for i in 0..len {
        buffer.push(i as u64);
    }
    println!("Allocated.");
    // Just referencing random items in the vector to help keep it
    // "hot" and out of swap.
    // TODO: no idea if this is effective or not.
    let mut rng = thread_rng();
    loop {
        sleep(Duration::new(1, 0));
        let idx:usize = rng.gen_range(0, len-1);
        println!("{}", buffer[idx]);
    }
}

fn network_sink_listen(port: u16) {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(&addr).unwrap();
    // TODO: handle this.
    listener.take_error().expect("Error binding port.");
    // Warning: this is processing serially, no fancy multi-stream, async etc yet.
    for stream in listener.incoming() {
        network_sink_client(stream.unwrap());
    }
}

// WARNING: untested and probably not working.
// fn network_sink_client(stream: TcpStream) -> Result<(), Error> {
fn network_sink_client(mut stream: TcpStream) {
    std::io::copy(&mut stream, &mut std::io::stdout());
    // TODO: maybe read into a fixed buffer and ignore...
}

// fn web_client(url: String, sleepms: Duration) {
fn web_client(url: String, sleepms: Duration) -> Result<(), Box<std::error::Error>> {
    let mut headers = reqwest::header::HeaderMap::new();
    // Added for testing one specific app which needs it
    // TODO: make this another commandline option
    headers.insert(reqwest::header::ACCEPT, "text/plain;version=0.0.4".parse().unwrap());
    let client = reqwest::Client::builder()
        .gzip(true)
        .default_headers(headers)
        .http1_title_case_headers()
        .build()?;
    loop {
        let body = client.get(&url).send()?.text()?;
        let count = count_newlines(&body);
        println!("{:#?}", count);
        println!("{:#?}", body);
        sleep(sleepms);
    }
}

fn spam_stdout(sleepms: Duration) {
    loop {
        println!("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Dolor sed viverra ipsum nunc aliquet bibendum enim. In massa tempor nec feugiat. Nunc aliquet bibendum enim facilisis gravida. Nisl nunc mi ipsum faucibus vitae aliquet nec ullamcorper. Amet luctus venenatis lectus magna fringilla. Volutpat maecenas volutpat blandit aliquam etiam erat velit scelerisque in. Egestas egestas fringilla phasellus faucibus scelerisque eleifend. Sagittis orci a scelerisque purus semper eget duis. Nulla pharetra diam sit amet nisl suscipit. Sed adipiscing diam donec adipiscing tristique risus nec feugiat in. Fusce ut placerat orci nulla. Pharetra vel turpis nunc eget lorem dolor. Tristique senectus et netus et malesuada.");
        sleep(sleepms);
    }
}
