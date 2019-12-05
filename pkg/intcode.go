package pkg

import "fmt"

// IntCodeComputer is a basic computer that can run int code
type IntCodeComputer struct {
	Memory []int
	PC     int
}

// Initialise the computer with a memory slice
func (icc *IntCodeComputer) Initialise(memory []int) {
	icc.Memory = make([]int, len(memory))
	copy(icc.Memory[:], memory)
	icc.PC = 0
}

// Advance operates on the computer, executing the next instruction
func (icc *IntCodeComputer) Advance() {
	code := icc.Memory[icc.PC]
	opcode := code % 100
	p1mode := code / 100 % 10
	p2mode := code / 1000 % 10
	p3mode := code / 10000 % 10
	var p1, p2 int
	if p1mode == 1 {
		p1 = icc.Memory[icc.PC+1]
	} else {
		p1 = icc.Memory[icc.Memory[icc.PC+1]]
	}
	if p2mode == 1 {
		p2 = icc.Memory[icc.PC+2]
	} else {
		p2 = icc.Memory[icc.Memory[icc.PC+2]]
	}
	fmt.Printf("%v => Op: %v Param1: %v(%v) Param2: %v(%v) Param3: %v\n", code, opcode, p1, p1mode, p2, p2mode, p3mode)

	switch opcode {
	case 1:
		icc.Memory[icc.Memory[icc.PC+3]] = p1 + p2
		icc.PC += 4
	case 2:
		icc.Memory[icc.Memory[icc.PC+3]] = p1 * p2
		icc.PC += 4
	}
}

// Run operates the computer until it reaches a halt instruction
func (icc *IntCodeComputer) Run() {
	for icc.Memory[icc.PC] != 99 {
		icc.Advance()
	}
}
