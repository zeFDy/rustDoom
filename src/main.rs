#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::{fs, io::Write, usize, vec};
use colored::Colorize;
use std::process::exit;
use fs::File;
use chrono::{DateTime, Utc};

use crate::logfile::myLogFile;
use crate::proc::mapProcFile;
use crate::mtr::mtrFile;
use crate::welcome::welcomeBanner;

pub mod logfile;
pub mod proc;
pub mod mtr;
pub mod hexadump;
pub mod welcome;

fn main() 
{
    let mut logFile = myLogFile::open();
    //logFile.log("sMessageToLog\n".to_string());
    //logFile.log("sMessageToLog\n".to_string());

    welcomeBanner::welcomeBanner(&mut logFile);
    let mut ourProcFile = mapProcFile::open(&mut logFile, "maps\\admin.proc");

    ourProcFile.extractData(&mut logFile);    
    let mut ourMtfFile = mtrFile::open(&mut logFile, "materials\\base_floor.mtr");
    return;
    
}

