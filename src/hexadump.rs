#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

// use std::{fs, io::Write, usize, vec};
// use colored::Colorize;
// use std::process::exit;
// use fs::File;
// use chrono::{DateTime, Utc};
// use crate::logfile::myLogFile;


fn HexaDump(toDisplay:&Vec<u8>)
{
    let mut index =0;
    let mut ascii: std::string::String = std::string::String::new();
    
    for element in toDisplay
    {
        print!("{:#04X} ", *element);
        let mut key:char = *element as char;
        if key.is_alphanumeric()==false {key='.';}
        ascii += &key.to_string();    
        //ascii += &element.to_owned().to_string();
        index +=1;
        if index%16==0  {print!("|{}|\n", ascii); ascii="".to_string();}
    }

    if index%16!=0
    {
        loop
        {
            index +=1;
            print!("     ");    
            ascii+=" ";
            if index%16==0   {break;}                    
        }
        print!("|{}|\n", ascii);
    }
}
