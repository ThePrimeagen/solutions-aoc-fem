package main

import (
	"log"
	"strconv"
	"strings"
)

type Coordinate = [2]int;

func getInitial() Coordinate {
    return [2]int{0, 0}
}

func getInput() string {
    return `forward 5
down 5
forward 8
up 3
down 8
forward 2`
}

func main() {
    lines := strings.Split(getInput(), "\n")
    pos := getInitial();

    for _, line := range lines {
        parts := strings.Split(line, " ");
        value, err := strconv.ParseInt(parts[1], 10, 0)

        if err != nil {
            log.Fatalf("Ohh no I have an error %v\n", err)
        }

        switch parts[0] {
        case "forward":
            pos[0] += int(value)
        case "up":
            pos[1] -= int(value)
        case "down":
            pos[1] += int(value)
        }
    }

    log.Printf("we have found %v\n", pos[0] * pos[1])
}
