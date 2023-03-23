# CPU-8
A CPU emulator with a custom instruction set

### Instructions
The following instructions are supported by the interpreter.
0x0  NOP                            	=> No Operation -> Quits the program
0x1  LDA    value			=> Puts the value in the A resister (also Mem location 0)
0x2  ADD    value			=> Adds the value to the A register
0x3  JNE    address			=> Jumps to the address if the Zero Flag is not set
0x4  CMP    value			=> compares the A register to the given value and changes CF, ZF, NF
0x5  OR
0x6  AND
0x7  PUSH
0x8  POP
0x9  OUTI   addr			=> Prints the contents of the Address
0xA  OUTC
0xB  LD     addr value			=> Puts the given value in the specified address 
0xC  JLT    addr                        => Jumps to the given address if Negative Flag is set
0xD  JGT    addr			=> Jumps to the given address if Carry Flag is set
0xE  CMX    value			=> Compares X to the value and changes the flags
0xF  CMY    value			=> Compares Y to the value and changes the flags
0x10
0x11 LDX    value			=> Loads X with the specified value
0x12 LDY    value			=> Loads Y with the specified value
0x13 ADR    address			=> Adds the value stored in the address to A
0x14 LDR    destination source		=> Loads the value of the source in the address

The [fibonacci](fibonacci) contains code that prints fibonacci numbers under 50 as a demo.
