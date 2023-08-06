use std::io::{self, Write, Error};

use crate::{infrastructure::docker, cli::{CommandTerminal, user_messages::{show_message, MessageType, Message}}};


// TODO: Add clickhouse validation
pub fn validate_clickhouse_run(term: &mut CommandTerminal, debug: bool) -> Result<(), Error> {
    let output = docker::filter_list_containers("clickhousedb-1");

    match output {
        Ok(o) => {
            if debug {
                io::stdout().write_all(&o.stdout).unwrap();
            }
            let output = String::from_utf8(o.stdout).unwrap();
            if output.contains("clickhouse") {
                show_message(term, MessageType::Success, Message {
                    action: "Successfully",
                    details: "validated clickhouse docker container",
                });
                Ok(())
            } else {
                show_message(term,
                    MessageType::Error,
                    Message {
                        action: "Failed",
                        details: "to validate clickhouse docker container",
                    },
                );
                return Err(io::Error::new(io::ErrorKind::Other, "Failed to validate clickhouse container exists"))
            }
        },
        Err(err) => {
            show_message(term,
                    MessageType::Error,
                    Message {
                        action: "Failed",
                        details: "to validate red panda docker container",
                    },
                );
            Err(err)
        },
    }
}



pub fn validate_red_panda_run(term: &mut CommandTerminal, debug: bool) -> Result<(), Error> {
    let output = docker::filter_list_containers("redpanda-1");

    match output {
        Ok(o) => {
            if debug {
                io::stdout().write_all(&o.stdout).unwrap();
            }
            let output = String::from_utf8(o.stdout).unwrap();
            if output.contains("redpanda-1") {
                show_message(term, MessageType::Success, Message {
                    action: "Successfully",
                    details: "validated red panda docker container",
                });
                Ok(())
            } else {
                show_message(term,
                    MessageType::Error,
                    Message {
                        action: "Failed",
                        details: "to validate red panda docker container",
                    },
                );
                return Err(io::Error::new(io::ErrorKind::Other, "Failed to validate red panda container exists"))
            }
        },
        Err(err) => {
            show_message(term,
                    MessageType::Error,
                    Message {
                        action: "Failed",
                        details: "to validate red panda docker container",
                    },
                );
            Err(err)
        },
    }
}

pub fn validate_panda_house_network(term: &mut CommandTerminal, debug: bool) -> Result<(), Error> {
    let output = docker::network_list();

    match output {
        Ok(o) => {
            if debug {
                io::stdout().write_all(&o.stdout).unwrap();
            }
            let output = String::from_utf8(o.stdout).unwrap();
            if output.contains("panda_house") {
                println!("Successfully validated docker panda_house network");
                show_message(term, MessageType::Success, Message {
                    action: "Successfully",
                    details: "validated panda_house docker network",
                });
                Ok(())
            } else {
                show_message(
                    term,
                    MessageType::Error,
                    Message {
                        action: "Failed",
                        details: "to validate panda_house docker network",
                    },
                );
                Err(io::Error::new(io::ErrorKind::Other, "Failed to validate panda_house network"))
            }
        },
        Err(err) => {
            println!("Failed to validate panda_house_network");
            Err(err)
        },
        
    }
}

pub fn validate_red_panda_cluster(term: &mut CommandTerminal, debug: bool) -> Result<(),  Error> {
    let output = docker::run_rpk_list();

    match output {
        Ok(o) => {
            if debug {
                io::stdout().write_all(&o.stdout).unwrap();
            }
            let output = String::from_utf8(o.stdout).unwrap();
            if output.contains("redpanda-1") {
                println!("Successfully validated red panda cluster");
                show_message(term, MessageType::Success, Message {
                    action: "Successfully",
                    details: "validated red panda cluster",
                });
                Ok(())
            } else {
                println!("Failed to validate docker container");
                Err(io::Error::new(io::ErrorKind::Other, "Failed to validate red panda cluster"))

            }
        },
        Err(err) => {
            println!("Failed to validate redpanda cluster");
            Err(err)
        },
    }
}