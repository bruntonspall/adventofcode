package pkg

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
	switch icc.Memory[icc.PC] {
	case 1:
		icc.Memory[icc.Memory[icc.PC+3]] = icc.Memory[icc.Memory[icc.PC+1]] + icc.Memory[icc.Memory[icc.PC+2]]
		icc.PC += 4
	case 2:
		icc.Memory[icc.Memory[icc.PC+3]] = icc.Memory[icc.Memory[icc.PC+1]] * icc.Memory[icc.Memory[icc.PC+2]]
		icc.PC += 4
	}
}

// Run operates the computer until it reaches a halt instruction
func (icc *IntCodeComputer) Run() {
	for icc.Memory[icc.PC] != 99 {
		icc.Advance()
	}
}
