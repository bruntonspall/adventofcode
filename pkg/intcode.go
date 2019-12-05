package pkg

import (
	"math"
)

// IntCodeComputer is a basic computer that can run int code
type IntCodeComputer struct {
	Memory []int
	PC     int
	Input  []int
	Output []int
}

// Initialise the computer with a memory slice
func (icc *IntCodeComputer) Initialise(memory []int) {
	icc.Memory = make([]int, len(memory))
	copy(icc.Memory[:], memory)
	icc.Input = []int{}
	icc.Output = []int{}
	icc.PC = 0
}

func (icc *IntCodeComputer) getParam(index int) int {
	code := icc.Memory[icc.PC]
	pnum := int(math.Pow10(index + 1))
	var mode bool = (code/pnum)%2 != 0

	if mode {
		return icc.Memory[icc.PC+index]
	}
	return icc.Memory[icc.Memory[icc.PC+index]]

}

func (icc *IntCodeComputer) setParam(index int, value int) {
	code := icc.Memory[icc.PC]
	pnum := int(math.Pow10(index + 1))
	var mode bool = (code/pnum)%2 != 0

	if mode {
		icc.Memory[icc.PC+index] = value
	} else {
		icc.Memory[icc.Memory[icc.PC+index]] = value
	}
}

// Advance operates on the computer, executing the next instruction
func (icc *IntCodeComputer) Advance() {
	code := icc.Memory[icc.PC]
	opcode := code % 100

	switch opcode {
	case 1:
		icc.Memory[icc.Memory[icc.PC+3]] = icc.getParam(1) + icc.getParam(2)
		icc.PC += 4
	case 2:
		icc.Memory[icc.Memory[icc.PC+3]] = icc.getParam(1) * icc.getParam(2)
		icc.PC += 4
	case 3:
		icc.setParam(1, icc.Input[0])
		icc.Input = icc.Input[1:]
		icc.PC += 2
	case 4:
		icc.Output = append(icc.Output, icc.getParam(1))
		icc.PC += 2
	case 5:
		if icc.getParam(1) != 0 {
			icc.PC = icc.getParam(2)
		} else {
			icc.PC += 3
		}
	case 6:
		if icc.getParam(1) == 0 {
			icc.PC = icc.getParam(2)
		} else {
			icc.PC += 3
		}
	case 7:
		if icc.getParam(1) < icc.getParam(2) {
			icc.setParam(3, 1)
		} else {
			icc.setParam(3, 0)
		}
		icc.PC += 4
	case 8:
		if icc.getParam(1) == icc.getParam(2) {
			icc.setParam(3, 1)
		} else {
			icc.setParam(3, 0)
		}
		icc.PC += 4
	default:
		icc.PC++
	}
}

// Run operates the computer until it reaches a halt instruction
func (icc *IntCodeComputer) Run() {
	for icc.Memory[icc.PC] != 99 {
		icc.Advance()
	}
}
