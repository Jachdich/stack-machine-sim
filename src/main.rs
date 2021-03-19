extern crate num;
#[macro_use]
extern crate num_derive;

#[derive(FromPrimitive)]
enum Opcode {
    NOP,
    ADD,
    SUB,
    BE,
    BNE,
    PUSH,
    OUT,
    HALT
}

struct CPU {
    stack: [u8; 256],
    mem: [u8; 256],
    sp: u8,
    pc: u8,
    
}

fn hex(n: u8) -> String {
    return format!("{:01$x}", n, 2);
}

impl CPU {
    fn clk_rising(&mut self) {
        let instr: std::option::Option<Opcode> = num::FromPrimitive::from_u32(self.mem[self.pc as usize] as u32);
        match instr {
            Some(Opcode::NOP) => {},
            Some(Opcode::PUSH) => {
                
            },
            Some(_)   => {},
            None => println!("Illegal opcode"),
        }
    }
    
    fn clk_falling(&mut self) {
        let instr: std::option::Option<Opcode> = num::FromPrimitive::from_u32(self.mem[self.pc as usize] as u32);
        match instr {
            Some(Opcode::NOP) => {},
            Some(Opcode::PUSH) => {
                
            },
            Some(_)   => {},
            None => println!("Illegal opcode"),
        }
        self.pc += 1;
    }

    fn print_state(&mut self) {
        for i in 0..32 {
            print!("{:01$x} ", self.mem[i], 2);
        }
        println!("");
        let pos: usize = (self.pc * 3) as usize;
        println!("{}^ {}", " ".repeat(pos), hex(self.pc));

        for i in 0..32 {
            print!("{:01$x} ", self.stack[i], 2);
        }
        println!("");
        let pos: usize = (self.sp * 3) as usize;
        println!("{}^ {}", " ".repeat(pos), hex(self.sp));
    }
}

fn main() {
    let mut cpu: CPU = CPU{
        stack: [0; 256],
        mem: [0; 256],
        sp: 0,
        pc: 0,
    };
    cpu.mem[0] = Opcode::PUSH as u8;
    cpu.mem[1] = 50;
    cpu.mem[2] = Opcode::PUSH as u8;
    cpu.mem[3] = 60;
    cpu.mem[4] = Opcode::ADD as u8;
    cpu.mem[5] = Opcode::OUT as u8;
    cpu.mem[6] = Opcode::HALT as u8;

    for _i in 0..10 {
        cpu.clk_rising();
        println!("Rising");
        cpu.print_state();
        cpu.clk_falling();
        println!("Falling");
        cpu.print_state();
        println!("\n================================\n");
    }
}
