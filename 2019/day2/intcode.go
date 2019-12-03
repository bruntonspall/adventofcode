package main

func advance(memory []int, pc int) []int {
	switch memory[pc] {
	case 1:
		memory[memory[pc+3]] = memory[memory[pc+1]] + memory[memory[pc+2]]
	case 2:
		memory[memory[pc+3]] = memory[memory[pc+1]] * memory[memory[pc+2]]
	}
	return memory
}

func runIntCode(memory []int) []int {
	pc := 0
	for memory[pc] != 99 {
		memory = advance(memory, pc)
		pc += 4
	}
	return memory
}
