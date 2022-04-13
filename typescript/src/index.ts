type Coordinate = [number, number];

function getInitial(): Coordinate {
    return [0, 0];
}

function getInput(): string {
    return `forward 5
down 5
forward 8
up 3
down 8
forward 2`;
}

function solve() {
    const input = getInput();

    const position = input.
        split("\n").
        reduce((position, line) => {
            const parts = line.split(" ");

            switch (parts[0]) {
                case "forward":
                    position[0] += +parts[1];
                    break;
                case "up":
                    position[1] -= +parts[1];
                    break;
                case "down":
                    position[1] += +parts[1];
                    break;
                default:
                    throw new Error("what have you done?");
            }

            return position;
        }, getInitial());

    console.log("position", position[0] * position[1]);
}

solve();
