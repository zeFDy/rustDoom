#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::{fs, io::Write, usize};
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


pub struct mapProcFile
{
    //logFile       :myLogFile,
    //pathProcFile  :String,
    iType           :u8,
    inputBuffer     :Vec<u8>,
    readOffset      :usize,
    //carac         :u8,
    bEOF            :bool,
    uiSize          :usize,
}

impl mapProcFile
{
    pub fn open(theLogFile:&mut myLogFile, fileName:&str) -> mapProcFile
    {
        let sMessage = format!("Open mapProcFile {}\n", fileName);
        theLogFile.log(sMessage.to_string());
        
        let ucBuffer          = fs::read(fileName).expect("Can't read file");
        let bufferSize          = ucBuffer.len();
        
        let ourFile = mapProcFile
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

    pub fn checkFirstLine(&mut self, theLogFile:&mut myLogFile)
    {
        let firstLine = self.getNextLine();
        println!("{}", firstLine);

        // check DOOM3 proc file
        if firstLine!="mapProcFile003"
        {
            // it is not a DOOM3 proc file
            // check if QUAKE4 proc file

            if firstLine!="PROC \"4\""
            {
                // not QUAKE4 or DOOM3 proc file
                // we exit   
                exit(-1);
            }
            self.iType = 4;    
        }
        else 
        {
            self.iType = 3;    
        }

        match self.iType
        {
            3   =>  {println!("DOOM3 proc file");   theLogFile.log("DOOM3 proc file\n".to_string());        /*fs::write("log.txt", "DOOM3 proc file") .expect("Could not write to log file");*/  },
            4   =>  {println!("QUAKE4 proc file");  theLogFile.log("QUAKE4 proc file\n".to_string());       /*fs::write("log.txt", "QUAKE4 proc file") .expect("Could not write to log file");*/ },
            _   =>  exit(-1),
        }
    }

    pub fn extractModelBlock(&mut self, theLogFile:&mut myLogFile, uiStart:usize, uiStop:usize)
    {
            let mut uiOffset = uiStart;

            //println!("extractModelBlock");
            theLogFile.log("extractModelBlock ".to_string());
            //theLogFile.log(sBlock.to_string());

            let sName = self.getNextString(uiOffset, uiStop);
            theLogFile.log(sName.0.to_string());
            uiOffset = sName.1;

            //theLogFile.log("\n".to_string());
            theLogFile.log(", numSurfaces = ".to_string());

            let dNumSurfaces = self.getNextNumber(uiOffset, uiStop);
            let iNumberOfSurfaces = dNumSurfaces.0 as usize;
            uiOffset = dNumSurfaces.1;

            theLogFile.log(dNumSurfaces.0.to_string());
            //theLogFile.log("\nSurfaces :\n".to_string());
            theLogFile.log("\n".to_string());

            let mut uiSurfaceLoopIndex =0;
            if iNumberOfSurfaces==0  {return;}
            loop 
            {
                let sSurfaceName = self.getNextString(uiOffset, uiStop);
                let sMessage = format!("Surface[{}] materialName = ", uiSurfaceLoopIndex);
                theLogFile.log(sMessage);
                theLogFile.log(sSurfaceName.0.to_string());
                uiOffset = sSurfaceName.1;

                let dNumVerts = self.getNextNumber(uiOffset, uiStop);
                let iNumberOfVertices = dNumVerts.0 as usize;
                theLogFile.log(", iNumberOfVertices = ".to_string());
                theLogFile.log(iNumberOfVertices.to_string());
                uiOffset = dNumVerts.1;
    
                let dNumIndexes = self.getNextNumber(uiOffset, uiStop);
                let iNumberOfIndices = dNumIndexes.0 as usize;
                theLogFile.log(", dNumberOfIndices = ".to_string());
                theLogFile.log(iNumberOfIndices.to_string());
                uiOffset = dNumIndexes.1;
                theLogFile.log("\n".to_string());

                if iNumberOfVertices>0
                {
                    let mut iNumberOfVerticesLoopCounter =0;
                    loop 
                    {
                        for i in 0..8
                        {
                            let coord = self.getNextNumber(uiOffset, uiStop);
                            if i!=0 {theLogFile.log(",".to_string());}
                            theLogFile.log(coord.0.to_string());
                            theLogFile.log(" ".to_string());
                            uiOffset = coord.1;
                        }
                        theLogFile.log("\n".to_string());

                        iNumberOfVerticesLoopCounter+=1;
                        if iNumberOfVerticesLoopCounter>=iNumberOfVertices    {break;};
                    }    
                }
            
                if iNumberOfIndices>0
                {
                    let mut iNumberOfIndicesLoopCounter =0;
                    loop 
                    {
                        let id = self.getNextNumber(uiOffset, uiStop);
                        if iNumberOfIndicesLoopCounter!=0 {theLogFile.log(",".to_string());}
                        theLogFile.log(id.0.to_string());
                        theLogFile.log(" ".to_string());
                        uiOffset = id.1;

                        iNumberOfIndicesLoopCounter+=1;
                        if iNumberOfIndicesLoopCounter>=iNumberOfIndices    {break;};
                    }
                    theLogFile.log("\n".to_string());

                }

                theLogFile.log("\n".to_string());
                uiSurfaceLoopIndex += 1;

                if uiSurfaceLoopIndex>=iNumberOfSurfaces    {break;};     
            };
    }

    pub fn extractInterreaPortalsBlock(&mut self, theLogFile:&mut myLogFile, uiStart:usize, uiStop:usize)
    {
            //println!("extractModelBlock");
            theLogFile.log("extractInterreaPortalsBlock\n".to_string());
            //theLogFile.log(sBlock.to_string());
    }

    pub fn extractNodesBlock(&mut self, theLogFile:&mut myLogFile, uiStart:usize, uiStop:usize)
    {
            //println!("extractModelBlock");
            theLogFile.log("extractModelBlock\n".to_string());
            //theLogFile.log(sBlock.to_string());
    }

    pub fn extractShadowModelBlock(&mut self, theLogFile:&mut myLogFile, uiStart:usize, uiStop:usize)
    {
            //println!("extractModelBlock");
            theLogFile.log("extractShadowModelBlock\n".to_string());
            //theLogFile.log(sBlock.to_string());
    }

    pub fn displayBlockData(&mut self, uiStart:usize, uiStop:usize)
    {
            let mut uiIndex =uiStart;
            let mut sString = "".to_string();

            loop 
            {
                //print!("{}", self.inputBuffer[uiIndex]);
                let key:char = self.inputBuffer[uiIndex] as char;
                sString += &key.to_string();

                if  uiIndex > uiStop
                {
                    println!("{}", sString);
                    break;    
                }
                uiIndex +=1;    
            };
    }

    pub fn extractData(&mut self, theLogFile:&mut myLogFile)
    {
        self.checkFirstLine(theLogFile);

        loop
        {
            let sBlockName = self.getNextBlockName();
            //println!("{}", sBlockName);
            let sBlockData = self.getNextBlockData();
            //println!("{}", sBlockData);

            /*
            println!("{}", sBlockData.0);
            println!("{}", sBlockData.1);
            self.displayBlockData(sBlockData.0, sBlockData.1);
            */

            match &sBlockName as &str 
            {
                "model"             =>  self.extractModelBlock(theLogFile, sBlockData.0, sBlockData.1),
                "interreaPortals"   =>  self.extractInterreaPortalsBlock(theLogFile, sBlockData.0, sBlockData.1),       //{self.logFile.log("ToDo :: extract interreaPortals\n".to_string());     /*println!("ToDo :: extract interreaPortals");   fs::write("log.txt", "ToDo :: extract interreaPortals") .expect("Could not write to log file");*/ },
                "nodes"             =>  self.extractNodesBlock(theLogFile, sBlockData.0, sBlockData.1),                 //{self.logFile.log("ToDo :: extract nodes\n".to_string());               /*println!("ToDo :: extract nodes");             fs::write("log.txt", "ToDo :: extract nodes") .expect("Could not write to log file");*/ },
                "shadowModel"       =>  self.extractShadowModelBlock(theLogFile, sBlockData.0, sBlockData.1),           //{self.logFile.log("ToDo :: extract shadowModel\n".to_string());         /*println!("ToDo :: extract shadowModel");       fs::write("log.txt", "ToDo :: extract shadowModel") .expect("Could not write to log file");*/ },
                _                   =>  break,  //{println!("Unknown block name {}", sBlockName); exit(-2);},
            }
            
        }

        /*
        let mut nextString = self.getNextString();   
        println!("{}", nextString);
        let mut nextNumber = self.getNextNumber();   
        println!("{}", nextNumber);
        let mut nextString = self.getNextString();   
        println!("{}", nextString);
        let mut nextNumber = self.getNextNumber();   
        println!("{}", nextNumber);
        */
    }

    pub fn getNextLine(&mut self) -> String
    {
        let mut     c:u8;
        //let mut   iSize = 0;
        let mut     sLine: String = "".to_string();
    
            if self.readOffset >= self.uiSize
            {
                self.bEOF = true;
                return sLine;
            }
    
            loop
            {
                c = self.inputBuffer[self.readOffset];
                if c == 0u8 {break};

                self.readOffset +=1;
                if self.readOffset >= self.uiSize
                {
                    self.bEOF = true;
                    return sLine;
                }
    
                //iSize+=1;
    
                if c == 10 && self.inputBuffer[self.readOffset] == 13
                {
                    //printf("cas 1013\n");
                    //iSize+=1;	// to skip 13
                    break;
                }
                if c == 13 && self.inputBuffer[self.readOffset] == 10
                {
                    //printf("cas 1310\n");
                    //iSize+=1;	// to skip 10
                    break;
                }
                if c == 10 && self.inputBuffer[self.readOffset] != 13
                {
                    //printf("cas 10\n");
                    break;
                }
                if c == 13 && self.inputBuffer[self.readOffset] != 10
                {
                    //printf("cas 13\n");
                    break;
                }
                let key:char = c as char;
                //let sDummy:&str= &c.to_string();
                sLine += &key.to_string();
            };
    
        sLine
    }

    pub fn getNextString(&mut self, uiStart:usize, uiStop:usize) -> (String,usize)
    {
        //let mut   iStatus:i32 = 0;
		let mut     c:u8;
        let mut     iBracket:i32 = 0;
		let mut     sNextString:String = "".to_string();
        let mut     uiOffset = uiStart;

		loop
		{
			c = self.inputBuffer[uiOffset /*self.readOffset*/];
            if c == 0u8 {break};

            uiOffset +=1;   //self.readOffset +=1;

			if c == b'"'	    {iBracket+=1;}
			if iBracket == 2	{break;}

			if iBracket == 1
			{
				if c != b'"'	
                {
                    let key:char = c as char;
                    sNextString += &key.to_string();
                }
			}

            //if self.readOffset >= self.uiSize
            if uiOffset > uiStop
            {
                self.bEOF = true;
                return (sNextString, uiOffset);
            }

		}

		return (sNextString, uiOffset);
    }

    pub fn getNextNumber(&mut self, uiStart:usize, uiStop:usize) -> (f64,usize)
    {
        //let mut   iStatus:i32     = 0;
		let mut     c:u8;
        //let mut   iComment:i32    = 0;
        let mut     iDejaVu:i32     = 0;
        let mut     iIsANumber:i32;
        //char		caDummy[10];
        let mut     sNumberAsString:String = "".to_string();
        let mut     uiOffset = uiStart;

    
            loop
            {
                c = self.inputBuffer[uiOffset /*self.readOffset*/];
                if c == 0u8 {break};

                //if self.readOffset >= self.uiSize
                if uiOffset > uiStop
                {
                    self.bEOF = true;
                    let Result:f64  = sNumberAsString.parse().unwrap();
                    return (Result, uiOffset);
                }
    
                if      c == b'/' 
                    &&  self.inputBuffer[uiOffset+1 /*self.readOffset +1*/] == b'*'
                {
                    loop
                    {
                        if      c == b'*' 
                            &&  self.inputBuffer[uiOffset+1 /*self.readOffset +1*/] == b'/'   	{break;}

                        //self.readOffset +=1;
                        uiOffset+=1;

                        c = self.inputBuffer[uiOffset /*self.readOffset*/];
                    }
                }
    
                c = self.inputBuffer[uiOffset /*self.readOffset*/];
                    
                if (c >= b'0' && c <= b'9') || c == b'.' || c == b'-'
                {
                    iIsANumber = 1;
                }
                else
                {
                    iIsANumber = 0;
                }
    
                if iIsANumber == 1 && iDejaVu == 0
                {
                    iDejaVu = 1;
                }
    
                if iIsANumber == 0 && iDejaVu == 1
                {
                    break;
                }
    
                if iDejaVu == 1
                {
                    let key:char = c as char;
                    sNumberAsString += &key.to_string();
                }
    
                //self.readOffset +=1;
                uiOffset+=1;
            }
    
        let Result:f64  = sNumberAsString.parse().unwrap();
        return (Result, uiOffset);
    }

    pub fn getNextBlockName(&mut self) -> String
    {
        let mut     c:u8;
        let mut     sBlockName:String = "".to_string();
    
        loop
        {
            if self.readOffset >= self.uiSize
            {
                self.bEOF = true;
                return sBlockName;
            }

            c = self.inputBuffer[self.readOffset];
           
            
            if c == b'{'	{break;}

            if      c > b'A'
                &&  c < b'z'
            {
                let key:char = c as char;
                sBlockName += &key.to_string();
            }
            
            self.readOffset +=1;
        }
        
        return	sBlockName;
    }

    pub fn getNextBlockData(&mut self) -> (usize, usize)
    {
        let mut     c:u8;
        //let mut   sBlockData:String = "".to_string();
        let mut		iIndent = 0;
        let mut	    bDejaVu = false;
        let mut	    bSendToOutput;
        let mut     uiStartIndex:usize =0;
        let mut     uiStopIndex:usize =0;
        
        loop
        {
            if self.readOffset >= self.uiSize
            {
                self.bEOF = true;
                return (uiStartIndex, uiStopIndex);
                //return sBlockData;
            }

            c = self.inputBuffer[self.readOffset];
           
            bSendToOutput = true;

            if	c == b'{'
            { 
                if bDejaVu == false
                {
                    bSendToOutput = false;
                    uiStartIndex = self.readOffset;
                }

                bDejaVu=true;
                iIndent+=1; 
            }

            if	c == b'}'
            { 
                iIndent-=1; 
            }
            
            if bDejaVu == true && iIndent == 0
            {
                break;
            }
                        
            if bSendToOutput == true
            {
            //    let key:char = c as char;
            //    sBlockData += &key.to_string();
                uiStopIndex = self.readOffset;
            }

            self.readOffset +=1;
        }

        //return	sBlockData;
        return (uiStartIndex, uiStopIndex);
                
    }
}

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


fn main() 
{
    let mut logFile = myLogFile::open();
    //logFile.log("sMessageToLog\n".to_string());
    //logFile.log("sMessageToLog\n".to_string());

    welcomeBanner(&mut logFile);
    let mut ourProcFile = mapProcFile::open(&mut logFile, "maps\\admin.proc");

    ourProcFile.extractData(&mut logFile);    
    let mut ourMtfFile = mtrFile::open(&mut logFile, "materials\\base_floor.mtr");
    return;
    
}



fn welcomeBanner(logFile:&mut myLogFile)
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