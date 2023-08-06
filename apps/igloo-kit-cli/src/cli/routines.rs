use std::{io::Error, path::PathBuf};

use crate::{framework, infrastructure};

use super::{CommandTerminal, user_messages::show_message, MessageType, Message};


pub fn start_containers(term: &mut CommandTerminal) -> Result<(), Error> {
    show_message( term, MessageType::Info, Message {
        action: "Running",
        details: "infrastructure spin up",
    });
    
    infrastructure::spin_up(term)?;
    Ok(())
}

pub fn initialize_project(term: &mut CommandTerminal) -> Result<(), Error> {
    let igloo_dir = framework::directories::create_top_level_temp_dir(term)?;
    match framework::directories::create_project_directories(term) {
        Ok(_) => {
            show_message( term, MessageType::Success, Message {
                action: "Finished",
                details: "initializing project directory",
            });
        },
        Err(err) => {
            show_message( term, MessageType::Error, Message {
                action: "Failed",
                details: "to create project directories",
            });
            return Err(err)
        }
    };
    infrastructure::init(term, &igloo_dir)?;
    Ok(())
}

pub fn clean_project(term: &mut CommandTerminal, igloo_dir: &PathBuf) -> Result<(), Error> {
    show_message( term, MessageType::Info, Message {
        action: "Cleaning",
        details: "project directory",
    });
    infrastructure::clean(term, igloo_dir)?;
    show_message(
        term,
        MessageType::Success,
        Message {
            action: "Finished",
            details: "cleaning project directory",
        },
    );
    Ok(())
}

pub fn stop_containers(term: &mut CommandTerminal) -> Result<(), Error> {
    show_message( term, MessageType::Info, Message {
        action: "Stopping",
        details: "local infrastructure",
    });
    infrastructure::spin_down(term)?;
    Ok(())
}
    