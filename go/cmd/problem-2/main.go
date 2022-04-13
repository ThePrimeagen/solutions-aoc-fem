package main

import (
	"fmt"
	"log"
	"strconv"
	"strings"
)

type Point struct {
    x int
    y int
}

type Line struct {
    p1 Point;
    p2 Point;
}

func newPoint(point string) (*Point, error) {
    p := strings.Split(point, ",")
    x, err := strconv.ParseInt(p[0], 10, 0)
    if err != nil {
        return nil, err
    }

    y, err := strconv.ParseInt(p[1], 10, 0)
    if err != nil {
        return nil, err
    }

    return &Point {
        x: int(x),
        y: int(y),
    }, nil
}

func newLine(line string) (*Line, error) {
    parts := strings.Split(line, " -> ")
    p1, err := newPoint(parts[0])
    if err != nil {
        return nil, err
    }

    p2, err := newPoint(parts[1])
    if err != nil {
        return nil, err
    }

    return &Line {
        p1: *p1,
        p2: *p2,
    }, nil
}

func abs(a int) int {
    if a < 0 {
        return a * -1
    }

    return a
}

func min(a, b int) int {
    if a > b {
        return b
    }

    return a
}

func (l *Line) mark(graph [][]int) {

    xDiff := abs(l.p1.x - l.p2.x);
    yDiff := abs(l.p1.y - l.p2.y);

    xStarting := min(l.p1.x, l.p2.x);
    yStarting := min(l.p1.y, l.p2.y);

    if xDiff != 0 && yDiff != 0 {
        return;
    }

    for x := xStarting; x <= xStarting + xDiff; x++ {
        for y := yStarting; y <= yStarting + yDiff; y++ {
            graph[x][y] += 1;
        }
    }
}

func getMax(lines []*Line) (int, int) {
    x := 0
    y := 0

    for _, line := range lines {
        if line.p1.x > x {
            x = line.p1.x
        }
        if line.p2.x > x {
            x = line.p2.x
        }
        if line.p1.y > y {
            y = line.p1.y
        }
        if line.p2.y > y {
            y = line.p2.y
        }
    }

    return x, y
}

func getInput() string {
    return `0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2`;
}

func main() {
    input := strings.Split(getInput(), "\n")
    lines := make([]*Line, len(input))

    for idx, l := range input {
        line, err := newLine(l)

        if err != nil {
            log.Fatalf("This is the end my friend %v\n", err)
        }

        lines[idx] = line
    }

    xMax, yMax := getMax(lines)
    graph := make([][]int, xMax + 1)
    for idx := range graph {
        graph[idx] = make([]int, yMax + 1)
    }

    for _, line := range lines {
        line.mark(graph)
    }

    fmt.Printf("graph: %+v\n", graph)
}

