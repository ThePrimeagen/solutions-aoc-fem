
function getInput(): string {
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

type Point = [number, number];

class Line {
    private p1: Point;
    private p2: Point;

    constructor(line: string) {
        const parts = line.split(" -> ");
        this.p1 = parts[0].split(",").map(x => +x) as Point;
        this.p2 = parts[1].split(",").map(x => +x) as Point;
    }

    mark(graph: number[][]) {
        const xDiff = Math.abs(this.p1[0] - this.p2[0]);
        const yDiff = Math.abs(this.p1[1] - this.p2[1]);

        const xStarting = Math.min(this.p1[0], this.p2[0]);
        const yStarting = Math.min(this.p1[1], this.p2[1]);

        if (xDiff != 0 && yDiff != 0) {
            return;
        }

        for (let x = xStarting; x <= xStarting + xDiff; x++) {
            for (let y = yStarting; y <= yStarting + yDiff; y++) {
                if (!graph[x]) {
                    graph[x] = [];
                }
                if (!graph[x][y]) {
                    graph[x][y] = 0;
                }

                graph[x][y] += 1;
            }
        }
    }
}


function solve() {
    const graph: number[][] = [];
    getInput().
        split("\n").
        map(x => new Line(x)).
        forEach(line => line.mark(graph));

    console.log(graph.map(line => line.join(", ")).join("\n"));
}

solve();

