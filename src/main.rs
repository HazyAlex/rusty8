#![allow(dead_code)]
#![allow(unused_variables)]

use std::{fs::File, io::Read};

use fltk::{app, prelude::*, window::Window};

const CHIP8_WIDTH: usize = 64;
const CHIP8_HEIGHT: usize = 32;

struct Emulator {
    registers: [u8; 16],

    stack: [u16; 16],
    stack_pointer: u8,

    address: u16,
    memory: [u8; 4096],
    program_counter: u16,

    screen: [[u8; CHIP8_HEIGHT]; CHIP8_WIDTH],

    keyboard: [u8; 16],
    delay_timer: u8,
}

impl Default for Emulator {
    fn default() -> Self {
        Self {
            registers: [0u8; 16],

            stack: [0u16; 16],
            stack_pointer: Default::default(),

            address: Default::default(),
            memory: [0u8; 4096],
            program_counter: 512,

            screen: [[0u8; CHIP8_HEIGHT]; CHIP8_WIDTH],

            keyboard: [0u8; 16],
            delay_timer: Default::default(),
        }
    }
}

impl Emulator {
    fn load_rom(&mut self, file: &mut File) {
        let data = &mut self.memory[0x0200..];

        file.read(data).expect("Invalid ROM");
    }

    fn next_opcode(&mut self) -> u16 {
        let mut current: u16 = *self
            .memory
            .get(self.program_counter as usize)
            .expect("Memory access out of bounds") as u16;

        let next: u16 = *self
            .memory
            .get((self.program_counter + 1) as usize)
            .expect("Memory access out of bounds") as u16;

        self.program_counter += 2;

        current = current << 8;
        current = current | next;

        current
    }

    fn run(&mut self) {
        let opcode = self.next_opcode();
        let first = opcode & 0xF000;
        let _second = opcode & 0x0F00;
        let third = opcode & 0x00F0;
        let fourth = opcode & 0x000F;

        match first {
            0x0000 => match opcode & fourth {
                0x0000 => self.clear_screen(),
                0x000E => self.fn_return(),
                _ => unreachable!(),
            },
            0x1000 => self.jump(opcode),
            0x2000 => self.fn_call(opcode),
            0x3000 => self.skip_if_variable_is_equal_to(opcode),
            0x4000 => self.skip_if_variable_is_not_equal_to(opcode),
            0x5000 => self.skip_if_variables_equal(opcode),
            0x6000 => self.set_register_to(opcode),
            0x7000 => self.add_to_variable(opcode),
            0x8000 => match opcode & fourth {
                0x0000 => self.op_assignment(opcode),
                0x0001 => self.op_or(opcode),
                0x0002 => self.op_and(opcode),
                0x0003 => self.op_xor(opcode),
                0x0004 => self.op_add(opcode),
                0x0005 => self.op_sub(opcode),
                0x0006 => self.op_shift_right(opcode),
                0x0007 => self.op_sub_assign(opcode),
                0x000E => self.op_shift_left(opcode),
                _ => unreachable!(),
            },
            0x9000 => self.skip_if_variables_not_equal(opcode),
            0xA000 => self.set_address_to(opcode),
            0xB000 => self.jump_add(opcode),
            0xC000 => self.bitwise_and_with_random(opcode),
            0xD000 => self.draw_sprite(opcode),
            0xE000 => match opcode & fourth {
                0x0001 => self.skip_if_not_pressed(opcode),
                0x000E => self.skip_if_pressed(opcode),
                _ => unreachable!(),
            },
            0xF000 => match opcode & third {
                0x0000 => match opcode & fourth {
                    0x0007 => self.set_variable_to_delay_timer(opcode),
                    0x000A => self.get_key_press(opcode),
                    _ => unreachable!(),
                },
                0x0010 => match opcode & fourth {
                    0x0005 => self.set_delay_timer_to(opcode),
                    0x0008 => self.set_sound_timer_to(opcode),
                    0x000E => self.add_variable_to_address(opcode),
                    _ => unreachable!(),
                },
                0x0020 => match opcode & fourth {
                    0x0009 => self.sets_address_to_sprite(opcode),
                    _ => unreachable!(),
                },
                0x0030 => match opcode & fourth {
                    0x0003 => self.store_variable_as_binary(opcode),
                    _ => unreachable!(),
                },
                0x0050 => match opcode & fourth {
                    0x0005 => self.dump_registers_into_memory(opcode),
                    _ => unreachable!(),
                },
                0x0060 => match opcode & fourth {
                    0x0005 => self.load_registers_from_memory(opcode),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },

            _ => unreachable!(),
        }
    }

    /// Clears the screen.
    fn clear_screen(&mut self) {
        self.screen = [[0u8; CHIP8_HEIGHT]; CHIP8_WIDTH];
    }

    /// Jumps to address NNN.
    fn jump(&mut self, opcode: u16) {
        self.program_counter = opcode & 0x0FFF;
    }

    /// Calls subroutine at NNN.
    fn fn_call(&mut self, opcode: u16) {
        self.stack_pointer += 1;
        self.stack[self.stack_pointer as usize] = self.program_counter;

        self.program_counter = opcode & 0x0FFF;
    }

    /// Returns from a subroutine.
    fn fn_return(&mut self) {
        self.program_counter = self.stack[self.stack_pointer as usize];

        self.stack_pointer -= 1;
    }

    /// Skips the next instruction if VX equals NN (usually the next instruction is a jump to skip a code block).
    fn skip_if_variable_is_equal_to(&mut self, opcode: u16) {
        let vx = (opcode & 0x0F00) >> 8;
        let value = opcode & 0x00FF;

        if self.registers[vx as usize] == value as u8 {
            self.program_counter += 2;
        }
    }

    fn skip_if_variable_is_not_equal_to(&mut self, opcode: u16) {
        todo!()
    }

    fn skip_if_variables_equal(&mut self, opcode: u16) {
        todo!()
    }

    fn skip_if_variables_not_equal(&mut self, opcode: u16) {
        todo!()
    }

    fn set_register_to(&mut self, opcode: u16) {
        let vx = (opcode & 0x0F00) >> 8;
        let value = opcode & 0x00FF;

        self.registers[vx as usize] = value as u8;
    }

    /// Adds NN to VX (carry flag is not changed)
    fn add_to_variable(&mut self, opcode: u16) {
        let vx = (opcode & 0x0F00) >> 8;
        let value = opcode & 0x00FF;

        self.registers[vx as usize] += value as u8;
    }

    fn op_assignment(&mut self, opcode: u16) {
        let vx = (opcode & 0x0F00) >> 8;
        let vy = (opcode & 0x00F0) >> 4;

        self.registers[vx as usize] = self.registers[vy as usize];
    }

    fn op_or(&mut self, opcode: u16) {
        todo!()
    }

    fn op_and(&mut self, opcode: u16) {
        todo!()
    }

    fn op_xor(&mut self, opcode: u16) {
        todo!()
    }

    fn op_add(&mut self, opcode: u16) {
        todo!()
    }

    fn op_sub(&mut self, opcode: u16) {
        todo!()
    }

    fn op_shift_right(&mut self, opcode: u16) {
        todo!()
    }

    fn op_sub_assign(&mut self, opcode: u16) {
        todo!()
    }

    fn op_shift_left(&mut self, opcode: u16) {
        todo!()
    }

    /// Sets I to the address NNN.
    fn set_address_to(&mut self, opcode: u16) {
        self.address = opcode & 0x0FFF;
    }

    fn jump_add(&mut self, opcode: u16) {
        todo!()
    }

    fn bitwise_and_with_random(&mut self, opcode: u16) {
        todo!()
    }

    fn get_key_press(&mut self, opcode: u16) {
        todo!()
    }

    /// Skips the next instruction if the key stored in VX is not pressed (usually the next instruction is a jump to skip a code block).
    fn skip_if_not_pressed(&mut self, opcode: u16) {
        let vx = (opcode & 0x0F00) >> 8;
        let key = self.registers[vx as usize];

        if self.keyboard[key as usize] == 0 {
            self.program_counter += 2;
        }
    }

    fn skip_if_pressed(&mut self, opcode: u16) {
        todo!()
    }

    /// Sets VX to the value of the delay timer.
    fn set_variable_to_delay_timer(&mut self, opcode: u16) {
        let vx = (opcode & 0x0F00) >> 8;

        self.registers[vx as usize] = self.delay_timer;
    }

    /// Adds VX to I. VF is not affected.
    fn add_variable_to_address(&mut self, opcode: u16) {
        let vx = (opcode & 0x0F00) >> 8;
        let value = self.registers[vx as usize];

        self.address += value as u16;
    }

    fn draw_sprite(&mut self, opcode: u16) {
        todo!()
    }

    fn sets_address_to_sprite(&mut self, opcode: u16) {
        todo!()
    }

    fn store_variable_as_binary(&mut self, opcode: u16) {
        todo!()
    }

    fn dump_registers_into_memory(&mut self, opcode: u16) {
        todo!()
    }

    /// Fills from V0 to VX (including VX) with values from memory, starting at address I.
    /// The offset from I is increased by 1 for each value read, but I itself is left unmodified.
    fn load_registers_from_memory(&mut self, opcode: u16) {
        let vx = (opcode & 0x0F00) >> 8;

        for register in 0..(vx + 1) {
            let result = self.memory[(self.address + register) as usize];

            self.registers[register as usize] = result;
        }
    }

    fn set_delay_timer_to(&mut self, opcode: u16) {
        let vx = (opcode & 0x0F00) >> 8;

        self.delay_timer = self.registers[vx as usize];
    }

    fn set_sound_timer_to(&mut self, opcode: u16) {
        todo!()
    }
}

fn main() {
    let mut file = File::open("games/Invaders.ch8").expect("ROM not found!");
    let mut emulator = Emulator::default();

    emulator.load_rom(&mut file);

    //
    // UI
    //
    const RATIO: usize = 4;
    const WIDTH: i32 = (CHIP8_WIDTH * RATIO) as i32;
    const HEIGHT: i32 = (CHIP8_HEIGHT * RATIO) as i32;

    let mut window = Window::default()
        .with_size(WIDTH, HEIGHT)
        .with_label("Rusty8");

    let mut frame = fltk::frame::Frame::default().size_of(&window);

    window.end();
    window.make_resizable(false);
    window.show();

    let mut frame_buffer = vec![0; (WIDTH * HEIGHT * 4) as usize];

    app::add_idle3(move |_| {
        emulator.run();

        if emulator.delay_timer > 0 {
            emulator.delay_timer -= 1;
        }

        for (i, pixel) in frame_buffer.chunks_exact_mut(4).enumerate() {
            let x = i % WIDTH as usize;
            let y = i / WIDTH as usize;

            let color = emulator.screen[x / RATIO][y / RATIO];

            let rgba = if color == 0 {
                [0, 0, 0, 255]
            } else {
                [255, 255, 255, 255]
            };

            pixel.copy_from_slice(&rgba)
        }

        fltk::draw::draw_rgba(&mut frame, &frame_buffer).unwrap();

        window.redraw();

        app::sleep(0.016);
    });

    app::App::default().run().unwrap();
}
