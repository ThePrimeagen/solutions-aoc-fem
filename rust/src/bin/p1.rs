type Coordinate = (i32, i32);

fn get_initial() -> Coordinate {
    return (0, 0);
}

fn get_input() -> String {
    return "forward 5
down 5
forward 8
up 3
down 8
forward 2".to_string();
}

fn main() {
    let final_position: Coordinate = get_input()
        .lines()
        .fold(get_initial(), |mut pos, line| {
            let (t, value) = line.split_once(" ").unwrap();
            let value = str::parse::<i32>(value).unwrap();

            match t {
                "forward" => {
                    pos.0 += value;
                },
                "up" => {
                    pos.1 -= value;
                },
                "down" => {
                    pos.1 += value;
                },
                _ => unreachable!(),
            }

            return pos
        });

    println!("value: {} - {:?}", final_position.0 * final_position.1, final_position);

}
