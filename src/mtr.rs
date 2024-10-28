#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::{fs, usize};
use crate::logfile::myLogFile;

pub struct mtrFile
{
    //logFile       :myLogFile,
    iType           :u8,
    inputBuffer     :Vec<u8>,
    readOffset      :usize,
    bEOF            :bool,
    uiSize          :usize,
}

impl mtrFile
{
    pub fn open(theLogFile:&mut myLogFile, fileName:&str) -> mtrFile
    {
        let sMessage = format!("Open mtrFile {}\n", fileName);
        theLogFile.log(sMessage.to_string());
        
        let ucBuffer        = fs::read(fileName).expect("Can't read file");
        let bufferSize        = ucBuffer.len();
        
        let ourFile = mtrFile
        {
            //pathProcFile  : fileName.to_string(),
            //logFile       : theLogFile,
            iType           : 0x03,
            inputBuffer     : ucBuffer,
            readOffset      : 0x00,
            //carac         : 0x00,
            bEOF            : false,
            uiSize          : bufferSize,
        };

        //self.pathProcFile   = fileName.clone();
        //let contenuProcFile = fs::read_to_string(pathProcFile).expect("Can't read file");
        //HexaDump(&contenuProcFile);     // borrow it (by reference) to avoid movement...
        //println!("{}", contenuProcFile);
        ourFile
	}
}
