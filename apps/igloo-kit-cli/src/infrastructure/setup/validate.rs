use std::io::{self, Write, Error};

use crate::infrastructure::docker;


// TODO: Add clickhouse validation

pub fn validate_docker_run(debug: bool) -> Result<(), Error> {
    let output = docker::filter_list_containers("redpanda-1");

    match output {
        Ok(o) => {
            if debug {
                io::stdout().write_all(&o.stdout).unwrap();
            }
            let output = String::from_utf8(o.stdout).unwrap();
            if output.contains("redpanda-1") {
                println!("Successfully validated docker container");
                Ok(())
            } else {
                println!("Failed to validate docker container");
                return Err(io::Error::new(io::ErrorKind::Other, "Failed to validate red panda container exists"))
            }
        },
        Err(err) => {
            println!("Failed to validate docker container");
            Err(err)
        },
    }
}

pub fn validate_panda_house_network(debug: bool) -> Result<(), Error> {
    let output = docker::network_list();

    match output {
        Ok(o) => {
            if debug {
                io::stdout().write_all(&o.stdout).unwrap();
            }
            let output = String::from_utf8(o.stdout).unwrap();
            if output.contains("panda_house") {
                println!("Successfully validated docker panda_house network");
                Ok(())
            } else {
                println!("Failed to validate panda_house_network");
                Err(io::Error::new(io::ErrorKind::Other, "Failed to validate panda_house network"))
            }
        },
        Err(err) => {
            println!("Failed to validate panda_house_network");
            Err(err)
        },
        
    }
}

pub fn validate_red_panda_cluster(debug: bool) -> Result<(),  Error> {
    let output = docker::run_rpk_list();

    match output {
        Ok(o) => {
            if debug {
                io::stdout().write_all(&o.stdout).unwrap();
            }
            let output = String::from_utf8(o.stdout).unwrap();
            if output.contains("redpanda-1") {
                println!("Successfully validated red panda cluster");
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