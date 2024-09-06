#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::{fs, io::Write};
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

    pub fn extractModelBlock(&mut self, theLogFile:&mut myLogFile, sBlock:String)
    {
            //println!("extractModelBlock");
            theLogFile.log("extractModelBlock\n".to_string());
            theLogFile.log(sBlock.to_string());
    }

    pub fn extractInterreaPortalsBlock(&mut self, theLogFile:&mut myLogFile, sBlock:String)
    {
            //println!("extractModelBlock");
            theLogFile.log("extractInterreaPortalsBlock\n".to_string());
            theLogFile.log(sBlock.to_string());
    }

    pub fn extractNodesBlock(&mut self, theLogFile:&mut myLogFile, sBlock:String)
    {
            //println!("extractModelBlock");
            theLogFile.log("extractModelBlock\n".to_string());
            theLogFile.log(sBlock.to_string());
    }

    pub fn extractShadowModelBlock(&mut self, theLogFile:&mut myLogFile, sBlock:String)
    {
            //println!("extractModelBlock");
            theLogFile.log("extractShadowModelBlock\n".to_string());
            theLogFile.log(sBlock.to_string());
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
            
            match &sBlockName as &str 
            {
                "model"             =>  self.extractModelBlock(theLogFile, sBlockData),
                "interreaPortals"   =>  self.extractInterreaPortalsBlock(theLogFile, sBlockData),       //{self.logFile.log("ToDo :: extract interreaPortals\n".to_string());     /*println!("ToDo :: extract interreaPortals");   fs::write("log.txt", "ToDo :: extract interreaPortals") .expect("Could not write to log file");*/ },
                "nodes"             =>  self.extractNodesBlock(theLogFile, sBlockData),                 //{self.logFile.log("ToDo :: extract nodes\n".to_string());               /*println!("ToDo :: extract nodes");             fs::write("log.txt", "ToDo :: extract nodes") .expect("Could not write to log file");*/ },
                "shadowModel"       =>  self.extractShadowModelBlock(theLogFile, sBlockData),           //{self.logFile.log("ToDo :: extract shadowModel\n".to_string());         /*println!("ToDo :: extract shadowModel");       fs::write("log.txt", "ToDo :: extract shadowModel") .expect("Could not write to log file");*/ },
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

    pub fn getNextString(&mut self) -> String
    {
        //let mut   iStatus:i32 = 0;
		let mut     c:u8;
        let mut     iBracket:i32 = 0;
		let mut     sNextString:String = "".to_string();

		loop
		{
			c = self.inputBuffer[self.readOffset];
            if c == 0u8 {break};

            self.readOffset +=1;

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

            if self.readOffset >= self.uiSize
            {
                self.bEOF = true;
                return sNextString;
            }

		}

		sNextString
    }

    pub fn getNextNumber(&mut self) -> f64
    {
        //let mut   iStatus:i32     = 0;
		let mut     c:u8;
        //let mut   iComment:i32    = 0;
        let mut     iDejaVu:i32     = 0;
        let mut     iIsANumber:i32;
        //char		caDummy[10];
        let mut     sNumberAsString:String = "".to_string();
        
    
            loop
            {
                c = self.inputBuffer[self.readOffset];
                if c == 0u8 {break};

                if self.readOffset >= self.uiSize
                {
                    self.bEOF = true;
                    let Result:f64  = sNumberAsString.parse().unwrap();
                    return Result;
                }
    
                if      c == b'/' 
                    &&  self.inputBuffer[self.readOffset +1] == b'*'
                {
                    loop
                    {
                        if      c == b'*' 
                            &&  self.inputBuffer[self.readOffset +1] == b'/'   	{break;}

                        self.readOffset +=1;
                        c = self.inputBuffer[self.readOffset];
                    }
                }
    
                c = self.inputBuffer[self.readOffset];
                    
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
    
                self.readOffset +=1;
            }
    
        let Result:f64  = sNumberAsString.parse().unwrap();
        Result
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

    pub fn getNextBlockData(&mut self) -> String
    {
        let mut     c:u8;
        let mut     sBlockData:String = "".to_string();
        let mut		iIndent = 0;
        let mut	    bDejaVu = false;
        let mut	    bSendToOutput;
    
        loop
        {
            if self.readOffset >= self.uiSize
            {
                self.bEOF = true;
                return sBlockData;
            }

            c = self.inputBuffer[self.readOffset];
           
            bSendToOutput = true;

            if	c == b'{'
            { 
                if bDejaVu == false
                {
                    bSendToOutput = false;
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
                let key:char = c as char;
                sBlockData += &key.to_string();
            }

            self.readOffset +=1;
        }

        return	sBlockData;
    
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