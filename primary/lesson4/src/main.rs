use std::fmt;
use std::fs;
use std::path::PathBuf;
pub use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "SearchString",
    about = "search wheather a file contains certain string or plainly print file",
    version = "0.0.1"
)]
pub struct Opts {
    #[structopt(subcommand)]
    commands: Option<App>,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "app")]
enum App {
    #[structopt(name = "find")]
    Find(FindOpt),

    #[structopt(name = "show")]
    Show(ShowOpt),
}

#[derive(StructOpt, Debug)]
#[structopt(about = "search string in a file")]
struct FindOpt {
    #[structopt(short, long, long_help = "aim file", required = true)]
    file: PathBuf,

    #[structopt(short, long, long_help = "search string", required = true)]
    search: String,
}

impl fmt::Display for FindOpt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "file={:?}, search={}", self.file, self.search)
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "print file")]
struct ShowOpt {
    #[structopt(short, long, long_help = "aim file", required = true)]
    file: PathBuf,
}

impl fmt::Display for ShowOpt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "show file {:?}", self.file)
    }
}

fn main() {
    let opt = Opts::from_args();
    handle_subcommand(opt);
}

fn handle_subcommand(opt: Opts) {
    // handle subcommands
    if let Some(subcommand) = opt.commands {
        match subcommand {
            App::Find(cfg) => {
                println!("{}", cfg);
                // 重新赋值使得所有权转移
                let file = cfg.file;
                let search = cfg.search;

                println!("find {} in file {:?}", search, file);

                let result = search_string(file, search);
                match result {
                    true => println!("find it"),
                    false => println!("not find it"),
                }
            }
            App::Show(cfg) => {
                println!("{}", cfg);
                print_file(cfg.file);
            }
        }
    } else {
        println!("no input argument, please check help document");
    }
}

fn search_str(query: &str, contents: &str) -> bool {
    for line in contents.lines() {
        if line.contains(query) {
            return true;
        }
    }
    return false;
}

fn search_string(file: PathBuf, search: String) -> bool {
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");
    search_str(&search, &contents)
}

fn print_file(file: PathBuf) {
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");
    // 使用迭代器
    for (index, line) in contents.lines().enumerate() {
        println!("{}. {}", index + 1, line);
    }
}
