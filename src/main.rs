// SPDX-FileCopyrightText: 2025 Imran M <imran@imranmustafa.net>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::io::BufReader;
use std::io::Error;
use std::ops::Range;
use std::io::prelude::*;
use std::fs::File;
use std::io;
use rand::Rng;

use eframe::{
    egui,
};

const SIZE: usize = 0x1000;

const PROGRAM_START: usize = 0x200;
const PROGRAM_END: usize = 0xea0;

const PROGRAM_SIZE: usize = PROGRAM_END - PROGRAM_END;
const PROGRAM_RANGE: Range<usize> = PROGRAM_START..PROGRAM_END;

const DISPLAY_HEIGHT: usize = 64;
const DISPLAY_WIDTH: usize = 64;

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
    program_counter: u16,
    i: u16,
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    display: [[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
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
	let bytes = &self.memory[self.program_counter as usize..=self.program_counter as usize + 1];
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

    fn draw(&self) {
	for ii in self.display {
	    for jj in ii {
		print!("{:0>8b}", jj);
	    }
	    println!();
	}
    }

    fn clear(&self) {
	print!("{}[2J", 27 as char);
    }

    pub fn run(mut self) {
	self.clear();
	self.draw();
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
	    (w) => {
		bytes[0]
	    };
	    (x) => {
		bytes[1]
	    };
	    (y) => {
		bytes[2]
	    };
	    (z) => {
		bytes[3]
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
	    ($x:expr) => {
		self.variable_register[$x as usize]
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
		0x0e0 => {
		    self.clear();
		},
		0x0ee => {
		    self.program_counter = self.stack.pop().unwrap();
		},
		_ => todo!(),
	    },
	    0x1 => {
		self.program_counter = encode!(nnn)
	    },
	    0x2 => {
		self.stack.push(self.program_counter);
		self.program_counter = encode!(nnn)
	    },
	    0x3 => if reg!(x) == encode!(nn) {
		self.program_counter += 2
	    },
	    0x4 => if reg!(x) != encode!(nn) {
		self.program_counter += 2
	    },
	    0x5 => if reg!(x) == reg!(y) {
		self.program_counter += 2
	    },
	    0x6 => {
		reg!(x) = encode!(nn);
	    },
	    0x7 => {
		reg!(x) += encode!(nn);
	    },
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
	    0x9 => if reg!(x) != reg!(y) {
		self.program_counter += 2
	    },
	    0xa => {
		self.i = encode!(nnn);
	    },
	    0xb => {
		self.program_counter = encode!(nnn) + reg!(0) as u16;
	    },
	    0xc => {
		let mut rng = rand::rng();
		self.variable_register[bytes[1] as usize] = rng.random::<u8>() & encode!(nn)
	    },
	    0xd => {
		reg!(f) = 0;
		for ii in 0..encode!(z) {
		    if self.display[reg!(x) as usize][(reg!(y) + ii) as usize] & 0b1111_1111 > 0x00 {
			reg!(f) = 1;
		    } 
		    self.display[reg!(x) as usize][(reg!(y) + ii) as usize] ^= self.memory[(self.i + ii as u16) as usize];
		}
		self.clear();
		self.draw();
	    },
	    0xe => match encode!(nn) {
		0x9e => {todo!()},
		0xa1 => {todo!()},
		_ => todo!(),
	    },
	    0xf => match encode!(nn) {
		0x0a => {
		    let stdin = io::stdin();
		    reg!(x) = stdin.bytes().nth(0).unwrap().expect("no key");
		},
		0x1e => {
		   self.i += reg!(x) as u16;
		},
		0x07 => {
		    reg!(x) = self.delay_timer;
		},
		0x15 => {
		    self.delay_timer = reg!(x);
		},
		0x18 => {
		    self.sound_timer = reg!(x);
		},
		0x29 => {todo!()},
		0x33 => {
		    
		},
		0x55 => {
		    for ii in 0..=encode!(x) {
			self.memory[(self.i + (ii as u16)) as usize] = reg!(ii);
		    }
		},
		0x65 => {
		    for ii in 0..=encode!(x) {
			reg!(ii) = self.memory[(self.i + (ii as u16)) as usize];
		    }
		},
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
    program_counter: Option<u16>,
    i: Option<u16>,
    stack: Option<Vec<u16>>,
    delay_timer: Option<u8>,
    sound_timer: Option<u8>,
    display: Option<[[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT]>,
    variable_register: Option<[u8; NUMOFREGISTERS]>,
    isa: Option<ISA>,
}

impl CrispBuilder {
    pub fn new() -> CrispBuilder {
	CrispBuilder {
	    program_counter: Some(0x200),
	    stack: Some(Vec::with_capacity(NUMOFREGISTERS)),
	    display: Some([[0x00; DISPLAY_WIDTH]; DISPLAY_HEIGHT]),
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
	    display: build!(display, [[0x00; DISPLAY_WIDTH]; DISPLAY_HEIGHT]),
	    variable_register: build!(variable_register, [0; NUMOFREGISTERS]),
	    isa: build!(isa),
	}
    }
}

fn main() -> eframe::Result {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    let mut c = CrispBuilder::new().load_file("test.bin").expect("File error").build();
    // c.run();
    eframe::run_native(
        "crisp8",
        options,
        Box::new(|cc| {
            // This gives us image support:
            Ok(Box::new(c))
        }),
    )
}

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for Crisp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
        });
    }
}
