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


struct  pakFileInfo
{
        sFileName       : String,
        sPakFileName    : String,
        archiveIndex    : usize,
}

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

pub fn mainRustDoom() 
{
    let mut procFilesInfoList:  Vec<pakFileInfo>    = Vec::new();
    let mut mapFilesInfoList:   Vec<pakFileInfo>    = Vec::new();
    let mut mtrFilesInfoList:   Vec<pakFileInfo>    = Vec::new();
    let mut tgaFilesInfoList:   Vec<pakFileInfo>    = Vec::new();
    // let mut ddsFilesInfoList:   Vec<pakFileInfo>    = Vec::new();
    // let mut wavFilesInfoList:   Vec<pakFileInfo>    = Vec::new();
    // let mut oggFilesInfoList:   Vec<pakFileInfo>    = Vec::new();
    let mut mtrFilesContent:    Vec<String>         = Vec::new();

    let mut theLogFile = myLogFile::open();
    welcomeBanner::welcomeBanner(&mut theLogFile);


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
                    procFilesInfoList.push(newPakFileInfoItem(&fileName, zipFilePath, i));
                }
                else if fileName.ends_with(".map")      
                {
                    if LOG_FILENAME_FROM_ZIP_ARCHIVE ==true {theLogFile.log(" -> MAP  File".to_string());}
                    mapFilesInfoList.push(newPakFileInfoItem(&fileName, zipFilePath, i));
                }
                else if fileName.ends_with(".mtr")      
                {
                    if LOG_FILENAME_FROM_ZIP_ARCHIVE ==true {theLogFile.log(" -> MTR  File".to_string());}
                    mtrFilesInfoList.push(newPakFileInfoItem(&fileName, zipFilePath, i));

                    let mut thisString:String = "".to_string();
                    file.read_to_string(&mut thisString).expect("io error");
                    mtrFilesContent.push(thisString);
                }
                else if fileName.ends_with(".tga")      
                {
                    if LOG_FILENAME_FROM_ZIP_ARCHIVE ==true {theLogFile.log(" -> TGA  File".to_string());}
                    tgaFilesInfoList.push(newPakFileInfoItem(&fileName, zipFilePath, i));
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
        logPakFileInfoList(     &mut theLogFile, 
                                &"Dump procFilesInfoList".to_string(), 
                                &procFilesInfoList);
    }

    if LOG_MAP_FILELIST == true
    {
        logPakFileInfoList(     &mut theLogFile, 
                                &"Dump mapFilesInfoList".to_string(), 
                                &mapFilesInfoList);
    }

    if  LOG_MTR_FILELIST == true
    {
        logPakFileInfoList(     &mut theLogFile, 
                                &"Dump mtrFilesInfoList".to_string(), 
                                &mtrFilesInfoList);
    }

    if  LOG_MTR_FILE_CONTENT == true
    {
        logStringList(  &mut theLogFile,  
                        &"Dump mtrFilesContent".to_string(),
                        &mtrFilesContent);
    }

    if  LOG_TGA_FILELIST == true
    {
        logPakFileInfoList(     &mut theLogFile, 
                                &"Dump tgaFilesInfoList".to_string(), 
                                &tgaFilesInfoList);
    }

    let ourScene = Scene::open(&mut theLogFile, "admin");

    let ourMtfFile = mtrFile::open(&mut theLogFile, "materials\\base_floor.mtr");
    return;
    
}
