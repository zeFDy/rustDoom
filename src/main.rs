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
use crate::scene::Scene;

pub mod logfile;
pub mod proc;
pub mod mtr;
pub mod hexadump;
pub mod welcome;
pub mod scene;

fn main() 
{
    let mut theLogFile = myLogFile::open();
    welcomeBanner::welcomeBanner(&mut theLogFile);


    let mut ourScene = Scene::open(&mut theLogFile, "admin");

    let mut ourMtfFile = mtrFile::open(&mut theLogFile, "materials\\base_floor.mtr");
    return;
    
}

