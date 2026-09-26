use create::create_process;

//reference to the actual process running an abstraction of it
use super::create;
pub enum ProcessState {
    Running,
    Sleeping,
    Zombie,
}

pub struct Process {
    pub pid: u64,
    pub uid: u64,
    pub state: ProcessState,
}

pub fn call_create(){
    create_process();
}