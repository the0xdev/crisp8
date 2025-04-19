// SPDX-FileCopyrightText: 2025 Imran M <imran@imranmustafa.net>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::io::BufReader;
use std::io::Error;
use std::ops::Range;
use std::io::prelude::*;
use std::fs::File;

const SIZE: usize = 0x1000;

const PROGRAM_START: usize = 0x200;
const PROGRAM_END: usize = 0xea0;

const PROGRAM_SIZE: usize = PROGRAM_END - PROGRAM_END;
const PROGRAM_RANGE: Range<usize> = PROGRAM_START..PROGRAM_END;

const NUMOFREGISTERS: usize = 16;

#[derive(Default, Debug, PartialEq, Copy, Clone)]
enum ISA {
    #[default]
    Chip8,
    SuperChip,
    MegaChip,
}

#[derive(Debug, PartialEq, Clone)]
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

// fn cycle(cur_state: Crisp) -> Crisp {
//     let op = &cur_state.memory[cur_state.program_counter..=cur_state.program_counter + 1];
//     let mut new_state: Crisp = cur_state.clone();

//     println!("0x{:x} | {:x?}", cur_state.program_counter, op);

//     new_state.program_counter += 2;
//     new_state
// }

// fn op_exe(state: Crisp, code: &[u8]) -> Result<Crisp, Error> {
//     todo!()
// }


impl Crisp {
    pub fn builder() -> CrispBuilder {
        CrispBuilder::default()
    }

    pub fn mem_dump(self) -> u8 {
        return self.memory[PROGRAM_START]
    }
    
    fn read_op_array(&self) -> [u8; 4] {
	let bytes = &self.memory[self.program_counter..=self.program_counter + 1];
	[
	    (bytes[0] & 0b11110000) >> 4,
	    bytes[0] & 0b00001111,
	    (bytes[1] & 0b11110000) >> 4,
	    bytes[1] & 0b00001111,
	]
    }

    fn read_op_string(&self) -> String {
	let op = self.read_op_array();
	format!("{:x}{:x}{:x}{:x}", op[0], op[1], op[2], op[3])
    }

    pub fn run(mut self) {
	for _ in 0..50 {
	    let _ = &self.cycle();
	}
    }

    fn cycle(&mut self) {
	println!("0x{:x} | {:?}", self.program_counter, self.read_op_array());
	match self.isa {
	    ISA::Chip8 => {
		self.chip8_op();
	    }
	    ISA::SuperChip => {
		todo!()
	    },
	    ISA::MegaChip => {
		todo!()
	    },
	}
    }

    fn chip8_op(&mut self) {
	self.program_counter += 2;
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
	    program_counter: Some(0x200),
	    stack: Some(Vec::with_capacity(NUMOFREGISTERS)),
	    ..Default::default()
	}
    }
    
    pub fn load(mut self, value: impl Into<String>) -> CrispBuilder {
	todo!()
    }
    pub fn load_file(mut self, value: impl Into<String>) -> Result<CrispBuilder, Error> {
	let file_name = value.into();
	let file = File::open(file_name)?;

	let mut reader = BufReader::new(file);
	let mut memory = [0; SIZE];

     	reader.read(&mut memory[PROGRAM_RANGE])?;
	self.memory = Some(memory);
	
        Ok(self)
    }

    pub fn load_font(mut self) -> CrispBuilder {
	todo!()
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
    let mut c = CrispBuilder::new().load_file("test.bin").expect("File error").build();
    c.run();
}
