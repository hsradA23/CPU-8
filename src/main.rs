#![allow(non_snake_case)]
#![allow(unused_variables)]
use std::fs;
use std::env;

const MAX_MEM: usize = 1024 * 64;

pub struct CPU{
	pub PC: usize,   // Program Counter
	pub SP: usize,   // Stack Pointer

	//REGISTERS
	pub A: usize,    // Accumulator
	pub X: usize,    // Index register X
	pub Y: usize,    // Index register Y

	//PROCESSOR STATUS
	pub CF: bool,    // Carry Flag
	pub ZF: bool,    // Zero Flag
	pub OF: bool,    // Overflow Flag
	pub NF: bool,    // Negative Flag
}

impl CPU{
	pub fn Reset(&mut self){
		self.PC = 100;
		self.SP = 0x00FF;

		self.A = 0;
		self.X = 0;
		self.Y = 0;

		self.CF = false; // Carry Flag
		self.ZF = false; // Zero Flag
		self.OF = false; // Overflow Flag
		self.NF = false; // Negative Flag
	}

	pub fn new() -> Self{
		CPU{
			PC : 0,
			SP : 50,

			A : 0,
			X : 0,
			Y : 0,

			CF : false, // Carry Flag
			ZF : false, // Zero Flag
			OF : false, // Overflow Flag
			NF : false, // Negative Flag
		}
	}

	pub fn exec(&mut self, Mem: &mut [usize]){
        loop{
          match Mem[self.PC]{
            0 => break,
            1 => {
                //LDA
                self.PC += 1;
                self.A = Mem[self.PC];
                Mem[0] = self.A;
            }

            2 => {
                //ADD
                self.PC += 1;
                self.A = self.A + Mem[self.PC];
                Mem[0] = self.A;
            }

            3 => {
                //JNE
                self.PC += 1;
                if self.ZF == false{
                    self.PC = Mem[self.PC] + 99;
                }
            }
            4 => {
                //CMP
                self.A = Mem[0];
                self.PC += 1;
                if self.A > Mem[self.PC]{
                    self.CF = true;
                    self.ZF = false;
                    self.NF = false;
                }
                else if self.A == Mem[self.PC]{
                    self.CF = false;
                    self.ZF = true;
                    self.NF = false;
                }
                else{
                    self.NF = true;
                    self.CF = false;
                    self.ZF = false;
                }
            }
            5 => {
                //OR
            }
            6 => {
                //AND
            }
            7 => {
                //PUSH
            }
            8 => {
                //POP
            }
            9 => {
                //OUTI
                self.PC += 1;
                let address = Mem[self.PC];
                println!("{}", Mem[address])
            }
            10 => {
                //OUTI
                self.PC += 1;
                let address = Mem[self.PC];
                println!("{}", Mem[address])
            }
            0xB => {
                //LD
                self.PC += 1;
                let address = Mem[self.PC];
                self.PC += 1;
                let val = Mem[self.PC];

                Mem[address] = val;
            }

            0xC => {
                //JLT
                self.PC += 1;
                if self.NF == true{
                    self.PC = Mem[self.PC] + 99;
                }
            }

            0xD => {
                //JGT
                self.PC += 1;
                if self.CF == true{
                    self.PC = Mem[self.PC] + 99;
                }
            }

            0xE => {
                //CMX
                self.X = Mem[1];
                self.PC += 1;
                if self.X > Mem[self.PC]{
                    self.CF = true;
                    self.ZF = false;
                    self.NF = false;
                }
                else if self.X == Mem[self.PC]{
                    self.CF = false;
                    self.ZF = true;
                    self.NF = false;
                }
                else{
                    self.NF = true;
                    self.CF = false;
                    self.ZF = false;
                }
            }

            0xF => {
                //CMY
                self.Y = Mem[2];
                self.PC += 1;
                if self.Y > Mem[self.PC]{
                    self.CF = true;
                    self.ZF = false;
                    self.NF = false;
                }
                else if self.Y == Mem[self.PC]{
                    self.CF = false;
                    self.ZF = true;
                    self.NF = false;
                }
                else{
                    self.NF = true;
                    self.CF = false;
                    self.ZF = false;
                }
            }

            0x11 => {
                //LDX
                self.PC += 1;
                self.X = Mem[self.PC];
                Mem[1] = self.X;
            }

            0x12 => {
                //LDY
                self.PC += 1;
                self.Y = Mem[self.PC];
                Mem[2] = self.Y;
            }

            0x13 => {
                //ADR
                self.PC += 1;
                let addr = Mem[self.PC];
                let val = Mem[addr];
                self.A = self.A + val;
                Mem[0] = self.A;
            }

            0x14 => {
                //LDR
                self.PC += 1;
                let address1 = Mem[self.PC];
                self.PC += 1;
                let address2 = Mem[self.PC];
                Mem[address1] = Mem[address2];
            }

            _ => {
                println!("Unknown Operation");
                break;
            }

          }
          self.PC += 1;
        }

	}
}

fn load(Mem : &mut [usize], filename : &str) {
    //Reads the assmebly from the given file and puts it in the memory
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut i = 100;
    for x in contents.split_whitespace(){
        match x {
            "NOP"  => Mem[i] =  0x0,   // DONE
            "LDA"  => Mem[i] =  0x1,   // DONE
            "ADD"  => Mem[i] =  0x2,   // DONE
            "JNE"  => Mem[i] =  0x3,   // DONE
            "CMP"  => Mem[i] =  0x4,   // DONE
            "OR"   => Mem[i] =  0x5,
            "AND"  => Mem[i] =  0x6,
            "PUSH" => Mem[i] =  0x7,
            "POP"  => Mem[i] =  0x8,
            "OUTI" => Mem[i] =  0x9,   // DONE
            "OUTC" => Mem[i] =  0xA,
            "LD"   => Mem[i] =  0xB,   // DONE
            "JLT"  => Mem[i] =  0xC,   // DONE
            "JGT"  => Mem[i] =  0xD,   // DONE
            "CMX"  => Mem[i] =  0xE,   // DONE
            "CMY"  => Mem[i] =  0xF,   // DONE
            "LDX"  => Mem[i] = 0x11,   // DONE
            "LDY"  => Mem[i] = 0x12,   // DONE
            "ADR"  => Mem[i] = 0x13,   // DONE
            "LDR"  => Mem[i] = 0x14,   // DONE
            _      => Mem[i] = x.parse().unwrap()
        }
        i+=1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
	let mut Mem: [usize; MAX_MEM] = [0; MAX_MEM]; // This is the memory for the CPU
	let mut cpu = CPU::new();	                  // The main CPU instance
	cpu.Reset();

    load(&mut Mem, filename);
    cpu.exec(&mut Mem);
}
