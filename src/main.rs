// use std::path::PathBuf;
use juan::data::Event;
use std::fs;
use std::io::{self, Read, Write};
use std::num::ParseIntError;
use structopt::StructOpt;

fn parse_hex(input: &str) -> Result<u64, ParseIntError> {
    u64::from_str_radix(input, 16)
}

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short, long, help = "Activate quite mode")]
    quiet: bool,
    #[structopt(short, long, help = "Activate verbose mode")]
    verbose: bool,
    #[structopt(long)]
    color: bool,
    #[structopt(subcommand)]
    cmd: Juan,
}

#[derive(StructOpt, Debug)]
#[structopt()]
enum Juan {
    Add {
        time: String,
        title: String,
        priority: Option<u8>,
    },
    Init,
    List,
    Finish {
        #[structopt(parse(try_from_str = parse_hex))]
        id: u64,
    },
}

fn main() {
    let opt = Opt::from_args();

    // let mut line = String::new();
    match opt.cmd {
        Juan::Add { ref time, ref title, priority } => {
            add_com(&time, &title, priority);
        }
        Juan::Init => {
            init_com();
        }
        Juan::List => {
            list_com();
        }
        Juan::Finish { id } => {
            finish_com(id);
        }
        _ => println!("ELSE!"),
    }
    // println!("{:?}", opt);
}

fn get_data() -> Vec<Event> {
    let mut file = fs::OpenOptions::new()
        .read(true)
        .create(true)
        .write(true)
        .open("data.yaml")
        .unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    serde_yaml::from_str(&contents).unwrap()
}

fn write_data(data_set: Vec<Event>) {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("data.yaml")
        .unwrap();

    let serialized = serde_yaml::to_string(&data_set).unwrap();
    file.write(serialized.as_bytes()).unwrap();
}

fn add_com(time: &str, title: &str, priority: Option<u8>) {
    let mut data_set: Vec<Event> = get_data();

    let event = Event::from_info(time, title, priority.unwrap_or(2));
    println!("One event added:\n{}", event);
    data_set.push(event);
    data_set = sort_data(data_set);

    write_data(data_set);
}

fn init_com() {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("data.yaml")
        .unwrap();
    let data_set: Vec<Event> = Vec::new();
    let serialized = serde_yaml::to_string(&data_set).unwrap();
    file.write(serialized.as_bytes()).unwrap();
}

fn list_com() {
    let data_set: Vec<Event> = get_data();

    println!("You have {} event(s) in total:", data_set.len());
    for e in &data_set {
        println!("{}", e);
    }
}

fn finish_com(id: u64) {
    let mut data_set: Vec<Event> = get_data();

    data_set.retain(|e| e.calculate_hash() != id);
    data_set.retain(|e| (e.calculate_hash() >> 48) != id);
    println!("Congratulations!");
    println!("{} event(s) left.", data_set.len());

    write_data(data_set);
}

fn sort_data(mut data_set: Vec<Event>) -> Vec<Event> {
    data_set.sort();
    data_set
}