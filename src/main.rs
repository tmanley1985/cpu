struct CPU {
    program_counter: usize, // this could be called position_in_memory
    registers: [u8; 16],
    memory: [u8; 0x1000], //We're simulating 4,096 bytes of RAM
    stack: [u16; 16],
    stack_pointer: usize
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let position_in_memory = self.program_counter;

        // We have u8 in memory, but we want a u16 opcode so we can just cast these
        // as u16 and it will left pad with 8 zeros.
        let opbyte_1 = self.memory[position_in_memory] as u16; 
        let opbyte_2 = self.memory[position_in_memory + 1] as u16;

        // We need to shift over to make room for the second byte and
        // then we can concatenate these with the OR operator.
        return (opbyte_1 << 8) | opbyte_2;

    }

    // This is simulating the Fetch Decode Execute cycle of a CPU of the CHIP-8 architecture.
    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            // Remember that we are using u8 for our memory but we are using 16 bit opcodes, so we
            // need to move places at a time.
            self.program_counter += 2;

            // This is applying a mask to the opcode
            // to isolate the number it represents.
            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >>  8) as u8;
            let y = ((opcode & 0x00F0) >>  4) as u8;
            let d = ((opcode & 0x000F) >>  0) as u8;

            let nnn = opcode & 0x0FFF;

            match (c, x, y, d) {
                // There is no opcode that is all zeros so this is the fixed point of our program!
                (0,0,0,0) => { return; },
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _  =>  todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn call(&mut self, address: u16) {
        let p = self.stack_pointer;
        let stack = &mut self.stack;

        if p > stack.len() {
            panic!("Stack Overflow!");
        }

        stack[p] = self.program_counter as u16;
        self.stack_pointer += 1;
        self.program_counter = address as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack Underflow!");
        }

        self.stack_pointer -= 1;
        let address = self.stack[self.stack_pointer];
        self.program_counter = address as usize;
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);

        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }

    }
}


// We're going to support addition.
// For this we need two registers (x, y).
// We'll take the value of y and add it to x.

// Initialize a CPU.
// Load u8 values into registers.
// Load the addition opcode into current_operation.
// Perform the operation.

fn main() {
    let mut cpu = CPU {
        program_counter: 0,
        registers: [0; 16],
        memory: [0; 4096],
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    let mem = &mut cpu.memory;

    // Remember, we only have 8 bits at each point in memory
    // but since we are using it like we have 16 bits, we need to
    // break up two bytes and put one at a certain index and the next
    // at that index plus 1.

    // Sets opcode to 0x2100: CALL the function at 0x100.
    mem[0x000] = 0x21; mem[0x001] = 0x00;
    // Sets opcode to 0x2100: CALL the function at 0x100.
    mem[0x002] = 0x21; mem[0x003] = 0x00;
    // Sets opcode to 0x0000. HALT. This isn't needed really
    // because when we create the memory we initialize it with zeros.
    mem[0x004] = 0x00; mem[0x005] = 0x00;

    // Sets the opcode to 0x8014. ADD the result of register 1 into register 0.
    mem[0x100] = 0x80; mem[0x101] = 0x14;
    // // Sets the opcode to 0x8014. ADD the result of register 1 into register 0.
    mem[0x102] = 0x80; mem[0x103] = 0x14;

    // Sets the opcode to 0xEE. RETURN
    mem[0x104] = 0x00; mem[0x105] = 0xEE;

    cpu.run();

    assert_eq!(cpu.registers[0], 45);

    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);

}
