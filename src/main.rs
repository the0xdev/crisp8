// SPDX-FileCopyrightText: 2025 Imran M <imran@imranmustafa.net>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::io::BufReader;
use std::io::Error;
use std::ops::Range;
use std::io::prelude::*;
use std::fs::File;
use rand::Rng;


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
	println!("0x{:x} | {:?}", self.program_counter, self.read_op_string());
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
	let bytes = self.read_op_array();
	macro_rules! encode {
	    (nn) => {
		(bytes[2] << 4) + bytes[3]
	    };
	    (nnn) => {
		((bytes[1] as u16) << 8) + (encode!(nn) as u16)
	    };
	    (x) => {
		bytes[1]
	    };
	    (y) => {
		bytes[2]
	    };
	}
	macro_rules! reg {
	    (x) => {
		self.variable_register[encode!(x) as usize]
	    };
	    (y) => {
		self.variable_register[encode!(y) as usize]
	    };
	    (f) => {
		self.variable_register[NUMOFREGISTERS - 1]
	    };
	}
	macro_rules! sigbit {
	    ($byte:expr, l) => {
		$byte & 0b00000001
	    };
	    ($byte:expr, m) => {
		($byte & 0b10000000).rotate_left(1)
	    };

	}
	match bytes[0] {
	    0x0 => match encode!(nnn) {
		0x0e0 => todo!(),
		0x0ee => todo!(),
		_ => todo!(),
	    },
	    0x1 => {
		todo!()	
	    },
	    0x2 => {
		todo!()	
	    },
	    0x3 => {
		todo!()
	    },
	    0x4 => {
		todo!()
	    },
	    0x5 => {
		todo!()
	    },
	    0x6 => {
		reg!(x) = encode!(nn);
	    },
	    0x7 => todo!(),
	    0x8 => match bytes[3] {
		0x0 => {
		    reg!(x) = reg!(y);
		},
		0x1 => {
		    reg!(x) |= reg!(y);
		},
		0x2 => {
		    reg!(x) &= reg!(y);
		},
		0x3 => {
		    reg!(x) ^= reg!(y);
		},
		0x4 => 
		    if let Some(i) = reg!(x).checked_add(reg!(y)) {
			reg!(x) = i;
			reg!(f) = 0;
		    } else {
			reg!(x) = reg!(x).wrapping_add(reg!(y));
			reg!(f) = 1;
		    }
		,
		0x5 =>
		    if let Some(i) = reg!(x).checked_sub(reg!(y)) {
			reg!(x) = i;
			reg!(f) = 0;
		    } else {
			reg!(x) = reg!(x).wrapping_sub(reg!(y));
			reg!(f) = 1;
		    }
		,
		0x6 => {
		    reg!(f) = sigbit!(reg!(x), l);
		    reg!(x) >>= 1;
		},
		0x7 =>
		    if let Some(i) = reg!(y).checked_sub(reg!(x)) {
			reg!(x) = i;
			reg!(f) = 0;
		    } else {
			reg!(x) = reg!(y).wrapping_sub(reg!(x));
			reg!(f) = 1;
		    }
		,
		0xe => {
		    reg!(f) = sigbit!(reg!(x), m);
		    reg!(x) <<= 1;
		},
		_ => todo!(),
	    },
	    0x9 => {
		todo!()	
	    },
	    0xa => {
		self.i = encode!(nnn);
	    },
	    0xb => {
		todo!()
	    },
	    0xc => {
		let mut rng = rand::rng();
		self.variable_register[bytes[1] as usize] = rng.random::<u8>() & encode!(nn)
		
	    },
	    0xd => {
		todo!()
	    },
	    0xe => match encode!(nn) {
		0x9e => {todo!()},
		0xa1 => {todo!()},
		_ => todo!(),
	    },
	    0xf => match encode!(nn) {
		0x0a => {todo!()},
		0x1e => {
		   self.i += reg!(x) as u16;
		},
		0x07 => {todo!()},
		0x15 => {todo!()},
		0x18 => {todo!()},
		0x29 => {todo!()},
		0x33 => {todo!()},
		0x55 => {todo!()},
		0x65 => {todo!()},
		_ => todo!(),
	    },
	    _ => todo!(),
	}
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
