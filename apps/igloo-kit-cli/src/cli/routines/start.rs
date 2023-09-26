use std::io::{self, Write, Error};

use crate::{cli::{CommandTerminal, display::{show_message, MessageType, Message}}, framework::directories, utilities::docker::{self, run_clickhouse}, infrastructure::{olap::clickhouse::ClickhouseConfig, PANDA_NETWORK}};

use super::{initialize::{create_docker_network, validate_mount_volumes}, validate::{validate_panda_house_network, validate_red_panda_run, validate_clickhouse_run}};


pub fn run_red_panda_docker_container(term: &mut CommandTerminal, debug: bool) -> Result<(), io::Error> {
    let igloo_dir = directories::get_igloo_directory()?;
    let output = docker::run_red_panda(igloo_dir);

    match output {
        Ok(o) => {
            if debug {
                println!("Debugging red panda container run");
                println!("{}", &o.status);
                io::stdout().write_all(&o.stdout).unwrap();
            }
            show_message( term, MessageType::Success, Message {
                action: "Successfully",
                details: "ran redpanda container",
            });
            Ok(())
        },
        Err(err) => {
            show_message( term, MessageType::Error, Message {
                action: "Failed",
                details: "to run redpanda container",
            });
            Err(err)
        },
    }
        
}

pub fn run_ch_docker_container(term: &mut CommandTerminal, clickhouse_config: ClickhouseConfig, debug: bool) -> Result<(), io::Error> {
    let igloo_dir: std::path::PathBuf = directories::get_igloo_directory()?;

    let output = run_clickhouse(igloo_dir, clickhouse_config);

    match  output {
        Ok(o) => {
            if debug {
                println!("Debugging clickhouse container run");
                io::stdout().write_all(&o.stdout).unwrap();
            }
            show_message( term, MessageType::Success, Message {
                action: "Successfully",
                details: "ran clickhouse container",
            });
            Ok(())
        },
        Err(err) => {
            show_message( term, MessageType::Error, Message {
                action: "Failed",
                details: "to run clickhouse container",
            });
            Err(err)
        },
    }
}

pub fn spin_up(term: &mut CommandTerminal, clickhouse_config: ClickhouseConfig) -> Result<(), Error> {
    let igloo_dir = match directories::get_igloo_directory() {
        Ok(dir) => dir,
        Err(err) => {
            show_message( term, MessageType::Error, Message {
                action: "Failed",
                details: "Please run `igloo init` to create the necessary mount volumes",
            });
            return Err(err);
        }
    };

    show_message(
        term,
        MessageType::Info,
        Message {
            action: "Running",
            details: "igloo cluster spin up",
        },
    );
    match validate_mount_volumes(&igloo_dir) {
        Ok(_) => {

            match validate_panda_house_network(term, PANDA_NETWORK, true) {
                Ok(_) => {
                    show_message( term, MessageType::Success, Message {
                        action: "Successfully",
                        details: "found docker network",
                    });
                },
                Err(_) => {
                    create_docker_network(term, PANDA_NETWORK)?;
                    match validate_panda_house_network(term, PANDA_NETWORK, true) {
                        Ok(_) => {},
                        Err(err) => {
                            show_message( term, MessageType::Error, Message {
                                action: "Failed",
                                details: "to recover and create docker network please contact support",
                            });
                            return Err(err);
                        }
                    }
                }
            };
            run_red_panda_docker_container(term,true)?;
            validate_red_panda_run(term,true)?;
            run_ch_docker_container(term, clickhouse_config, true, )?;
            validate_clickhouse_run(term, true)?;
            Ok(())
        },
        Err(err) => {
            show_message( term, MessageType::Error, Message {
                action: "Failed",
                details: "Please run `igloo init` to create the necessary mount volumes",
            });
            return Err(err);
        }
    }
}
