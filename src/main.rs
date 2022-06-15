use clap::Parser;
extern crate colored;
use colored::*;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // Name
    #[clap(short, long, value_parser)]
    name: String,
    // Languages
    #[clap(short, long, value_parser)]
    language: String,
    // Path
    #[clap(short, long, value_parser)]
    path: Option<std::path::PathBuf>,
    // Verbose
    #[clap(short, long, value_enum)]
    verbose: Bool,
    // Git
    #[clap(short, long, value_enum)]
    git: Bool,
    #[clap(short, long, value_enum)]
    code: Bool,
}

impl Args {
    fn new(
        name: &str,
        language: &str,
        path: Option<std::path::PathBuf>,
        git: Bool,
        verbose: Bool,
        code: Bool,
    ) -> Args {
        Args {
            name: name.to_string(),
            language: language.to_string(),
            path: path,
            git: git,
            verbose: verbose,
            code: code,
        }
    }
}

impl Clone for Args {
    fn clone(&self) -> Args {
        Args::new(
            &self.name,
            &self.language,
            self.path.clone(),
            self.git,
            self.verbose,
            self.code,
        )
    }
}

#[derive(clap::ValueEnum, Clone, Debug, Copy)]
enum Bool {
    True = 0,
    False = 1,
}

impl Bool {
    fn to_bool(&self) -> bool {
        match self {
            Bool::True => true,
            Bool::False => false,
        }
    }
}

#[cfg(target_os = "windows")]
static PATH: &str = &("C:\\Users\\{}\\AppData\\Roaming\\GitHub CLI\\hosts.yml");
#[cfg(target_os = "windows")]
static CODE: &str = "code.cmd";
#[cfg(target_os = "linux")]
static PATH: &str = "~/.config/gh/hosts.yml";
#[cfg(target_os = "linux")]
static CODE: &str = "code";
#[cfg(target_os = "macos")]
static PATH: &str = "~/.config/gh/hosts.yml";
#[cfg(target_os = "macos")]
static CODE: &str = "code";

fn git(args: Args) {
    use std::io::Read;
    let (tx, rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();
    let (tx2, rx2): (Sender<bool>, Receiver<bool>) = mpsc::channel();
    std::thread::spawn(move || print_flush("Creating git repository", rx, tx2));
    std::process::Command::new("git")
        .current_dir(args.path.as_ref().unwrap().join(args.name.as_str()))
        .arg("init")
        .output()
        .expect("failed to execute process");
    std::process::Command::new("gh")
        .current_dir(args.path.as_ref().unwrap().join(args.name.as_str()))
        .arg("repo")
        .arg("create")
        .arg(args.name.as_str())
        .arg("--private")
        .output()
        .expect("failed to execute process");
    let mut f = std::fs::File::open(PATH.replace("{}", &whoami::username())).unwrap();
    let mut contents = String::new();
    let _ = f.read_to_string(&mut contents);
    let s: Vec<&str> = contents.split(" ").collect::<Vec<&str>>();
    let mut ans: String = String::new();
    for i in 0..s.len() {
        if s[i] == "user:" {
            ans = s[i + 1].trim().replace("\n", "");
        }
    }
    std::process::Command::new("git")
        .current_dir(args.path.as_ref().unwrap().join(args.name.as_str()))
        .arg("add")
        .arg(".")
        .output()
        .expect("failed to execute process");
    std::process::Command::new("git")
        .current_dir(args.path.as_ref().unwrap().join(args.name.as_str()))
        .arg("branch")
        .arg("-M")
        .arg("main")
        .output()
        .expect("failed to execute process");
    std::process::Command::new("git")
        .current_dir(args.path.as_ref().unwrap().join(args.name.as_str()))
        .arg("remote")
        .arg("add")
        .arg("origin")
        .arg(
            "https://github.com/()/{}.git"
                .replace("()", ans.as_str())
                .replace("{}", args.name.as_str()),
        )
        .output()
        .expect("failed to execute process");
    std::process::Command::new("git")
        .current_dir(args.path.as_ref().unwrap().join(args.name.as_str()))
        .arg("commit")
        .arg("-m")
        .arg("Initial Commit")
        .output()
        .expect("failed to execute process");
    std::process::Command::new("git")
        .current_dir(args.path.as_ref().unwrap().join(args.name.as_str()))
        .arg("push")
        .arg("-u")
        .arg("origin")
        .arg("main")
        .output()
        .expect("failed to execute process");
    tx.clone().send(true).unwrap();
    rx2.recv().unwrap();
    println!(
        "Git repository created in https://github.com/{}/{}",
        ans.green(),
        args.name.as_str().blue().italic()
    );
}

fn main() {
    let args = Args::parse();

    println!(
        "Creating Project {} in {}",
        args.name.green(),
        args.language.blue().italic()
    );

    match args.language.as_str() {
        "rust" => {
            let (tx, rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();
            let (tx2, rx2): (Sender<bool>, Receiver<bool>) = mpsc::channel();
            std::thread::spawn(move || print_flush("Running cargo", rx, tx2));
            std::process::Command::new("cargo")
                .current_dir(args.path.as_ref().unwrap())
                .arg("new")
                .arg(args.name.as_str())
                .output()
                .expect("failed to execute process");
            std::fs::write(
                &args
                    .path
                    .as_ref()
                    .unwrap()
                    .join(args.name.as_str())
                    .join("README.md"),
                include_str!("../templates/README.md")
                    .replace("()", args.language.as_ref())
                    .replace("{}", args.name.as_str()),
            )
            .expect("failed to write to file");
            tx.clone().send(true).unwrap();
            rx2.recv().unwrap();
            if args.git.to_bool() {
                git(args.clone());
            }
        }
        "python" => {}
        "c++" => {
            let (tx, rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();
            let (tx2, rx2): (Sender<bool>, Receiver<bool>) = mpsc::channel();
            std::thread::spawn(move || print_flush("Creating dir", rx, tx2));
            use std::fs::File;
            std::fs::create_dir(args.path.as_ref().unwrap().join(args.name.as_str()))
                .expect("failed to create directory");
            tx.clone().send(true).unwrap();
            rx2.recv().unwrap();
            let (tx, rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();
            let (tx2, rx2): (Sender<bool>, Receiver<bool>) = mpsc::channel();
            std::thread::spawn(move || print_flush("Creating files", rx, tx2));
            _ = File::create(
                &args
                    .path
                    .as_ref()
                    .unwrap()
                    .join(args.name.as_str())
                    .join("main.cpp"),
            )
            .expect("failed to create file");
            std::fs::write(
                &args
                    .path
                    .as_ref()
                    .unwrap()
                    .join(args.name.as_str())
                    .join("main.cpp"),
                include_str!("../templates/main.cpp").as_bytes(),
            )
            .expect("failed to write to file");
            std::fs::write(
                &args
                    .path
                    .as_ref()
                    .unwrap()
                    .join(args.name.as_str())
                    .join("README.md"),
                include_str!("../templates/README.md")
                    .replace("()", args.language.as_ref())
                    .replace("{}", args.name.as_str()),
            )
            .expect("failed to write to file");
            tx.clone().send(true).unwrap();
            rx2.recv().unwrap();
            if args.git.to_bool() {
                git(args.clone());
            }
        }
        "go" => {
            let (tx, rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();
            let (tx2, rx2): (Sender<bool>, Receiver<bool>) = mpsc::channel();
            std::thread::spawn(move || print_flush("Creating dir", rx, tx2));
            use std::fs::File;
            std::fs::create_dir(args.path.as_ref().unwrap().join(args.name.as_str()))
                .expect("failed to create directory");
            tx.clone().send(true).unwrap();
            rx2.recv().unwrap();
            let (tx, rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();
            let (tx2, rx2): (Sender<bool>, Receiver<bool>) = mpsc::channel();
            std::thread::spawn(move || print_flush("Creating files", rx, tx2));
            _ = File::create(
                &args
                    .path
                    .as_ref()
                    .unwrap()
                    .join(args.name.as_str())
                    .join("main.go"),
            )
            .expect("failed to create file");
            std::fs::write(
                &args
                    .path
                    .as_ref()
                    .unwrap()
                    .join(args.name.as_str())
                    .join("main.go"),
                (include_str!("../templates/main.go").replace("{}", args.name.as_str())).as_bytes(),
            )
            .expect("failed to write to file");
            std::fs::write(
                &args
                    .path
                    .as_ref()
                    .unwrap()
                    .join(args.name.as_str())
                    .join("README.md"),
                include_str!("../templates/README.md")
                    .replace("()", args.language.as_ref())
                    .replace("{}", args.name.as_str()),
            )
            .expect("failed to write to file");
            std::process::Command::new("go")
                .current_dir(args.path.as_ref().unwrap().join(args.name.as_str()))
                .arg("mod")
                .arg("init")
                .arg(args.name.as_str())
                .output()
                .expect("failed to execute process");
            tx.clone().send(true).unwrap();
            rx2.recv().unwrap();
            if args.git.to_bool() {
                git(args.clone());
            }
        }
        _ => {
            println!("{}", "Unknown language".red());
            return;
        }
    }
    if args.code.to_bool() {
        std::process::Command::new(CODE)
            .args(&args.path.as_ref().unwrap().join(args.name.as_str()))
            .output()
            .expect("failed to execute process");
    }
    println!(
        "Project {} created in {}",
        args.name.green(),
        args.path.unwrap().to_str().unwrap().italic()
    );
}
use crossterm::{
    cursor::{position, Hide, MoveTo, Show},
    execute, terminal,
};
use std::io::{stdout, Write};
fn print_flush(s: &str, rx: std::sync::mpsc::Receiver<bool>, tx: std::sync::mpsc::Sender<bool>) {
    let (a, b) = position().unwrap();
    execute!(stdout(), Hide).unwrap();
    print!("{}", s.italic());
    stdout().flush().unwrap();
    loop {
        if rx.try_recv() == Ok(true) {
            execute!(
                stdout(),
                terminal::Clear(terminal::ClearType::CurrentLine),
                MoveTo(a, b),
                Show
            )
            .unwrap();
            tx.send(false).unwrap();
            break;
        }
        for _ in 0..3 {
            std::thread::sleep(std::time::Duration::from_millis(500));
            stdout().write(".".as_bytes()).unwrap();
            stdout().flush().unwrap();
        }
        stdout()
            .write("\x08\x08\x08   \x08\x08\x08".as_bytes())
            .unwrap();
        stdout().flush().unwrap();
    }
}
