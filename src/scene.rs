#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

// use std::fs;
// use std::{io::Write, usize, vec};
// use colored::Colorize;
// use std::process::exit;
// use fs::File;
// use chrono::{DateTime, Utc};

use crate::proc::mapProcFile;
use crate::logfile::myLogFile;

pub struct Scene
{

}

impl Scene
{
    pub fn open(ourLogFile:&mut myLogFile, mapName:&str) -> Scene
    {
        let ourProcFileName = format!("maps\\{}.proc", mapName);
        let mut ourProcFile = mapProcFile::open(ourLogFile, ourProcFileName.as_str());
        ourProcFile.extractData(ourLogFile);    

        let ourScene = Scene
        {

        };

        ourScene
    }

}