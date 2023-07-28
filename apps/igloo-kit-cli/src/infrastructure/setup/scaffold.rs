use std::{fs, env};

pub fn delete_mount_volume() {
    let current_dir = env::current_dir().unwrap();
    let mount_dir = current_dir.join(".panda_house");
    
    let output = fs::remove_dir_all(mount_dir.clone());

    match output {
        Ok(_) => println!("Removed mount directory at {}", mount_dir.display()),
        Err(err) => {
            println!("Failed to remove mount directory at {}", mount_dir.display());
            println!("error: {}", err)
        },
    }
}

pub fn create_red_panda_mount_volume() {
    let current_dir = env::current_dir().unwrap();
    let mount_dir = current_dir.join(".panda_house");
    
    // This function will fail silently if the directory already exists
    let output = fs::create_dir_all(mount_dir.clone());

    match output {
        Ok(_) => println!("Created mount directory at {}", mount_dir.display()),
        Err(err) => {
            println!("Failed to create mount directory at {}", mount_dir.display());
            println!("error: {}", err)
        },
    }
}

pub fn create_ch_mount_volume() {
    let current_dir = env::current_dir().unwrap();
    let mount_dir = current_dir.join(".clickhouse");

    // This function will fail silently if the directory already exists
    let main_dir_result = fs::create_dir_all(mount_dir.clone());
    let data_dir_result = fs::create_dir_all(mount_dir.clone().join("data"));
    let logs_dir_result = fs::create_dir_all(mount_dir.clone().join("logs"));

    match main_dir_result {
        Ok(_) => {
            println!("Created main mount directory at {}", mount_dir.display());

            match data_dir_result {
                Ok(_) => println!("Created data mount directory at {}", mount_dir.clone().join("data").display()),
                Err(err) => {
                    println!("Failed to create data mount directory at {}", mount_dir.clone().join("data").display());
                    println!("error: {}", err)
                },
            }

            match logs_dir_result {
                Ok(_) => println!("Created logs mount directory at {}", mount_dir.clone().join("logs").display()),
                Err(err) => {
                    println!("Failed to create logs mount directory at {}", mount_dir.clone().join("logs").display());
                    println!("error: {}", err)
                },
            }
        },
        Err(err) => {
            println!("Failed to create mount directory at {}", mount_dir.display());
            println!("error: {}", err)
        },
    }
}