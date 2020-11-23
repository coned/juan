// use std::path::PathBuf;
use juan::data::Event;
use std::fs;
use std::io::{self, Read, Write};
use structopt::StructOpt;

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
    Add,
    Init,
    List,
    // Fetch {
    //     #[structopt(long)]
    //     dry_run: bool,
    //     #[structopt(long)]
    //     all: bool,
    //     repository: Option<String>,
    // },
    // Commit {
    //     #[structopt(short)]
    //     message: Option<String>,
    //     #[structopt(short)]
    //     all: bool,
    // },
}

fn main() {
    let opt = Opt::from_args();

    // let mut line = String::new();
    match opt.cmd {
        Juan::Add => {
            add_com();
        },
        Juan::Init => {
            init_com();
        },
        Juan::List => {
            list_com();
        },
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
    file.read_to_string(&mut contents);

    serde_yaml::from_str(&contents).unwrap()
}

fn write_data(data_set: Vec<Event>) {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("data.yaml")
        .unwrap();

    let serialized = serde_yaml::to_string(&data_set).unwrap();
    file.write(serialized.as_bytes()).unwrap();
}

fn add_com() {
    let mut data_set: Vec<Event> = get_data();

    let mut time = String::new();
    print!("time (today): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut time).unwrap();

    let mut title = String::new();
    print!("title: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut title).unwrap();

    let event = Event::from_str(&time, &title);
    println!("{}", event);
    data_set.push(event);

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
    let mut data_set: Vec<Event> = get_data();

    for e in &data_set {
        println!("{}", e);
    }
}
