#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::{fs, io::Write, usize, vec};
use colored::Colorize;
use std::process::exit;
use fs::File;
use chrono::{DateTime, Utc};

pub struct myLogFile
{
    myFile :File,
}

impl myLogFile
{
    pub fn open() -> myLogFile
    {
        let localFile = File::create("myLogFile02.txt").expect("Cannot create file myLogFile02.txt");   
        let thisFile = myLogFile
        {
            myFile      :localFile,
        };

        thisFile
    }

    pub fn log(&mut self, sMessageToLog:String)
    {
        write!(self.myFile, "{}", sMessageToLog).expect("Cannot write to file myLogFile02.txt");
    }

}
