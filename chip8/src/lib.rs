pub mod chip8 {
    use rand::random;

    const RAM_SIZE: usize = 4096;
    pub const SCREEN_WIDTH: usize = 64;
    pub const SCREEN_HEIGHT: usize = 32;
    const DISPLAY_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;
    const REG_COUNT: usize = 16;
    const STACK_SIZE: usize = 16;
    const START_ADDRESS: u16 = 0x200;
    const NUM_KEYS: usize = 16;
    const FONT_SIZE: usize = 80;
    const FONT: [u8; FONT_SIZE] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80, // F
    ];
    pub struct Chip8 {
        ram: [u8; RAM_SIZE],
        display: [bool; DISPLAY_SIZE],
        pc: u16, //program counter
        i: u16,
        stack: [u16; STACK_SIZE],
        sp: u16, //stack pointer
        delay_timer: u8,
        sound_timer: u8,
        v_reg: [u8; REG_COUNT],
        keys: [bool; NUM_KEYS],
    }

    impl Chip8 {
        pub fn new() -> Chip8 {
            let mut new_chip8 = Self {
                ram: [0; RAM_SIZE],
                display: [false; DISPLAY_SIZE],
                pc: START_ADDRESS,
                i: 0,
                stack: [0; STACK_SIZE],
                sp: 0,
                delay_timer: 0,
                sound_timer: 0,
                v_reg: [0; REG_COUNT],
                keys: [false; NUM_KEYS],
            };

            new_chip8.ram[..FONT_SIZE].copy_from_slice(&FONT);
            new_chip8
        }

        pub fn reset(&mut self) {
            self.ram = [0; RAM_SIZE];
            self.display = [false; DISPLAY_SIZE];
            self.pc = START_ADDRESS;
            self.i = 0;
            self.stack = [0; STACK_SIZE];
            self.sp = 0;
            self.delay_timer = 0;
            self.sound_timer = 0;
            self.v_reg = [0; REG_COUNT];
            self.keys = [false; NUM_KEYS];
            self.ram[..FONT_SIZE].copy_from_slice(&FONT);
        }

        fn push(&mut self, val: u16) {
            self.stack[self.sp as usize] = val;
            self.sp += 1;
        }

        fn pop(&mut self) -> u16 {
            self.sp -= 1;
            self.stack[self.sp as usize]
        }

        pub fn tick(&mut self) {
            //fetch
            let opcode = self.fetch();
            //decode and execute
            self.decode_and_execute(opcode);
        }

        fn fetch(&mut self) -> u16 {
            let higher_byte = self.ram[self.pc as usize] as u16;
            let lower_byte = self.ram[(self.pc + 1) as usize] as u16;
            let opcode = (higher_byte << 8) | lower_byte;
            self.pc += 2;
            opcode
        }

        fn decode_and_execute(&mut self, opcode: u16) {
            let nibble1 = (opcode & 0xF000) >> 12;
            let nibble2 = (opcode & 0x0F00) >> 8;
            let nibble3 = (opcode & 0x00F0) >> 4;
            let nibble4 = opcode & 0x000F;

            match (nibble1, nibble2, nibble3, nibble4) {
                (0, 0, 0, 0) => return, // No operation
                (0, 0, 0xE, 0) => {
                    // Clear the screen
                    self.display = [false; DISPLAY_SIZE];
                }
                (0, 0, 0xE, 0xE) => {
                    // Return from subroutine
                    self.pc = self.pop(); //the return address should have been saved at the start of the subroutine.
                }
                (1, _, _, _) => {
                    // JUMP
                    self.pc = opcode & 0xFFF;
                }
                (2, _, _, _) => {
                    // Call subroutine
                    self.push(self.pc);
                    self.pc = opcode & 0xFFF;
                }
                (3, _, _, _) => {
                    // Skip next if VX is equal to NN
                    let x = nibble2 as usize;
                    let nn = (opcode & 0xFF) as u8;
                    if self.v_reg[x] == nn {
                        self.pc += 2;
                    }
                }
                (4, _, _, _) => {
                    // Skip next if VX is NOT equal to NN
                    let x = nibble2 as usize;
                    let nn = (opcode & 0xFF) as u8;
                    if self.v_reg[x] != nn {
                        self.pc += 2;
                    }
                }
                (5, _, _, 0) => {
                    // Skip next if VX is equal to VY
                    let x = nibble2 as usize;
                    let y = nibble3 as usize;
                    if self.v_reg[x] == self.v_reg[y] {
                        self.pc += 2
                    }
                }
                (6, _, _, _) => {
                    // set VX to NN
                    let x = nibble2 as usize;
                    let nn = (opcode & 0xFF) as u8;
                    self.v_reg[x] = nn;
                }
                (7, _, _, _) => {
                    // set VX += NN
                    let x = nibble2 as usize;
                    let nn = (opcode & 0xFF) as u8;
                    self.v_reg[x] = self.v_reg[x].wrapping_add(nn);
                }
                (8, _, _, 0) => {
                    // set VX = VY
                    let x = nibble2 as usize;
                    let y = nibble3 as usize;
                    self.v_reg[x] = self.v_reg[y];
                }
                (8, _, _, 1) => {
                    // set VX to the bitwise OR of VX, VY.
                    let x = nibble2 as usize;
                    let y = nibble3 as usize;
                    self.v_reg[x] |= self.v_reg[y];
                }
                (8, _, _, 2) => {
                    // set VX to the bitwise AND of VX, VY.
                    let x = nibble2 as usize;
                    let y = nibble3 as usize;
                    self.v_reg[x] &= self.v_reg[y];
                }
                (8, _, _, 3) => {
                    // set VX to the bitwise NOT of VX, VY.
                    let x = nibble2 as usize;
                    let y = nibble3 as usize;
                    self.v_reg[x] ^= self.v_reg[y];
                }
                (8, _, _, 4) => {
                    // set VX += VY.
                    let x = nibble2 as usize;
                    let y = nibble3 as usize;

                    let (new_vx, carry) = self.v_reg[x].overflowing_add(self.v_reg[y]);
                    let new_vf = if carry { 1 } else { 0 };

                    self.v_reg[x] = new_vx;
                    self.v_reg[0xF] = new_vf;
                }
                (8, _, _, 5) => {
                    // set VX -= VY.
                    let x = nibble2 as usize;
                    let y = nibble3 as usize;

                    let (new_vx, carry) = self.v_reg[x].overflowing_sub(self.v_reg[y]);
                    let new_vf = if carry { 0 } else { 1 };

                    self.v_reg[x] = new_vx;
                    self.v_reg[0xF] = new_vf;
                }
                (8, _, _, 6) => {
                    // set VX >>= 1.
                    let x = nibble2 as usize;
                    let left_side_bit = self.v_reg[x] & 1;

                    self.v_reg[x] >>= 1;
                    self.v_reg[0xF] = left_side_bit
                }
                (8, _, _, 7) => {
                    // set VY -= VX.
                    let x = nibble2 as usize;
                    let y = nibble3 as usize;

                    let (new_vy, carry) = self.v_reg[y].overflowing_sub(self.v_reg[x]);
                    let new_vf = if carry { 0 } else { 1 };

                    self.v_reg[y] = new_vy;
                    self.v_reg[0xF] = new_vf;
                }
                (8, _, _, 0xE) => {
                    // set VX <<= 1.
                    let x = nibble2 as usize;
                    let right_side_bit = (self.v_reg[x] >> 7) & 1;

                    self.v_reg[x] <<= 1;
                    self.v_reg[0xF] = right_side_bit
                }
                (9, _, _, 0) => {
                    // Skip next if VX is NOT equal to VY
                    let x = nibble2 as usize;
                    let y = nibble3 as usize;
                    if self.v_reg[x] != self.v_reg[y] {
                        self.pc += 2
                    }
                }
                (0xA, _, _, _) => {
                    // Set the immediate register to NNN
                    self.i = opcode & 0xFFF;
                }
                (0xB, _, _, _) => {
                    // Jump to V0 + NNN
                    self.pc = self.v_reg[0] as u16 + (opcode & 0xFFF)
                }
                (0xC, _, _, _) => {
                    // VX = random & NN
                    // let x = nibble2 as usize;
                    // self.v_reg[x] = random::<u8>() * (opcode &

                    let x = nibble2 as usize;
                    let nn = (opcode & 0xFF) as u8;
                    let rng: u8 = random();
                    self.v_reg[x] = rng & nn;
                }
                // (0xD, _, _, _) => {
                //     // Draw Sprite
                //     let x_coord = self.v_reg[nibble2 as usize] % 64;
                //     let y_coord = self.v_reg[nibble3 as usize] % 32;
                //     self.v_reg[0xF] = 0;
                //
                //     for byte in 0..nibble4 as u8 {
                //         let address = self.i + byte;
                //         for mut flipped_flag = false;
                //
                //         for bit in 0..8 {
                //             if (self.ram[address])
                //         }
                //     }
                // }
                (0xD, _, _, _) => {
                    // Draw Sprite
                    let x_coord = self.v_reg[nibble2 as usize] as u16;
                    let y_coord = self.v_reg[nibble3 as usize] as u16;

                    let num_rows = nibble4;
                    let mut flipped_flag = false;

                    for y_line in 0..num_rows {
                        let addr = self.i + y_line as u16;
                        let pixels = self.ram[addr as usize];

                        for x_line in 0..8 {
                            if (pixels & (0b1000_0000 >> x_line)) != 0 {
                                let x = (x_coord + x_line) as usize % SCREEN_WIDTH;
                                let y = (y_coord + y_line) as usize % SCREEN_HEIGHT;

                                let idx = x + SCREEN_WIDTH * y;
                                flipped_flag |= self.display[idx];
                                self.display[idx] ^= true;
                            }
                        }
                    }

                    if flipped_flag {
                        self.v_reg[0xF] = 1;
                    } else {
                        self.v_reg[0xF] = 0;
                    }
                }
                (0xE, _, 9, 0xE) => {
                    // Skip if Key Pressed
                    let x = nibble2 as usize;
                    let vx = self.v_reg[x] as usize;
                    let key = self.keys[vx];
                    if key {
                        self.pc += 2;
                    }
                }
                (0xE, _, 0xA, 1) => {
                    // Skip if Key Not Pressed
                    let x = nibble2 as usize;
                    let vx = self.v_reg[x] as usize;
                    let key = self.keys[vx];
                    if !key {
                        self.pc += 2;
                    }
                }
                (0xF, _, 0, 7) => {
                    // VX = delay_timer
                    self.v_reg[nibble2 as usize] = self.delay_timer
                }
                (0xF, _, 0, 0xA) => {
                    // Wait for key press
                    let x = nibble2 as usize;
                    let mut pressed = false;
                    for i in 0..self.keys.len() {
                        if self.keys[i] {
                            self.v_reg[x] = i as u8;
                            pressed = true;
                            break;
                        }
                    }
                    if !pressed {
                        self.pc -= 2
                    }
                }
                (0xF, _, 1, 5) => {
                    // Delay Timer = VX
                    self.delay_timer = self.v_reg[nibble2 as usize];
                }
                (0xF, _, 1, 8) => {
                    // Sound Timer = VX
                    self.sound_timer = self.v_reg[nibble2 as usize];
                }
                (0xF, _, 1, 0xE) => {
                    // Immediate register += VX
                    self.i = self.i.wrapping_add(self.v_reg[nibble2 as usize] as u16);
                }
                (0xF, _, 2, 9) => {
                    // Set I to Font
                    let font_address = self.v_reg[nibble2 as usize] as u16;
                    self.i = font_address * 5
                }
                (0xF, _, 3, 3) => {
                    // I = Binary-Coded Decimal of VX
                    //TODO - Optimize this with a better BCD algorithm.
                    let x = nibble2 as usize;
                    let vx = self.v_reg[x] as f32;

                    let hundreds = (vx / 100.0).floor() as u8;
                    let tens = ((vx / 10.0) % 10.0).floor() as u8;
                    let ones = (vx % 10.0).floor() as u8;
                    let i = self.i as usize;

                    self.ram[i] = hundreds;
                    self.ram[i + 1] = tens;
                    self.ram[i + 2] = ones;
                }
                (0xF, _, 5, 5) => {
                    // Store V0 through VX into the immediate register
                    let x = nibble2 as usize;
                    let i = self.i as usize;
                    for idx in 0..=x {
                        self.ram[i + idx] = self.v_reg[idx]
                    }
                }
                (0xF, _, 6, 5) => {
                    // Load from the immediate register into V0 though VX;
                    let x = nibble2 as usize;
                    let i = self.i as usize;
                    for idx in 0..=x {
                        self.v_reg[idx] = self.ram[i + idx];
                    }
                }
                (_, _, _, _) => {
                    unimplemented!(
                        "opcode: {}, Nibble1: {}, Nibble2: {}, Nibble3: {}, Nibble4: {}",
                        opcode,
                        nibble1,
                        nibble2,
                        nibble3,
                        nibble4
                    );
                }
            }
        }

        pub fn tick_timers(&mut self) {
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }
            if self.sound_timer > 0 {
                if self.sound_timer == 1 {
                    //TODO: Add sound functions
                }
                self.sound_timer -= 1;
            }
        }

        pub fn get_display(&self) -> &[bool] {
            &self.display
        }

        pub fn key_press(&mut self, idx: usize, pressed: bool) {
            self.keys[idx] = pressed;
        }

        pub fn load(&mut self, data: &[u8]) {
            let start = START_ADDRESS as usize;
            let end = (START_ADDRESS as usize) + data.len();

            self.ram[start..end].copy_from_slice(data);
        }
    }
}
