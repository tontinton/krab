extern crate tokio;


use nix::unistd::{getpid, setsid, dup2, close};
use nix::fcntl::{open, OFlag};
use nix::sys::stat::{Mode};


#[tokio::main]
async fn main() {
    let pid = getpid();

    if pid.as_raw() != 1 {
        println!("Can only run as PID 1, but my pid is: {}", pid);
        std::process::exit(1);
    }

    if let Err(e) = setsid() {
        println!("Failed creating a session, {}", e);
        std::process::exit(1);
    }

    let fd = match open("/dev/console",
                        OFlag::O_WRONLY | OFlag::O_NDELAY,
                        Mode::empty()) {
        Ok(fd) => fd,
        Err(e) => {
            println!("Failed opening /dev/console, {}", e);
            std::process::exit(1);
        },
    };

    if let Err(e) = dup2(fd, 0) {
        println!("Failed duplicating the console fd to stdin, {}", e);
        std::process::exit(1);
    }

    if let Err(e) = dup2(fd, 1) {
        println!("Failed duplicating the console fd to stdout, {}", e);
        std::process::exit(1);
    }

    if let Err(e) = dup2(fd, 2) {
        println!("Failed duplicating the console fd to stderr, {}", e);
        std::process::exit(1);
    }

    if fd > 2 {
        if let Err(e) = close(fd) {
            println!("Failed closing the console fd, {}", e);
        }
    }

    println!("Hello linux!");
}
