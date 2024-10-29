use std::fs;
use crate::logfile::myLogFile;
use crate::mtr::mtrFile;
use crate::welcome::welcomeBanner;
use crate::scene::Scene;
use std::path::Path;
use zip::ZipArchive;
use std::fs::File;
use std::io::Read;


// Log Options for test/debug
pub const LOG_FILENAME_FROM_ZIP_ARCHIVE :bool = false;       
pub const LOG_MTR_FILE_CONTENT          :bool = false;       
pub const LOG_MTR_FILELIST              :bool = false;
pub const LOG_MAP_FILELIST              :bool = false;
pub const LOG_PROC_FILELIST             :bool = true;
pub const LOG_TGA_FILELIST              :bool = true;
pub const LOG_MODEL_DETAILS             :bool = false;


#[derive(Clone, Debug)]
struct  pakFileInfo
{
        sFileName       : String,
        sPakFileName    : String,
        archiveIndex    : usize,
}

#[derive(Clone, Debug, Default)]
pub struct  RustDoom
{
        procFilesInfoList:  Vec<pakFileInfo>,
        mapFilesInfoList:   Vec<pakFileInfo>,
        mtrFilesInfoList:   Vec<pakFileInfo>,
        tgaFilesInfoList:   Vec<pakFileInfo>,
        mtrFilesContent:    Vec<String>,
}

impl    RustDoom 
{
    fn logPakFileInfoList(ourLogFile:&mut myLogFile, sMessage:&String, thisList:&Vec<pakFileInfo>)
    {
        let sMessage = format!("\n{}\n", sMessage);
        ourLogFile.log(sMessage);

        let listSize = thisList.len();
        let mut iCounter: usize =0;
        loop
        {
            if iCounter>=listSize    {break;}
            
            let thisEntry = thisList.get(iCounter).expect("out of range");
            let sMessage = format!("{:20} {:7} {}\n", thisEntry.sPakFileName, thisEntry.archiveIndex, thisEntry.sFileName);
            ourLogFile.log(sMessage);

            iCounter += 1;

        }
    }

    fn logStringList(ourLogFile:&mut myLogFile, sMessage:&String, thisList:&Vec<String>)
    {
        let sMessage = format!("\nDump mtrFilesContent\n");
        ourLogFile.log(sMessage);

        let listSize = thisList.len();
        let mut iCounter: usize =0;
        loop
        {
            if iCounter>=listSize    {break;}
            
            let thisEntry = thisList.get(iCounter).expect("out of range");
            let sMessage = format!("{}\n", thisEntry);
            ourLogFile.log(sMessage);

            iCounter += 1;

        }
    }

    fn newPakFileInfoItem (fileName:&String, zipFilePath:&Path, iIndex:usize) -> pakFileInfo
    {
        let thisFileInfo = pakFileInfo 
        {
            sFileName       : fileName.clone(),
            sPakFileName    : zipFilePath.display().to_string(),
            archiveIndex    : iIndex,
        };

        thisFileInfo
    }

    pub fn readTgaFileFromPak(&mut self, fileName:&String, buffer : &mut Vec<u8> ) 
    {
            println!("readTgaFileFromPak({:#?});", fileName);
            //let mut thisVector = Vec::new();

            let listSize = self.tgaFilesInfoList.len();
            let mut iCounter: usize =0;
            loop
            {
                if iCounter>=listSize    {break;}
                
                let thisEntry = self.tgaFilesInfoList.get(iCounter).expect("out of range");
                //let thisEntry = self.procFilesInfoList.get(iCounter).expect("out of range");
                //let sMessage = format!("{:20} {:7} {}\n", thisEntry.sPakFileName, thisEntry.archiveIndex, thisEntry.sFileName);
                //ourLogFile.log(sMessage);
    
                if thisEntry.sFileName == *fileName
                {
                    println!("--> found !");

                    let zipFilePath = Path::new(&thisEntry.sPakFileName);
                    let zipFile = File::open(zipFilePath).expect("file error");
                    let mut archive = ZipArchive::new(zipFile).expect("file error");
                    let mut file = archive.by_index(thisEntry.archiveIndex).expect("file error");
    
                    file.read_to_end(buffer);
                    return;
                }

                iCounter += 1;
    
            }
    
    }

    pub fn readProcFileFromPak(&mut self, fileName:&String, buffer : &mut Vec<u8> )
    {
        println!("readProcFileFromPak({:#?});", fileName);
        //let mut thisVector = Vec::new();

        let listSize = self.procFilesInfoList.len();
        let mut iCounter: usize =0;
        loop
        {
            if iCounter>=listSize    {break;}
            
            let thisEntry = self.procFilesInfoList.get(iCounter).expect("out of range");
            //let thisEntry = self.procFilesInfoList.get(iCounter).expect("out of range");
            //let sMessage = format!("{:20} {:7} {}\n", thisEntry.sPakFileName, thisEntry.archiveIndex, thisEntry.sFileName);
            //ourLogFile.log(sMessage);

            if thisEntry.sFileName == *fileName
            {
                println!("--> found !");

                let zipFilePath = Path::new(&thisEntry.sPakFileName);
                let zipFile = File::open(zipFilePath).expect("file error");
                let mut archive = ZipArchive::new(zipFile).expect("file error");
                let mut file = archive.by_index(thisEntry.archiveIndex).expect("file error");
                
                //file.read_to_string(buffer);
                file.read_to_end(buffer);
                return;
            }

            iCounter += 1;
        }
    }

    pub fn createRustDoom(mut theLogFile:&mut myLogFile) -> RustDoom 
    {

        let mut thisProcFilesInfoList:  Vec<pakFileInfo>    = Vec::new();
        let mut thisMapFilesInfoList:   Vec<pakFileInfo>    = Vec::new();
        let mut thisMtrFilesInfoList:   Vec<pakFileInfo>    = Vec::new();
        let mut thisTgaFilesInfoList:   Vec<pakFileInfo>    = Vec::new();
        // let mut ddsFilesInfoList:   Vec<pakFileInfo>    = Vec::new();
        // let mut wavFilesInfoList:   Vec<pakFileInfo>    = Vec::new();
        // let mut oggFilesInfoList:   Vec<pakFileInfo>    = Vec::new();
        let mut thisMtrFilesContent:    Vec<String>         = Vec::new();

        let entries = fs::read_dir(".").expect("io error");

        for entry in entries
        {
            let entry = entry.expect("io error");
            let path = entry.path();
            println!("{:#?}", path);

            //let sMessage = format!("path is {:#?}\n", path);
            //theLogFile.log(sMessage);
            
            let sPath = path.display().to_string();
            //let sMessage = format!("sPath is {}\n", sPath);
            //theLogFile.log(sMessage);

            if sPath.ends_with(".pk4")
            {
                // ----- essai fichier zip ----- 
                let zipFilePath = Path::new(&sPath);
                let zipFile = File::open(zipFilePath).expect("file error");

                let mut archive = ZipArchive::new(zipFile).expect("file error");

                // Iterate through the files in the ZIP archive.
                for i in 0..archive.len() {
                    let mut file = archive.by_index(i).expect("file error");
                    let fileName = file.name().to_owned();

                    //let sMessage = format!("-----\nNEXT FILE is {}\n", fileName);
                    
                    if fileName.ends_with(".proc")          
                    {
                        if LOG_FILENAME_FROM_ZIP_ARCHIVE ==true {theLogFile.log(" -> PROC File".to_string());}
                        thisProcFilesInfoList.push(Self::newPakFileInfoItem(&fileName, zipFilePath, i));
                    }
                    else if fileName.ends_with(".map")      
                    {
                        if LOG_FILENAME_FROM_ZIP_ARCHIVE ==true {theLogFile.log(" -> MAP  File".to_string());}
                        thisMapFilesInfoList.push(Self::newPakFileInfoItem(&fileName, zipFilePath, i));
                    }
                    else if fileName.ends_with(".mtr")      
                    {
                        if LOG_FILENAME_FROM_ZIP_ARCHIVE ==true {theLogFile.log(" -> MTR  File".to_string());}
                        thisMtrFilesInfoList.push(Self::newPakFileInfoItem(&fileName, zipFilePath, i));

                        let mut thisString:String = "".to_string();
                        file.read_to_string(&mut thisString).expect("io error");
                        thisMtrFilesContent.push(thisString);
                    }
                    else if fileName.ends_with(".tga")      
                    {
                        if LOG_FILENAME_FROM_ZIP_ARCHIVE ==true {theLogFile.log(" -> TGA  File".to_string());}
                        thisTgaFilesInfoList.push(Self::newPakFileInfoItem(&fileName, zipFilePath, i));
                        /*/
                        let mut readBuffer = Vec::new();
                        file.read_to_end(&mut readBuffer);
                        */
                    }
                    else if fileName.ends_with(".dds")      
                    {
                        if LOG_FILENAME_FROM_ZIP_ARCHIVE ==true {theLogFile.log(" -> DDS  File".to_string());}
                    }
                    else if fileName.ends_with(".ogg")      
                    {
                        if LOG_FILENAME_FROM_ZIP_ARCHIVE ==true {theLogFile.log(" -> OGG  File".to_string());}
                    }
                    else if fileName.ends_with(".wav")      
                    {
                        if LOG_FILENAME_FROM_ZIP_ARCHIVE ==true {theLogFile.log(" -> WAV  File".to_string());}
                    }
                    else
                    {
                        if LOG_FILENAME_FROM_ZIP_ARCHIVE ==true {theLogFile.log(" ->      File".to_string());}
                    }

                    if LOG_FILENAME_FROM_ZIP_ARCHIVE ==true
                    {
                        let sMessage = format!(" is {}", &fileName);
                        theLogFile.log(sMessage);

                        theLogFile.log("\n".to_string());
                    }

                    // Create the path to the extracted file in the destination directory.
                    // let target_path = extraction_dir.join(file_name);

                    // // Create the destination directory if it does not exist.
                    // if let Some(parent_dir) = target_path.parent() {
                    //     std::fs::create_dir_all(parent_dir)?;
                    // }

                    // let mut output_file = File::create(&target_path)?;

                    // // Read the contents of the file from the ZIP archive and write them to the destination file.
                    // io::copy(&mut file, &mut output_file)?;
                    
                    //let mut _thisString:String = "".to_string();
                    //file.read_to_string(&mut _thisString);
                    //theLogFile.log(_thisString);

                    //theLogFile.log("-----\n".to_string());

                }

                //println!("Files successfully extracted to {:?}", extraction_dir);

                // ----- fin essai fichier zip ----- 

            }

        }

        if LOG_PROC_FILELIST == true
        {
            Self::logPakFileInfoList(       &mut theLogFile, 
                                            &"Dump procFilesInfoList".to_string(), 
                                            &thisProcFilesInfoList);
        }

        if LOG_MAP_FILELIST == true
        {
            Self::logPakFileInfoList(       &mut theLogFile, 
                                            &"Dump mapFilesInfoList".to_string(), 
                                            &thisMapFilesInfoList);
        }

        if  LOG_MTR_FILELIST == true
        {
            Self::logPakFileInfoList(       &mut theLogFile, 
                                    &"Dump mtrFilesInfoList".to_string(), 
                                    &thisMtrFilesInfoList);
        }

        if  LOG_MTR_FILE_CONTENT == true
        {
            Self::logStringList(  &mut theLogFile,  
                            &"Dump mtrFilesContent".to_string(),
                            &thisMtrFilesContent);
        }

        if  LOG_TGA_FILELIST == true
        {
            Self::logPakFileInfoList(     &mut theLogFile, 
                                    &"Dump tgaFilesInfoList".to_string(), 
                                    &thisTgaFilesInfoList);
        }

        /*
        let ourScene = Scene::open(&mut theLogFile, "admin");
        let ourMtfFile = mtrFile::open(&mut theLogFile, "materials\\base_floor.mtr");
        */

        let thisRustDoom =  RustDoom
        {
            procFilesInfoList:  thisProcFilesInfoList,
            mapFilesInfoList:   thisMapFilesInfoList,
            mtrFilesInfoList:   thisMtrFilesInfoList,
            tgaFilesInfoList:   thisTgaFilesInfoList,
            mtrFilesContent:    thisMtrFilesContent,
        };

        return  thisRustDoom;
        
    }

}