use std::io::BufReader;
use std::io::Error;
use std::ops::Range;
use std::io::prelude::*;
use std::fs::File;

const SIZE: usize = 40_964;

const PROGRAM_START: usize = 512;
const PROGRAM_END: usize = 3744;

const PROGRAM_SIZE: usize = 3744 - 512;
const PROGRAM_RANGE: Range<usize> = PROGRAM_START..PROGRAM_END;

const NUMOFREGISTERS: usize = 16;


#[derive(Default, Debug, PartialEq)]
enum ISA {
    #[default]
    Chip8,
    SuperChip,
    MegaChip,
}

#[derive(Debug, PartialEq)]
pub struct Crisp {
    memory: [u8; SIZE],
    program_counter: usize,
    i: u16,
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    variable_register: [u8; NUMOFREGISTERS],
    isa: ISA
}

impl Crisp {
    pub fn builder() -> CrispBuilder {
        CrispBuilder::default()
    }

    pub fn mem_dump(self) -> u8 {
        return self.memory[PROGRAM_START]
    }
}

#[derive(Default)]
pub struct CrispBuilder {
    memory: Option<[u8; SIZE]>,
    program_counter: Option<usize>,
    i: Option<u16>,
    stack: Option<Vec<u16>>,
    delay_timer: Option<u8>,
    sound_timer: Option<u8>,
    variable_register: Option<[u8; NUMOFREGISTERS]>,
    isa: Option<ISA>,
}

impl CrispBuilder {
    
    pub fn new() -> CrispBuilder {
	CrispBuilder {
	    ..Default::default()
	}
    }
    pub fn load(mut self, value: impl Into<String>) -> Result<CrispBuilder, Error> {
	let file_name = value.into();
	let file = File::open(file_name)?;

	let mut reader = BufReader::new(file);
	let mut memory = [0; SIZE];

     	reader.read(&mut memory[PROGRAM_RANGE])?;
	self.memory = Some(memory);
	
        Ok(self)
    }

    pub fn build(self) -> Crisp {
	macro_rules! build {
	    ($i:ident) => {
		self.$i.unwrap_or_default()
	    };
	
	    ($i:ident, $or:expr) => {
		self.$i.unwrap_or($or)
	    };
	}
	Crisp {
	    memory: build!(memory, [0; SIZE]),
	    program_counter: build!(program_counter),
	    i: build!(i),
	    stack: build!(stack),
	    delay_timer: build!(delay_timer),
	    sound_timer: build!(sound_timer),
	    variable_register: build!(variable_register, [0; NUMOFREGISTERS]),
	    isa: build!(isa),
	}
    }
}

fn main() {
    let c = CrispBuilder::new().load("test.bin").expect("File error").build();
}
