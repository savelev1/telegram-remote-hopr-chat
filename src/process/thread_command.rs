pub struct ThreadCommand {
    pub action: ThreadCommandAction,
    pub data: String,
}

pub enum ThreadCommandAction {
    Terminate,
    Error,
    Stdin,
    Stdout,
    Stderr,
    Exit,
}

impl ThreadCommand {
    pub(crate) fn new(action: ThreadCommandAction, data: String) -> ThreadCommand {
        ThreadCommand {
            action,
            data,
        }
    }
}
