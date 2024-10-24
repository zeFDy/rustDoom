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

use std::error::Error;
use std::io::{self, Read};
use std::path::Path;
use zip::ZipArchive;

pub mod logfile;
pub mod proc;
pub mod mtr;
pub mod hexadump;
pub mod welcome;
pub mod scene;

// use std::io::prelude::*;
// fn list_zip_contents(reader: impl Read + Seek) -> zip::result::ZipResult<()> {
//     use zip::HasZipMetadata;
//     let mut zip = zip::ZipArchive::new(reader)?;

//     for i in 0..zip.len() {
//         let mut file = zip.by_index(i)?;
//         println!("Filename: {}", file.name());
//         //std::io::copy(&mut file, &mut std::io::stdout())?;
//     }

//     Ok(())
// }

fn main() 
{
    let mut theLogFile = myLogFile::open();
    welcomeBanner::welcomeBanner(&mut theLogFile);

    // ----- essai fichier zip ----- 
    let zip_file_path = Path::new("PeachCompiler-EndOfParser.zip");
    let zip_file = File::open(zip_file_path).expect("file error");

    let mut archive = ZipArchive::new(zip_file).expect("file error");
    // let extraction_dir = Path::new("extracted_files");

    // // Create the directory if it does not exist.
    // if !extraction_dir.exists() {
    //     std::fs::create_dir(extraction_dir)?;
    // }

    // Iterate through the files in the ZIP archive.
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).expect("file error");
        let file_name = file.name().to_owned();

        let sMessage = format!("-----\nNEXT FILE is {}\n", file_name);
        theLogFile.log(sMessage);

        // Create the path to the extracted file in the destination directory.
        // let target_path = extraction_dir.join(file_name);

        // // Create the destination directory if it does not exist.
        // if let Some(parent_dir) = target_path.parent() {
        //     std::fs::create_dir_all(parent_dir)?;
        // }

        // let mut output_file = File::create(&target_path)?;

        // // Read the contents of the file from the ZIP archive and write them to the destination file.
        // io::copy(&mut file, &mut output_file)?;
        let mut _thisString:String = "".to_string();
        file.read_to_string(&mut _thisString);
        theLogFile.log(_thisString);

        theLogFile.log("-----\n".to_string());

    }

    //println!("Files successfully extracted to {:?}", extraction_dir);

    // ----- fin essai fichier zip ----- 
    

    
    let mut ourScene = Scene::open(&mut theLogFile, "admin");

    let mut ourMtfFile = mtrFile::open(&mut theLogFile, "materials\\base_floor.mtr");
    return;
    
}

