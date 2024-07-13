use std::borrow::BorrowMut;

use std::process::{Child, Command, Stdio};
use tauri::api::process::Command as TCommand;
use log::{info, error};
use std::io::{BufReader, BufRead};
use std::thread;

#[cfg(unix)]
use command_group::{Signal, UnixChildExt};

fn log_child_stderr(mut child: Child) -> Child {
    if let Some(stderr) = child.stderr.take() {
        let stderr_reader = BufReader::new(stderr);
        thread::spawn(move || {
            for line in stderr_reader.lines() {
                match line {
                    Ok(line) => error!("[sidecar | core]: {}", line.trim_end_matches('\n')),
                    Err(err) => error!("Error reading stderr: {}", err),
                }
            }
        });
    }

    child
}

pub struct SidecarLifeCycleService {
    program: String,
    sidecar_command: Command,
    child: Option<Child>,
}

impl SidecarLifeCycleService {
    pub fn new<S: Into<String>>(program: S) -> SidecarLifeCycleService {
        let program_string = program.into();
        let sidecar_command = TCommand::new_sidecar(&program_string).expect("failed to setup sidecar");
        SidecarLifeCycleService {
            program: program_string,
            sidecar_command: sidecar_command.into(),
            child: None,
        }
    }

    pub fn start(&mut self) -> Result<String, String> {
        match self.child.borrow_mut() {
            Some(_) => {
                let info = format!("Sidecar {} already running", self.program);
                info!("{}", &info);
                Ok(info.into())
            }
            None => {
                let child = self.sidecar_command.stderr(Stdio::piped()).spawn();
                match child {
                    Ok(mut child) => {
                        let id = child.id();
                        child = log_child_stderr(child);

                        self.child = Some(child);
                        
                        let info = format!("Sidecar {} started - {}", self.program, id);
                        info!("{}", &info);

                        Ok(info.into())
                    }
                    Err(e) => {
                        let info = format!("Sidecar {} start failed - {}", self.program, e.to_string());
                        error!("{}", &info);
                        Err(info.into())
                    }
                }
            }
        }     
    }

    pub fn stop(&mut self) -> Result<String, String> {
        match self.child.borrow_mut() {
            Some(child) => {
                let id = child.id();
                
                #[cfg(unix)]
                {
                    child
                    .signal(Signal::SIGTERM)
                    .expect("Some error happened when killing child process");
                }
                #[cfg(windows)]
                {
                    child.kill().expect("Some error happened when killing child process");
                }

                self.child = None;
                let info = format!("Sidecar {} stopped - {}", self.program, id);
                info!("{}", &info);
                Ok(info.into())
            }
            _ => {
                let info = format!("Sidecar {} stop failed", self.program);
                println!("{}", &info);
                Ok(info.into())
            }
        }
    }
}
