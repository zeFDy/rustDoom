#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use crate::proc::mapProcFile;
use crate::logfile::myLogFile;
use crate::AppData;

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

    pub fn openFromPak(ourLogFile:&mut myLogFile, data: &mut AppData, mapName:&str) -> Scene
    {
        let ourProcFileName = format!("maps/game/{}.proc", mapName);
        let mut ourProcFile = mapProcFile::openFromPak(ourLogFile, data, ourProcFileName.as_str());
        ourProcFile.extractData(ourLogFile);    

        let ourScene = Scene
        {

        };

        ourScene
    }

}