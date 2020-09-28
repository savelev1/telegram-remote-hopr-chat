use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::process::ChildStdin;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::thread;

use tokio::time::Duration;

use crate::process::thread_command::{ThreadCommand, ThreadCommandAction};

pub mod thread_command;

pub struct Process {
    command_text: String,
    sender: Sender<ThreadCommand>,
    receiver: Receiver<ThreadCommand>,
}

impl Process {
    pub fn new(command_text: &str, sender: Sender<ThreadCommand>, receiver: Receiver<ThreadCommand>) -> Process {
        Process {
            command_text: command_text.to_owned(),
            sender,
            receiver,
        }
    }

    pub fn start(&mut self) {
        let split = self.command_text.split(" ");
        let mut args: Vec<&str> = split.collect();
        let mut command = Command::new(&args[0]);
        args.remove(0);
        if args.len() > 0 {
            command.args(args);
        }
        let child_result = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        if child_result.is_ok() {
            let mut child = child_result.unwrap();

            let ref mut stdin = child.stdin.take().unwrap();
            let stdout = child.stdout.take().unwrap();
            let stderr = child.stderr.take().unwrap();

            let (exit_sender, exit_receiver) = mpsc::channel();
            let cloned_exit_sender = exit_sender.clone();
            thread::Builder::new().name("process_exitcode".to_string()).spawn(move || {
                let code = child.wait()
                    .expect("failed to wait on child");
                match cloned_exit_sender.send(code.to_string()) {
                    Ok(_) => (),
                    Err(_) => (),
                }
            }).unwrap();

            let (stdout_sender, stdout_receiver) = mpsc::channel();
            let cloned_stdout_sender = stdout_sender.clone();
            thread::Builder::new().name("process_stdout".to_string()).spawn(move || {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    match cloned_stdout_sender.send(line.unwrap()) {
                        Ok(_) => (),
                        Err(_) => (),
                    }
                }
            }).unwrap();

            let (stderr_sender, stderr_receiver) = mpsc::channel();
            let cloned_stderr_sender = stderr_sender.clone();
            thread::Builder::new().name("process_stderr".to_string()).spawn(move || {
                let reader = BufReader::new(stderr);
                for line in reader.lines() {
                    cloned_stderr_sender.send(line.unwrap()).unwrap();
                }
            }).unwrap();

            self.tick(stdin, &stdout_receiver, &stderr_receiver, &exit_receiver)
        } else {
            let error = child_result.unwrap_err().to_string();
            println!("Spawn command error: {}", error);
            self.sender.send(ThreadCommand::new(ThreadCommandAction::Error, error)).unwrap();
        }
    }

    fn tick(&mut self, stdin: &mut ChildStdin, stdout_receiver: &Receiver<String>, stderr_receiver: &Receiver<String>, exit_receiver: &Receiver<String>) {
        let mut need_terminate = false;

        self.check_stdout_stderr_exit_code(stdout_receiver, stderr_receiver, exit_receiver);

        match self.receiver.recv_timeout(Duration::from_millis(50)) {
            Ok(command) => {
                match command.action {
                    ThreadCommandAction::Terminate => need_terminate = true,
                    ThreadCommandAction::Stdin => self.send_stdin(stdin, &command.data),
                    _ => {}
                }
            }
            Err(_) => (),
        }

        if need_terminate == false {
            self.tick(stdin, stdout_receiver, stderr_receiver, exit_receiver);
        }
    }

    fn check_stdout_stderr_exit_code(&mut self, stdout_receiver: &Receiver<String>, stderr_receiver: &Receiver<String>, exit_receiver: &Receiver<String>) {
        match stdout_receiver.recv_timeout(Duration::from_millis(50)) {
            Ok(message) => {
                println!("{}", message);
                self.sender.send(ThreadCommand::new(ThreadCommandAction::Stdout, message)).unwrap();
            }
            Err(_) => self.check_stderr_exit_code(stderr_receiver, exit_receiver),
        }
    }

    fn check_stderr_exit_code(&mut self, stderr_receiver: &Receiver<String>, exit_receiver: &Receiver<String>) {
        match stderr_receiver.recv_timeout(Duration::from_millis(50)) {
            Ok(message) => {
                self.sender.send(ThreadCommand::new(ThreadCommandAction::Stderr, message)).unwrap();
            }
            Err(_) => self.check_error_code(exit_receiver),
        }
    }

    fn check_error_code(&mut self, exit_receiver: &Receiver<String>) {
        match exit_receiver.recv_timeout(Duration::from_millis(50)) {
            Ok(code) => {
                self.sender.send(ThreadCommand::new(ThreadCommandAction::Exit, code)).unwrap();
            }
            Err(_) => (),
        }
    }

    fn send_stdin(&self, stdin: &mut ChildStdin, command: &str) {
        let mut cmd = command.to_string();
        cmd.push_str("\n");
        println!("stdin: {}", cmd);
        stdin.write_all(cmd.as_bytes()).expect("Send command error");
    }
}
