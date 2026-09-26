use std;

mod process;
mod cli;
mod os;
use process::process::call_create;
fn main() {
    call_create();
    
}
