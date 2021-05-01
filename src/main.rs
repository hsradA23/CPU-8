#![allow(non_snake_case)]
#![allow(unused_variables)]
/// SHITTY 6502 CPU EMULATOR
/// http://www.obelisk.me.uk/6502/
/// http://www.obelisk.me.uk/6502/instructions.html ------- INSTRUCTIONS 


const MAX_MEM: usize = 1024 * 64;

pub struct CPU{
	pub PC: u16,  // Program Counter
	pub SP: u8,   // Stack Pointer

	//REGISTERS
	pub A: u8,    // Accumulator
	pub X: u8,    // Index register X
	pub Y: u8,    // Index register Y

	//PROCESSOR STATUS
	pub CF: bool, // Carry Flag
	pub ZF: bool, // Zero Flag
	pub ID: bool, // Interrupt disable
	pub DM: bool, // Decimal Mode
	pub BC: bool, // Break Command
	pub OF: bool, // Overflow Flag
	pub NF: bool, // Negative Flag
}


impl CPU{
	pub fn Reset(&mut self){
		self.PC = 0xFFFC;
		self.SP = 0x00FF;

		self.A = 0;
		self.X = 0;
		self.Y = 0;

		self.CF = false; // Carry Flag
		self.ZF = false; // Zero Flag
		self.ID = false; // Interrupt disable
		self.DM = false; // Decimal Mode
		self.BC = false; // Break Command
		self.OF = false; // Overflow Flag
		self.NF = false; // Negative Flag
	}

	pub fn new() -> Self{
		CPU{
			PC : 0xFFFC,
			SP : 0x00FF,

			A : 0,
			X : 0,
			Y : 0,

			CF : false, // Carry Flag
			ZF : false, // Zero Flag
			ID : false, // Interrupt disable
			DM : false, // Decimal Mode
			BC : false, // Break Command
			OF : false, // Overflow Flag
			NF : false, // Negative Flag
		}
	}

	pub fn exec(Cycles: usize, Mem: &mut [u8]){
		// OPCODES
		// Load/Store Operations
		const LDA = ;
		const LDX = ;
		const LDY = ;
		const STA = ;
		const STX = ;
		const STY = ;
		// Register Transfers
		const TAX = ;
		const TAY = ;
		const TXA = ;
		const TYA = ;
		// Stack Operations
		const TSX = ;
		const TXS = ;
		const PHA = ;
		const PHP = ;
		const PLA = ;
		const PLP = ;
		// Logical
		const AND = ;
		const EOR = ;
		const ORA = ;
		const BIT = ;
		// Arithmetic
		const ADC = ;
		const SBC = ;
		const CMP = ;
		const CPX = ;
		const CPY = ;
		// Increment and Decrement
		const INC = ;
		const INX = ;
		const INY = ;
		const DEC = ;
		const DEX = ;
		const DEY = ;
		// Shifts
		const ASL = ;
		const LSR = ;
		const ROL = ;
		const ROR = ;
		// Jumps and calls
		const JMP = ;
		const JSR = ;
		const RTS = ;
		// Branches
		const BCC = ;
		const BCS = ;
		const BEQ = ;
		const BMI = ;
		const BNE = ;
		const BPL = ;
		const BVC = ;
		const BVS = ;
		// Status Flag changes
		const CLC = ;
		const CLD = ;
		const CLI = ;
		const CLV = ;
		const SEC = ;
		const SED = ;
		const SEI = ;
		// System functions
		const BRK = ;
		const NOP = ;
		const RTI = ;

		for _ in Cycles{
			Fetch();
		}
	}
}


fn main() {

	let mut Data: [u8; MAX_MEM]; // This is the memory for the CPU
	let mut cpu = CPU::new();	 // The main CPU instance
	cpu.Reset();

    println!("Hello, world!");
}
