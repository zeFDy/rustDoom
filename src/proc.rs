#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::{fs, usize};
use std::process::exit;
use crate::logfile::myLogFile;
use crate::rustDoom::LOG_MODEL_DETAILS;


pub struct Vertex
{
    x   :f64,
    y   :f64,
    z   :f64,
    nx  :f64,
    ny  :f64,
    nz  :f64,
    s   :f64,
    t   :f64,
}

pub struct Surface
{
    MaterialName    : String,
    Vertices        : Vec<Vertex>,
    Indices         : Vec<usize>,
}

pub struct Model
{
    Name        : String,
    Surfaces    : Vec<Surface>,
}

impl Model
{
    pub fn DebugDisplay(&mut self, theLogFile:&mut myLogFile)
    {
        theLogFile.log("Model - DebugDisplay()\n".to_string());
        let sMessage = format!("ModelName = {}\n", self.Name);
        theLogFile.log(sMessage);
                
        let SurfaceCount =self.Surfaces.len();

        if  SurfaceCount==0 {return;};
        let mut SurfaceIndex =0;
        loop 
        {
            if SurfaceIndex>= SurfaceCount  {break;}
            
            let errorMessage = format!("Could't get thisSurface {}", SurfaceIndex);
            let thisSurface:&Surface = self.Surfaces.get(SurfaceIndex).expect(&errorMessage);
            let thisSurfaceMaterialName = thisSurface.MaterialName.clone();

            let sMessage = format!("Surface[{}] materialName = ", SurfaceIndex);
            theLogFile.log(sMessage);
            theLogFile.log(thisSurfaceMaterialName);
            theLogFile.log("\n".to_string());

            let VerticesCount   = thisSurface.Vertices.len();
            let IndicesCount    = thisSurface.Indices.len();
            
            theLogFile.log("iNumberOfVertices = ".to_string());
            theLogFile.log(VerticesCount.to_string());
            theLogFile.log("\n".to_string());

            let VexList:&Vec<Vertex> = &thisSurface.Vertices;
            for (i,vex) in VexList.into_iter().enumerate()
            {
                let sMessage = format!("[{:4}] {:8.2} {:8.2} {:8.2} {:8.2} {:8.2} {:8.2} {:8.2} {:8.2}\n", i, vex.x, vex.y, vex.z, vex.nx, vex.ny, vex.nz, vex.s, vex.t);
                theLogFile.log(sMessage);
            }

            theLogFile.log("dNumberOfIndices = ".to_string());
            theLogFile.log(IndicesCount.to_string());
            theLogFile.log("\n".to_string());

            let IndexList:&Vec<usize> = &thisSurface.Indices;
            //for index in IndexList.into_iter()
            for (i, index) in IndexList.into_iter().enumerate()
            {
                let sMessage = format!("{:3} ", index);
                theLogFile.log(sMessage);
                if (i+1)%16 == 0 {theLogFile.log("\n".to_string());};
            }
            theLogFile.log("\n".to_string());

            SurfaceIndex += 1;
        }
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

    pub fn extractModelBlock(&mut self, theLogFile:&mut myLogFile, uiStart:usize, uiStop:usize) -> Model
    {
            let mut uiOffset = uiStart;
            let mut SurfaceList:Vec<Surface> = Vec::new();

            //println!("extractModelBlock");
            //theLogFile.log("extractModelBlock ".to_string());
            //theLogFile.log(sBlock.to_string());

            let sName = self.getNextString(uiOffset, uiStop);
            //theLogFile.log(sName.0.to_string());
            uiOffset = sName.1;

            //theLogFile.log("\n".to_string());
            //theLogFile.log(", numSurfaces = ".to_string());

            let dNumSurfaces = self.getNextNumber(uiOffset, uiStop);
            let iNumberOfSurfaces = dNumSurfaces.0 as usize;
            uiOffset = dNumSurfaces.1;

            //theLogFile.log(dNumSurfaces.0.to_string());
            //theLogFile.log("\nSurfaces :\n".to_string());
            //theLogFile.log("\n".to_string());

            let mut uiSurfaceLoopIndex =0;
            if iNumberOfSurfaces==0  
            {
                let thisModel = Model
                {
                    Name            : sName.0,
                    Surfaces        : SurfaceList,
                };
    
                return thisModel;
            }

            loop 
            {
                
                let sSurfaceName = self.getNextString(uiOffset, uiStop);
                //let sMessage = format!("Surface[{}] materialName = ", uiSurfaceLoopIndex);
                //theLogFile.log(sMessage);
                //theLogFile.log(sSurfaceName.0.to_string());
                uiOffset = sSurfaceName.1;

                let dNumVerts = self.getNextNumber(uiOffset, uiStop);
                let iNumberOfVertices = dNumVerts.0 as usize;
                //theLogFile.log(", iNumberOfVertices = ".to_string());
                //theLogFile.log(iNumberOfVertices.to_string());
                uiOffset = dNumVerts.1;
    
                let dNumIndexes = self.getNextNumber(uiOffset, uiStop);
                let iNumberOfIndices = dNumIndexes.0 as usize;
                //theLogFile.log(", dNumberOfIndices = ".to_string());
                //theLogFile.log(iNumberOfIndices.to_string());
                uiOffset = dNumIndexes.1;
                //theLogFile.log("\n".to_string());

                let mut theVertices:Vec<Vertex> = Vec::new();
                let mut theIndices:Vec<usize>   = Vec::new();

                if iNumberOfVertices>0
                {
                    let mut iNumberOfVerticesLoopCounter =0;
                    loop 
                    {

                        let coord = self.getNextNumber(uiOffset, uiStop);
                        let dx = coord.0;
                        uiOffset = coord.1;
                        let coord = self.getNextNumber(uiOffset, uiStop);
                        let dy = coord.0;
                        uiOffset = coord.1;
                        let coord = self.getNextNumber(uiOffset, uiStop);
                        let dz = coord.0;
                        uiOffset = coord.1;

                        let coord = self.getNextNumber(uiOffset, uiStop);
                        let dnx = coord.0;
                        uiOffset = coord.1;
                        let coord = self.getNextNumber(uiOffset, uiStop);
                        let dny = coord.0;
                        uiOffset = coord.1;
                        let coord = self.getNextNumber(uiOffset, uiStop);
                        let dnz = coord.0;
                        uiOffset = coord.1;

                        let coord = self.getNextNumber(uiOffset, uiStop);
                        let ds = coord.0;
                        uiOffset = coord.1;
                        let coord = self.getNextNumber(uiOffset, uiStop);
                        let dt = coord.0;
                        uiOffset = coord.1;

                        let vert:Vertex = Vertex
                        {
                            x       :dx,
                            y       :dy,
                            z       :dz,

                            nx      :dnx,
                            ny      :dny,
                            nz      :dnz,

                            s       :ds,
                            t       :dt,
                        };

                        /*/
                        for i in 0..8
                        {
                            let coord = self.getNextNumber(uiOffset, uiStop);
                            if i!=0 {theLogFile.log(",".to_string());}
                            theLogFile.log(coord.0.to_string());
                            theLogFile.log(" ".to_string());
                            uiOffset = coord.1;
                        }
                        */
                        theVertices.push(vert);

                        //theLogFile.log("\n".to_string());

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
                        //if iNumberOfIndicesLoopCounter!=0 {theLogFile.log(",".to_string());}
                        //theLogFile.log(id.0.to_string());
                        //theLogFile.log(" ".to_string());
                        uiOffset = id.1;

                        theIndices.push(id.0 as usize);
                        iNumberOfIndicesLoopCounter+=1;
                        if iNumberOfIndicesLoopCounter>=iNumberOfIndices    {break;};
                    }
                    //theLogFile.log("\n".to_string());

                }

                let thisSurface = Surface
                {
                    MaterialName    : sSurfaceName.0.to_string(),
                    Vertices        : theVertices,
                    Indices         : theIndices,
                };
                
                SurfaceList.push(thisSurface);

                //theLogFile.log("\n".to_string());
                uiSurfaceLoopIndex += 1;

                if uiSurfaceLoopIndex>=iNumberOfSurfaces    {break;};     
            };

            let thisModel = Model
            {
                Surfaces            : SurfaceList,
                Name                : sName.0,
            };

            //theLogFile.log("\n".to_string());
            thisModel
    }

    pub fn extractInterreaPortalsBlock(&mut self, theLogFile:&mut myLogFile, uiStart:usize, uiStop:usize)
    {
            //println!("extractModelBlock");
            //theLogFile.log("extractInterreaPortalsBlock\n".to_string());
            //theLogFile.log(sBlock.to_string());
    }

    pub fn extractNodesBlock(&mut self, theLogFile:&mut myLogFile, uiStart:usize, uiStop:usize)
    {
            //println!("extractModelBlock");
            //theLogFile.log("extractModelBlock\n".to_string());
            //theLogFile.log(sBlock.to_string());
    }

    pub fn extractShadowModelBlock(&mut self, theLogFile:&mut myLogFile, uiStart:usize, uiStop:usize)
    {
            //println!("extractModelBlock");
            //theLogFile.log("extractShadowModelBlock\n".to_string());
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

        let ModelList:Vec<Model> = Vec::new();

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
                "model"             =>  {   
                                            let mut thisModel = self.extractModelBlock(theLogFile, sBlockData.0, sBlockData.1);
                                            if LOG_MODEL_DETAILS == true {thisModel.DebugDisplay(theLogFile);}
                                        },
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
