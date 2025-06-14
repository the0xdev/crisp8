// SPDX-FileCopyrightText: 2025 Imran M <imran@imranmustafa.net>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::io::BufReader;
use std::io::Error;
use std::ops::Range;
use std::io::prelude::*;
use std::fs::File;
use std::io;
use std::thread::sleep;
use std::time;
use eframe::egui::vec2;
use eframe::egui::Color32;
use eframe::egui::CornerRadius;
use eframe::egui::Rect;
use eframe::egui::Sense;
use eframe::egui::Stroke;
use crate::egui::Vec2;
use crate::egui::pos2;
use rand::Rng;
use std::io::stdout;
use std::io::stdin;

use std::f32::consts::TAU;

use eframe::{
    egui,
};

const SIZE: usize = 0x1000;

const PROGRAM_START: usize = 0x200;
const PROGRAM_END: usize = 0xea0;

const PROGRAM_SIZE: usize = PROGRAM_END - PROGRAM_END;
const PROGRAM_RANGE: Range<usize> = PROGRAM_START..PROGRAM_END;

const DISPLAY_HEIGHT: usize = 32;
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
    display: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
    variable_register: [u8; NUMOFREGISTERS],
    isa: ISA
}
use std::io::{Write};

fn pause() {
    print!("press enter to contiue... ");
    let _ = stdout().flush();
    let _: Option<i32> = std::io::stdin()
	.bytes() 
	.next()
	.and_then(|result| result.ok())
	.map(|byte| byte as i32);
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
		print!("{}", if jj {"1"} else {"0"});
	    }
	    println!();
	}
    }
    fn clear_terminal(&mut self) {
	print!("{}[2J", 27 as char);
    }

    fn clear(&mut self) {
	self.display = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
    }

    pub fn run(&mut self) {
	loop {
	    let _ = &self.cycle();
	    // println!("0x{:X} | {:?}", self.program_counter, self.read_op_string());
	//     for reg in self.variable_register.iter().enumerate() {
	// 	println!("reg[{:X}] | 0x{:X}", reg.0, reg.1);
	// }
	//     println!("\nI | 0x{:X}", self.i);
	//     println!("Stack:");
	//     for adrr in &self.stack {
	// 	println!("\t0x{:X}", adrr);
	// }

	    self.draw();
	    stdout().flush();
	    sleep(time::Duration::from_millis(150));
	    self.clear_terminal();
	    // pause();
	}
		
    }

    fn cycle(&mut self) {
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
		self.program_counter = encode!(nnn) - 2;
	    },
	    0x2 => {
		self.stack.push(self.program_counter);
		self.program_counter = encode!(nnn) - 2;
	    },
	    0x3 => if reg!(x) == encode!(nn) {
		self.program_counter += 2;
	    },
	    0x4 => if reg!(x) != encode!(nn) {
		self.program_counter += 2;
	    },
	    0x5 => if reg!(x) == reg!(y) {
		self.program_counter += 2;
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
		self.program_counter = encode!(nnn) + reg!(0) as u16 - 2;
	    },
	    0xc => {
		let mut rng = rand::rng();
		self.variable_register[bytes[1] as usize] = rng.random::<u8>() & encode!(nn)
	    },
	    0xd => {
		macro_rules! byte_to_bool {
		    ($b:expr, $n:expr) => {
			($b & 0b10000000_u8.rotate_right($n)) != 0
		    };
		}
		reg!(f) = 0;

		for ii in 0..encode!(z) {
		    let sprite_byte = self.memory[self.i as usize + ii as usize];
		    for jj in 0..8 {
			self.display[(reg!(y) + ii) as usize][reg!(x) as usize] ^= byte_to_bool!(sprite_byte, jj);
		    }

		}
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
    display: Option<[[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT]>,
    variable_register: Option<[u8; NUMOFREGISTERS]>,
    isa: Option<ISA>,
}

impl CrispBuilder {
    pub fn new() -> CrispBuilder {
	CrispBuilder {
	    program_counter: Some(0x200),
	    stack: Some(Vec::with_capacity(NUMOFREGISTERS)),
	    display: Some([[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT]),
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
	    display: build!(display, [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT]),
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
    let mut c = CrispBuilder::new().load_file("test2.bin").expect("File error").build();
    c.run();
    eframe::run_native(
        "crisp8",
        options,
        Box::new(|cc| {
            // This gives us image support:
            Ok(Box::new(c))
        }),
    )
}


impl eframe::App for Crisp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            //ui.heading("My egui Application");
	    // Create a "canvas" for drawing on that's 100% x 300px
	    let (response, painter) = ui.allocate_painter(
		eframe::egui::Vec2::new(640.0, 320.0),
		Sense::hover()
	    );

	    let rect = response.rect;
	    let c = rect.center();
	    let r = rect.width() / 2.0 - 1.0;
	    let color = Color32::from_gray(128);
	    let stroke = Stroke::new(1.0, color);
	    for ii in 0..64 {
		for jj in 0..32 {
		    painter.rect_filled(
			Rect::from_min_size(
			    pos2(
				ii as f32 * 10.0_f32,
				jj as f32 * 10.0_f32
			    ),
			    Vec2::splat(10.0_f32)
			),
			CornerRadius::ZERO,
			{
			    if self.display[ii][jj] {
			    //if (ii + jj) % 2 == 0 {
				Color32::WHITE
			    } else {
				Color32::BLACK
			    }
			}
		    );
		}
	    }
	    //painter.rect_filled(Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), CornerRadius::ZERO, Color32::WHITE);
	    // painter.circle_stroke(c, r, stroke);
	    // painter.line_segment([c - vec2(0.0, r), c + vec2(0.0, r)], stroke);
	    // painter.line_segment([c, c + r * Vec2::angled(TAU * 1.0 / 8.0)], stroke);
	    // painter.line_segment([c, c + r * Vec2::angled(TAU * 3.0 / 8.0)], stroke);

	    // // Get the relative position of our "canvas"
	    // let to_screen = response.rect;
	    // // The line we want to draw represented as 2 points
	    // let first_point = Pos2 { x: 0.0, y: 0.0 };
	    // let second_point = Pos2 { x: 300.0, y: 300.0 };
	    // // Make the points relative to the "canvas"
	    // let first_point_in_screen = to_screen.transform_pos(first_point);
	    // let second_point_in_screen = to_screen.transform_pos(second_point);

	    // // Paint the line!
	    // painter.add(Shape::LineSegment {
	    // 	points: [first_point_in_screen, second_point_in_screen],
	    // 	stroke: Stroke {
	    // 	    width: 10.0,
	    // 	    color: Color32::BLUE,
	    // 	},
	    // });
        });
    }
}
