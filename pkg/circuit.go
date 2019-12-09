package pkg

// Circuit is a collection of IntCodeComputers wired together in serial
type Circuit struct {
	computers []IntCodeComputer
	input     chan int
	output    chan int
}

// NewCircuit sets up a new set of circuits, all running the same program
func NewCircuit(numCircuits int, program []int) (circuit Circuit) {
	input := make(chan int, 2)
	t := input
	for i := 0; i < numCircuits; i++ {
		cpu := NewIntCodeComputer(program)
		cpu.Input = t
		t = cpu.Output
		circuit.computers = append(circuit.computers, *cpu)
	}
	circuit.input = input
	circuit.output = t
	return circuit
}

// SetPhase sets the phase of each computer.  The phase should be the same length as the number of computers
func (c *Circuit) SetPhase(phase []int) {
	for i := range c.computers {
		c.computers[i].AddInput(phase[i])
	}
}

// AddInput pushes an input into the circuit
func (c *Circuit) AddInput(in int) {
	c.input <- in
}

// GetOutput returns the final output of the circuit
func (c *Circuit) GetOutput() int {
	return <-c.output
}

// Run sets each computer running in its own GoRoutine
func (c *Circuit) Run() {
	for i := range c.computers {
		c.computers[i].Run()
	}
}

// RunToCompletion takes a set of computers, iniitialises their Phase and then pushing input in
func (c *Circuit) RunToCompletion(phase []int, input int) int {
	for i := range c.computers {
		c.computers[i].Run()
	}
	c.SetPhase(phase)
	c.input <- input
	var o = 0
	for o = range c.output {
		c.input <- o
	}
	return o
}
