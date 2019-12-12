package pkg

import (
	"fmt"
	"math"
)

var globalid = 1

// IntCodeComputer is a basic computer that can run int code
type IntCodeComputer struct {
	Memory map[int]int
	PC     int
	Input  chan int
	Output chan int
	id     int
	base   int
	debug  bool
}

// NewIntCodeComputer creates a new IntCodeComputer
func NewIntCodeComputer(memory []int) *IntCodeComputer {
	icc := new(IntCodeComputer)
	icc.Memory = make(map[int]int)
	for i, k := range memory {
		icc.Memory[i] = k
	}
	icc.Input = make(chan int)
	icc.Output = make(chan int)
	icc.PC = 0
	icc.id = globalid
	globalid++
	return icc
}

// AddInput pushes an input into the computer
func (icc *IntCodeComputer) AddInput(v int) {
	icc.Input <- v
}

// GetOutput gets the last output digit
func (icc *IntCodeComputer) GetOutput() int {
	o := <-icc.Output
	return o
}

func (icc *IntCodeComputer) getParam(index int) int {
	code := icc.Memory[icc.PC]
	pnum := int(math.Pow10(index + 1))
	mode := (code / pnum) % 10
	var memloc int
	switch mode {
	case 0:
		memloc = icc.Memory[icc.PC+index]
	case 1:
		memloc = icc.PC + index
	case 2:
		memloc = icc.base + icc.Memory[icc.PC+index]
	}
	return icc.Memory[memloc]

}

func (icc *IntCodeComputer) setParam(index int, value int) {
	code := icc.Memory[icc.PC]
	pnum := int(math.Pow10(index + 1))
	mode := (code / pnum) % 10
	var memloc int
	switch mode {
	case 0:
		memloc = icc.Memory[icc.PC+index]
	case 1:
		memloc = icc.PC + index
	case 2:
		memloc = icc.base + icc.Memory[icc.PC+index]
	}

	icc.Memory[memloc] = value

}

// Advance operates on the computer, executing the next instruction
func (icc *IntCodeComputer) Advance() {
	code := icc.Memory[icc.PC]
	opcode := code % 100

	switch opcode {
	case 1:
		icc.setParam(3, icc.getParam(1)+icc.getParam(2))
		icc.PC += 4
	case 2:
		icc.setParam(3, icc.getParam(1)*icc.getParam(2))
		icc.PC += 4
	case 3:
		if icc.debug {
			fmt.Printf("%v: Waiting for input\n", icc.id)
		}
		in := <-icc.Input
		if icc.debug {
			fmt.Printf("%v: Got input %v\n", icc.id, in)
		}
		icc.setParam(1, in)

		icc.PC += 2
	case 4:
		out := icc.getParam(1)
		if icc.debug {
			fmt.Printf("%v: Waiting to output %v\n", icc.id, out)
		}
		icc.Output <- out
		if icc.debug {
			fmt.Printf("%v: Sent output\n", icc.id)
		}
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
	case 9:
		icc.base += icc.getParam(1)
		icc.PC += 2
	case 99:
		close(icc.Output)
		if icc.debug {
			fmt.Printf("%v: End\n", icc.id)
		}
	default:
		icc.PC++
	}
}

// Run operates the computer until it reaches a halt instruction
func (icc *IntCodeComputer) Run() (c chan int) {
	c = make(chan int, 2)
	go func() {
		if icc.debug {
			fmt.Printf("%v: START\n", icc.id)
		}
		for icc.Memory[icc.PC] != 99 {
			icc.Advance()
		}
		close(icc.Output)
		if icc.debug {
			fmt.Printf("%v: EOP\n", icc.id)
		}
		c <- 1
	}()
	return
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
