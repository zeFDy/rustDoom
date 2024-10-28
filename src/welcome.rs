#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use colored::Colorize;
use crate::logfile::myLogFile;

pub struct welcomeBanner
{
    //myFile :File,
}

impl welcomeBanner
{
    pub fn  welcomeBanner(logFile:&mut myLogFile)
    {
        //let now: DateTime<Utc> = Utc::now();
        let now  = chrono::offset::Local::now();

        logFile.log("\n".to_string());
        logFile.log("                                       \\o.o/\n".to_string());
        logFile.log("---------------------------------.ooO---(_)---Ooo.----------\n".to_string());
        logFile.log("rustDoom\n".to_string());
        let sDate = format!("{}\n", now.format("%m/%d/%Y %T"));
        logFile.log(sDate.to_string());
        logFile.log("--------------------------------------------Oooo.-----------\n".to_string());
        logFile.log("                                     .oooO  (   )\n".to_string());
        logFile.log("                                     (   )   ) /\n".to_string());
        logFile.log("                                      \\ (   (_/\n".to_string());
        logFile.log("                                       \\_)\n".to_string());
        logFile.log("\n".to_string());


        print!("\n");
        print!("                                       \\");
        print!("{}", "o".red());
        print!("{}", ".".normal());
        print!("{}", "o".red());
        print!("{}", "/\n".normal());
        print!("---------------------------------.ooO---(_)---Ooo.----------\n");
        print!("rustDoom\n");
        print!("{}\n", now.format("%m/%d/%Y %T"));
        //print!("\n");
        print!("--------------------------------------------Oooo.-----------\n");
        print!("                                     .oooO  (   )\n");
        print!("                                     (   )   ) /\n");
        print!("                                      \\ (   (_/\n");
        print!("                                       \\_)\n");
        print!("\n");
    }
}