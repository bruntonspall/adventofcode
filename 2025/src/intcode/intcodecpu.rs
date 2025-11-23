
pub struct IntCodeCPU {
    pub memory: Vec<u32>,
    pub pc: usize,
    pub running: bool,
}
impl IntCodeCPU {

    pub fn new(program: Vec<u32>) -> Self {
        IntCodeCPU {
            memory: program,
            pc: 0,
            running: true,
        }
    }
    pub fn execute(&mut self) {
        let opcode = self.memory[self.pc];
        match opcode {
            1 => {
                let src1 = self.memory[self.memory[self.pc + 1] as usize] as u32;
                let src2 = self.memory[self.memory[self.pc + 2] as usize] as u32;
                let dest = self.memory[self.pc + 3] as usize;
                self.memory[dest] = src1 + src2;
                self.pc += 4;
            }
            2 => {
                let src1 = self.memory[self.memory[self.pc + 1] as usize] as u32;
                let src2 = self.memory[self.memory[self.pc + 2] as usize] as u32;
                let dest = self.memory[self.pc + 3] as usize;
                self.memory[dest] = src1 * src2;
                self.pc += 4;
            }
            99 => {
                self.running = false;
            }
            _ => {
                panic!("Unknown opcode: {}", opcode);
            }
        }
    }
    pub fn run(&mut self) {
        while self.running {
            self.execute();
        }
    }
}    



#[cfg(test)]
mod tests {
    use crate::intcode::intcodecpu::IntCodeCPU;

        #[test]
    fn test_intcode_halt() {
        let mut cpu = IntCodeCPU::new(vec![99]);
        assert_eq!(cpu.running, true);
        assert_eq!(cpu.pc, 0);
        cpu.execute();
        assert_eq!(cpu.running, false);   
        assert_eq!(cpu.pc, 0);
    }

    #[test]
    fn test_intcode_add() {
        let mut cpu = IntCodeCPU::new(vec![1,0,0,0,99]);
        assert_eq!(cpu.memory[0], 1);
        assert_eq!(cpu.pc, 0);
        cpu.execute();
        assert_eq!(cpu.pc, 4);
        assert_eq!(cpu.memory[0], 2);   
    }

    #[test]
    fn test_intcode_mul() {
        let mut cpu = IntCodeCPU::new(vec![2,3,0,3,99]);
        assert_eq!(cpu.memory[3], 3);
        assert_eq!(cpu.pc, 0);
        cpu.execute();
        assert_eq!(cpu.pc, 4);
        assert_eq!(cpu.memory[3], 6);   
    }

    #[test]
    fn test_intcode_large_mul() {
        let mut cpu = IntCodeCPU::new(vec![2,4,4,5,99,0]);
        assert_eq!(cpu.memory[5], 0);
        assert_eq!(cpu.pc, 0);
        cpu.execute();
        assert_eq!(cpu.pc, 4);
        assert_eq!(cpu.memory[5], 9801);   
    }

    #[test]
    fn test_intcode_multistep() {
        let mut cpu = IntCodeCPU::new(vec![1,1,1,4,99,5,6,0,99]);
        assert_eq!(cpu.memory[4], 99);
        assert_eq!(cpu.pc, 0);
        assert_eq!(cpu.running, true);
        cpu.execute();
        assert_eq!(cpu.pc, 4);
        assert_eq!(cpu.running, true);
        assert_eq!(cpu.memory[4], 2);
        assert_eq!(cpu.memory[0], 1);
        cpu.execute();
        assert_eq!(cpu.pc, 8);
        assert_eq!(cpu.running, true);
        assert_eq!(cpu.memory[4], 2);
        assert_eq!(cpu.memory[0], 30);
        cpu.execute();
        assert_eq!(cpu.pc, 8);
        assert_eq!(cpu.running, false);
    }

}