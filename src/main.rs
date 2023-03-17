#![allow(dead_code)]
#![allow(unused_variables)]

use std::{fs::File, io::Read};

use fltk::{app, prelude::*, window::Window};

const CHIP8_WIDTH: usize = 64;
const CHIP8_HEIGHT: usize = 32;

struct Emulator {
    registers: [u8; 16],
    stack: Vec<u16>,
    program_counter: u16,
    address: u16,
    memory: [u8; 4096],

    screen: [[u8; CHIP8_HEIGHT]; CHIP8_WIDTH],
}

impl Default for Emulator {
    fn default() -> Self {
        Self {
            registers: [0u8; 16],
            stack: Default::default(),
            program_counter: 512,
            address: Default::default(),
            memory: [0u8; 4096],
            screen: [[0u8; CHIP8_HEIGHT]; CHIP8_WIDTH],
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

        println!("Opcode: 0x{:X}", current);

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

            _ => unimplemented!(),
        }
    }

    //
    // Functions
    //
    fn clear_screen(&mut self) {
        todo!()
    }

    fn jump(&mut self, opcode: u16) {
        self.program_counter = opcode & 0x0FFF;

        println!("JUMP: {}", self.program_counter)
    }

    fn fn_call(&mut self, opcode: u16) {
        todo!()
    }

    fn fn_return(&mut self) {
        todo!()
    }

    fn skip_if_variable_is_equal_to(&mut self, _opcode: u16) {
        todo!()
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
        let register = (opcode & 0x0F00) >> 8;
        let value = opcode & 0x00FF;

        println!("SET REGISTER {}: {}", register, value);

        if register > 16 {
            println!("REGISTER {} OUT OF BOUNDS: setting {}", register, value);
            return;
        }

        self.registers[register as usize] = value as u8;
    }

    fn add_to_variable(&mut self, opcode: u16) {
        todo!()
    }

    fn op_assignment(&mut self, opcode: u16) {
        todo!()
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

    fn set_address_to(&mut self, opcode: u16) {
        todo!()
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

    fn skip_if_not_pressed(&mut self, opcode: u16) {
        todo!()
    }

    fn skip_if_pressed(&mut self, opcode: u16) {
        todo!()
    }

    fn set_variable_to_delay_timer(&mut self, opcode: u16) {
        todo!()
    }

    fn add_variable_to_address(&mut self, opcode: u16) {
        todo!()
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

    fn load_registers_from_memory(&mut self, opcode: u16) {
        todo!()
    }

    fn set_delay_timer_to(&mut self, opcode: u16) {
        todo!()
    }

    fn set_sound_timer_to(&mut self, opcode: u16) {
        todo!()
    }
}

fn main() {
    let mut file = File::open("roms/invaders.ch8").expect("ROM not found!");
    let mut emulator = Emulator::default();

    emulator.load_rom(&mut file);

    //
    // UI
    //
    const WIDTH: i32 = CHIP8_WIDTH as i32 * 4;
    const HEIGHT: i32 = CHIP8_HEIGHT as i32 * 4;

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

        for (i, pixel) in frame_buffer.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;

            let paint_black = if x < y { true } else { false };

            let rgba = if paint_black {
                [0, 0, 0, 255]
            } else {
                [255, 255, 255, 255]
            };

            pixel.copy_from_slice(&rgba);
        }

        fltk::draw::draw_rgba(&mut frame, &frame_buffer).unwrap();

        window.redraw();
        app::sleep(0.016);
    });

    app::App::default().run().unwrap();
}
