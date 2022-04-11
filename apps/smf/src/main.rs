// Copyright (c) 2022 VPlane Telecoms

use log::*;
use simple_logger::SimpleLogger;
mod init;

fn app_initialize() -> Result<(), &'static str>{
    match init::smf_initialize() {
        Ok(_) => {
            info!("SMF initialize...done");
            Ok(())
        },
        Err(err) => {
            error!("Failed to intialize SMF");
            Err(err)
        }
    }
}

#[allow(dead_code)]
fn app_terminate()
{
    init::smf_terminate();
    info!("SMF terminate...done");
}

//#[tokio::main]
fn main() {
    SimpleLogger::new().init().unwrap();
    app_initialize().unwrap();
}