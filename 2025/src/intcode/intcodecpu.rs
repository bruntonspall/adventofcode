
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

    pub fn get_parameter(&self, offset: usize, mode: u32) -> u32 {
        
        let param_value = self.memory[self.pc + offset];
        match mode {
            0 => { 
                println!("Getting parameter at offset {} with mode {} = {}", offset, mode, self.memory[param_value as usize]);
                self.memory[param_value as usize] // Position mode
            },
            1 => {
                println!("Getting parameter at offset {} with mode {} = {}", offset, mode, param_value);
                param_value},                       // Immediate mode
            _ => panic!("Unknown parameter mode: {}", mode),
        }
    }

    pub fn execute(&mut self) {
        let opcode = self.memory[self.pc];
        match opcode % 100{
            1 => {
                let src1 = self.get_parameter(1, (opcode / 100) % 10);
                let src2 = self.get_parameter(2, (opcode / 1000) % 10);
                let dest = self.memory[self.pc+3] as usize;
                self.memory[dest] = src1 + src2;
                self.pc += 4;
            }
            2 => {
                let src1 = self.get_parameter(1, (opcode / 100) % 10);
                let src2 = self.get_parameter(2, (opcode / 1000) % 10);
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

    #[test]
    fn test_intcode_add_value_value() {
        let mut cpu = IntCodeCPU::new(vec![1101,5,7,0,99]);
        assert_eq!(cpu.memory[0], 1101);   
        cpu.execute();
        assert_eq!(cpu.pc, 4);
        assert_eq!(cpu.memory[0], 12);   
    }

    #[test]
    fn test_intcode_mul_value_value() {
        let mut cpu = IntCodeCPU::new(vec![1002,4,3,4,33]);
        assert_eq!(cpu.memory[0], 1002);   
        cpu.execute();
        assert_eq!(cpu.pc, 4);
        assert_eq!(cpu.memory[4], 99);   
    }


}