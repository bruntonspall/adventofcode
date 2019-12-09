package pkg

import (
	"fmt"
	"math"
)

var globalid = 1

// IntCodeComputer is a basic computer that can run int code
type IntCodeComputer struct {
	Memory []int
	PC     int
	Input  chan int
	Output chan int
	id     int
}

func NewIntCodeComputer(memory []int) *IntCodeComputer {
	icc := new(IntCodeComputer)
	icc.Memory = make([]int, len(memory))
	copy(icc.Memory[:], memory)
	icc.Input = make(chan int)
	icc.Output = make(chan int)
	icc.PC = 0
	icc.id = globalid
	globalid++
	return icc
}

func (icc *IntCodeComputer) AddInput(v int) {
	icc.Input <- v
}

func (icc *IntCodeComputer) GetOutput() int {
	o := <-icc.Output
	return o
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
		fmt.Printf("%v: Waiting for input\n", icc.id)
		in := <-icc.Input
		fmt.Printf("%v: Got input %v\n", icc.id, in)
		icc.setParam(1, in)

		icc.PC += 2
	case 4:
		out := icc.getParam(1)
		fmt.Printf("%v: Waiting to output %v\n", icc.id, out)
		icc.Output <- out
		fmt.Printf("%v: Sent output\n", icc.id)
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
	case 99:
		close(icc.Output)
		fmt.Printf("%v: End\n", icc.id)
	default:
		icc.PC++
	}
}

// Run operates the computer until it reaches a halt instruction
func (icc *IntCodeComputer) Run() {
	go func() {
		fmt.Printf("%v: START\n", icc.id)
		for icc.Memory[icc.PC] != 99 {
			icc.Advance()
		}
		close(icc.Output)
		fmt.Printf("%v: EOP\n", icc.id)
	}()
}

// SyncRun runs the program until the program stops
func (icc *IntCodeComputer) SyncRun() {
	c := make(chan int)
	go func() {
		fmt.Printf("%v: START\n", icc.id)
		for icc.Memory[icc.PC] != 99 {
			icc.Advance()
		}
		close(icc.Output)
		fmt.Printf("%v: EOP\n", icc.id)
		c <- 1
	}()
	<-c
}

func (icc *IntCodeComputer) GetOutputs() (result []int) {
	for r := range icc.Output {
		result = append(result, r)
	}
	return
}
