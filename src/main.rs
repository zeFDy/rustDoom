#![allow(non_snake_case)]
#![allow(non_camel_case_types)]


use std::fs;
use colored::Colorize;
use std::process::exit;

pub struct mapProcFile
{
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
    pub fn open(fileName:&str) -> mapProcFile
    {
        let ucBuffer    = fs::read(fileName).expect("Can't read file");
        let bufferSize = ucBuffer.len();

        let mut ourFile = mapProcFile
        {
            //pathProcFile  : fileName.to_string(),
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

    pub fn checkFirstLine(&mut self)
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
            3   =>  println!("DOOM3 proc file"),
            4   =>  println!("QUAKE4 proc file"),
            _   =>  exit(-1),
        }
    }

    pub fn extractData(&mut self)
    {
        self.checkFirstLine();
        
    }

    pub fn getNextLine(&mut self) -> String
    {
        let mut     c:u8;
        let mut		iSize = 0;
        let mut     sLine: String = "".to_string();
    
            if self.readOffset >= self.uiSize
            {
                self.bEOF = true;
                return(sLine);
            }
    
            loop
            {
                c = self.inputBuffer[self.readOffset];
                if c == 0u8 {break};

                self.readOffset +=1;
                if self.readOffset >= self.uiSize
                {
                    self.bEOF = true;
                    return(sLine);
                }
    
                iSize+=1;
    
                if c == 10 && self.inputBuffer[self.readOffset] == 13
                {
                    //printf("cas 1013\n");
                    iSize+=1;	// to skip 13
                    break;
                }
                if c == 13 && self.inputBuffer[self.readOffset] == 10
                {
                    //printf("cas 1310\n");
                    iSize+=1;	// to skip 10
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

    pub fn getNextString() -> String
    {
        "".to_string()
    }

    pub fn getNextNumber() -> f64
    {
        0.00
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
    welcomeBanner();
    let mut ourProcFile = mapProcFile::open("maps\\admin.proc");
    ourProcFile.extractData();    

    return;
    
}



fn welcomeBanner()
{
    print!("\n");
    print!("                                       \\");
    print!("{}", "o".red());
    print!("{}", ".".normal());
    print!("{}", "o".red());
    print!("{}", "/\n".normal());
    print!("---------------------------------.ooO---(_)---Ooo.----------\n");
    print!("rustDoom\n");
    print!("\n");
    print!("--------------------------------------------Oooo.-----------\n");
    print!("                                     .oooO  (   )\n");
    print!("                                     (   )   ) /\n");
    print!("                                      \\ (   (_/\n");
    print!("                                       \\_)\n");
    print!("\n");
}