pub struct IntCodeCPU {
    pub memory: Vec<i32>,
    pub pc: usize,
    pub running: bool,
    pub input: Vec<i32>,
    pub output: Vec<i32>,
}

#[derive(Debug, Clone, Copy)]
pub enum Modes {
    Position,
    Immediate,
}

impl Modes {
    pub fn from(value: i32) -> Self {
        match value {
            0 => Modes::Position,
            1 => Modes::Immediate,
            _ => panic!("Unknown mode: {}", value),
        }
    }
}

pub struct IntCodeOpParams {
    modes: Vec<Modes>,
    size: usize,
}
pub enum IntCodeOp {
    Add(IntCodeOpParams),
    Mul(IntCodeOpParams),
    Input(IntCodeOpParams),
    Output(IntCodeOpParams),
    Halt(IntCodeOpParams),
}

impl IntCodeOp {
    pub fn from_opcode(opcode: i32) -> Self {
        match opcode % 100 {
            1 => IntCodeOp::Add(IntCodeOpParams {
                modes: vec![
                    Modes::from((opcode / 100) % 10),
                    Modes::from((opcode / 1000) % 10),
                ],
                size: 4,
            }),
            2 => IntCodeOp::Mul(IntCodeOpParams {
                modes: vec![
                    Modes::from((opcode / 100) % 10),
                    Modes::from((opcode / 1000) % 10),
                ],
                size: 4,
            }),
            3 => IntCodeOp::Input(IntCodeOpParams {
                modes: vec![Modes::from((opcode / 100) % 10)],
                size: 2,
            }),
            4 => IntCodeOp::Output(IntCodeOpParams {
                modes: vec![Modes::from((opcode / 100) % 10)],
                size: 2,
            }),
            99 => IntCodeOp::Halt(IntCodeOpParams {
                size: 1,
                modes: vec![],
            }),
            _ => panic!("Unknown opcode: {}", opcode),
        }
    }
    pub fn get_parameter(&self, cpu: &mut IntCodeCPU, offset: usize, mode: Modes) -> i32 {
        let param_value = cpu.memory[cpu.pc + offset];
        match mode {
            Modes::Position => {
                cpu.memory[param_value as usize] // Position mode
            }
            Modes::Immediate => param_value, // Immediate mode
        }
    }

    pub fn execute(&self, cpu: &mut IntCodeCPU) {
        match self {
            IntCodeOp::Add(params) => {
                let target = cpu.memory[cpu.pc + 3] as usize;
                let p1 = self.get_parameter(cpu, 1, params.modes[0]);
                let p2 = self.get_parameter(cpu, 2, params.modes[1]);
                cpu.memory[target] = p1 + p2;
                cpu.pc += params.size;
            }
            IntCodeOp::Mul(params) => {
                let target = cpu.memory[cpu.pc + 3] as usize;
                let p1 = self.get_parameter(cpu, 1, params.modes[0]);
                let p2 = self.get_parameter(cpu, 2, params.modes[1]);
                cpu.memory[target] = p1 * p2;
                cpu.pc += params.size;
            }
            IntCodeOp::Input(params) => {
                let target = cpu.memory[cpu.pc + 1] as usize;
                if cpu.input.is_empty() {
                    panic!("No input available for IntCode Input operation");
                }
                let input_value = cpu.input.remove(0);
                match params.modes[0] {
                    Modes::Position => {cpu.memory[target] = input_value}   // Position mode
                    Modes::Immediate => {panic!("Writes should never be immediate: {}", input_value)} // Immediate mode
                }
                cpu.pc += params.size;
            }
            IntCodeOp::Output(params) => {
                let target = cpu.memory[cpu.pc + 1];
                match params.modes[0] {
                    Modes::Position => {cpu.output.push(cpu.memory[target as usize]);}   // Position mode
                    Modes::Immediate => {cpu.output.push(target)} // Immediate mode
                }
                cpu.pc += params.size;
            }
            IntCodeOp::Halt(params) => {
                cpu.running = false;
                cpu.pc += params.size;
            }
        }
    }
}

impl IntCodeCPU {
    pub fn new(program: Vec<i32>) -> Self {
        IntCodeCPU {
            memory: program,
            pc: 0,
            running: true,
            input: vec![],
            output: Vec::new(),
        }
    }
    pub fn new_with_IO(program: Vec<i32>, input: Vec<i32>) -> Self {
        IntCodeCPU {
            memory: program,
            pc: 0,
            running: true,
            input: input,
            output: Vec::new(),
        }
    }
    pub fn execute(&mut self) {
        let opcode_value = self.memory[self.pc];
        let opcode = IntCodeOp::from_opcode(opcode_value);
        opcode.execute(self);
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
        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn test_intcode_add() {
        let mut cpu = IntCodeCPU::new(vec![1, 0, 0, 0, 99]);
        assert_eq!(cpu.memory[0], 1);
        assert_eq!(cpu.pc, 0);
        cpu.execute();
        assert_eq!(cpu.pc, 4);
        assert_eq!(cpu.memory[0], 2);
    }

    #[test]
    fn test_intcode_mul() {
        let mut cpu = IntCodeCPU::new(vec![2, 3, 0, 3, 99]);
        assert_eq!(cpu.memory[3], 3);
        assert_eq!(cpu.pc, 0);
        cpu.execute();
        assert_eq!(cpu.pc, 4);
        assert_eq!(cpu.memory[3], 6);
    }

    #[test]
    fn test_intcode_large_mul() {
        let mut cpu = IntCodeCPU::new(vec![2, 4, 4, 5, 99, 0]);
        assert_eq!(cpu.memory[5], 0);
        assert_eq!(cpu.pc, 0);
        cpu.execute();
        assert_eq!(cpu.pc, 4);
        assert_eq!(cpu.memory[5], 9801);
    }

    #[test]
    fn test_intcode_multistep() {
        let mut cpu = IntCodeCPU::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
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
        assert_eq!(cpu.pc, 9);
        assert_eq!(cpu.running, false);
    }

    #[test]
    fn test_intcode_add_value_value() {
        let mut cpu = IntCodeCPU::new(vec![1101, 5, 7, 0, 99]);
        assert_eq!(cpu.memory[0], 1101);
        cpu.execute();
        assert_eq!(cpu.pc, 4);
        assert_eq!(cpu.memory[0], 12);
    }

    #[test]
    fn test_intcode_mul_value_value() {
        let mut cpu = IntCodeCPU::new(vec![1002, 4, 3, 4, 33]);
        assert_eq!(cpu.memory[0], 1002);
        cpu.execute();
        assert_eq!(cpu.pc, 4);
        assert_eq!(cpu.memory[4], 99);
    }
    #[test]
    fn test_intcode_add_negative() {
        let mut cpu = IntCodeCPU::new(vec![1101, 100, -1, 4, 0]);
        assert_eq!(cpu.memory[4], 0);
        cpu.execute();
        assert_eq!(cpu.pc, 4);
        assert_eq!(cpu.memory[4], 99);
    }
    #[test]
    fn test_intcode_immediete_position_combos() {
        for (expected, program) in &[
            (30, vec![1, 5, 6, 4, 0, 10, 20]),
            (25, vec![101, 5, 6, 4, 0, 10, 20]),
            (16, vec![1001, 5, 6, 4, 0, 10, 20]),
            (11, vec![1101, 5, 6, 4, 0, 10, 20]),
            (200, vec![2, 5, 6, 4, 0, 10, 20]),
            (100, vec![102, 5, 6, 4, 0, 10, 20]),
            (60, vec![1002, 5, 6, 4, 0, 10, 20]),
            (30, vec![1102, 5, 6, 4, 0, 10, 20]),
        ] {
            let mut cpu = IntCodeCPU::new(program.clone());
            cpu.execute();
            assert_eq!(cpu.memory[4], *expected);
        }
    }
    #[test]
    fn test_intcode_input() {
        let mut cpu = IntCodeCPU::new_with_IO(vec![3, 2, 0], vec![99]);
        assert_eq!(cpu.input, vec![99]);
        assert_eq!(cpu.memory[2], 0);
        cpu.execute();
        assert_eq!(cpu.input, vec![]);
        assert_eq!(cpu.pc, 2);
        assert_eq!(cpu.memory[2], 99);
    }
    #[test]
    fn test_intcode_output() {
        let mut cpu = IntCodeCPU::new_with_IO(vec![4, 2, 99], vec![]);
        assert_eq!(cpu.output, vec![]);
        cpu.execute();
        assert_eq!(cpu.output, vec![99]);
        assert_eq!(cpu.pc, 2);
    }
    #[test]
    fn test_intcode_immediate_output() {
        let mut cpu = IntCodeCPU::new_with_IO(vec![104, 2, 99], vec![]);
        assert_eq!(cpu.output, vec![]);
        cpu.execute();
        assert_eq!(cpu.output, vec![2]);
        assert_eq!(cpu.pc, 2);
    }
    #[test]
    fn test_intcode_intcode_mul_io() {
        let mut cpu =
            IntCodeCPU::new_with_IO(vec![3, 9, 1001, 9, 5, 10, 4, 10, 99, 0, 0], vec![52]);
        assert_eq!(cpu.output, vec![]);
        cpu.run();
        assert_eq!(cpu.memory, vec![3, 9, 1001, 9, 5, 10, 4, 10, 99, 52, 57]);
        assert_eq!(cpu.output, vec![57]);
    }
}
