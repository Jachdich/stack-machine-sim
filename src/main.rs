extern crate num;
#[macro_use]
extern crate num_derive;

#[derive(FromPrimitive)]
enum Opcode {
    NOP,
    ADD,
    SUB,
    PUSH,
    OUT,
    HALT
}

struct CPU {
    stack: [u8; 256],
    mem: [u8; 256],
    sp: u8,
    pc: u8,
    ir: u8,
    bus: u8,
    z: u8,
    t: u8,
    wr_addr: u8,
    rd_addr: u8,
}

fn hex(n: u8) -> String {
    return format!("{:01$x}", n, 2);
}

impl CPU {
    fn clk_rising(&mut self) -> bool {
        let instr: std::option::Option<Opcode> = num::FromPrimitive::from_u32(self.ir as u32);
        match instr {
            Some(Opcode::NOP) => {
                self.pc += 1;
            },
            Some(Opcode::PUSH) => {
                self.pc += 1;
                self.sp += 1;
                self.bus = self.mem[self.pc as usize];
                self.wr_addr = self.sp;
            },

            Some(Opcode::ADD) => {
                if t == 0 {
                    self.rd_addr = self.sp;
                    self.bus = self.stack[self.rd_addr as usize];
                } else {
                    self.fucking_die
                }
            },
            Some(Opcode::HALT) => {
                return false;
            },
            Some(_)   => {},
            None => println!("Illegal opcode"),
        }
        return true;
    }

    
    fn clk_falling(&mut self) {
        let instr: std::option::Option<Opcode> = num::FromPrimitive::from_u32(self.ir as u32);
        let mut setup_next = false;
        match instr {
            Some(Opcode::NOP) => {
                setup_next = true;
            },
            Some(Opcode::PUSH) => {
                self.stack[self.wr_addr as usize] = self.bus;
                setup_next = true;
                self.pc += 1;
            },
            Some(Opcode::ADD) => {
                if self.t == 0 {
                    self.t = 1;
                    self.z = self.bus;
                    self.sp -= 1;
                    self.rd_addr = self.sp;
                    self.bus = self.stack[sp];
                } else {
                    setup_next = true;

                }
            },
            Some(_)   => {},
            None => println!("Illegal opcode"),
        }
        if setup_next {
            self.bus = self.mem[self.pc as usize];
            self.ir = self.bus;
        }
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
        ir: 0,
        bus: 0,
        z: 0,
        t: 0,
        wr_addr: 0,
        rd_addr: 0,
    };
    cpu.mem[0] = Opcode::PUSH as u8;
    cpu.mem[1] = 0x33;
    cpu.mem[2] = Opcode::PUSH as u8;
    cpu.mem[3] = 0x44;
    cpu.mem[4] = Opcode::ADD as u8;
    cpu.mem[5] = Opcode::OUT as u8;
    cpu.mem[6] = Opcode::HALT as u8;

    cpu.clk_falling();

    for _i in 0..20 {
        if !cpu.clk_rising() { break; }
        println!("Rising");
        cpu.print_state();
        cpu.clk_falling();
        println!("Falling");
        cpu.print_state();
        println!("\n================================\n");
    }
}
