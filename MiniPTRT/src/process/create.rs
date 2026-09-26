//should create a child process by invoking fork() system caall
use nix::unistd::{fork, ForkResult};

pub fn create_process() {
match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            println!("Hello from the child process!");
        }
        Ok(ForkResult::Parent { child }) => {
            println!("Hello from the parent process, child PID is {}.", child);
        }
        Err(_) => eprintln!("Fork failed"),
    }
}