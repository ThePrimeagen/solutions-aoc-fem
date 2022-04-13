#![feature(int_abs_diff)]

struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: &str, y: &str) -> Result<Point, OurError> {
        let x = if let Ok(x) = str::parse::<usize>(x) {
            x
        } else {
            return Err(OurError::BadPointData);
        };

        let y = if let Ok(y) = str::parse::<usize>(y) {
            y
        } else {
            return Err(OurError::BadPointData);
        };

        return Ok(Point {
            x,
            y
        });
    }
}

struct Coordinate {
    p1: Point,
    p2: Point,
}

impl Coordinate {
    fn mark(&self, graph: &mut Vec<Vec<usize>>) {
        let x_diff = self.p1.x.abs_diff(self.p2.x);
        let y_diff = self.p1.y.abs_diff(self.p2.y);

        let x_starting = std::cmp::min(self.p1.x, self.p2.x);
        let y_starting = std::cmp::min(self.p1.y, self.p2.y);

        if x_diff != 0 && y_diff != 0 {
            return;
        }

        for x in x_starting..=(x_starting + x_diff) {
            for y in y_starting..=(y_starting + y_diff) {
                graph[x][y] += 1;
            }
        }
    }
}

enum OurError {
    BadPointData,
    BadCoordinateData,
}

impl TryInto<Point> for &str {
    type Error = OurError;
    fn try_into(self) -> Result<Point, Self::Error> {
        let split = self.split_once(",");

        match split {
            Some((x, y)) => {
                return Ok(Point::new(x, y)?);
            },
            None => {
                return Err(OurError::BadPointData);
            }
        }
    }
}

impl TryInto<Coordinate> for &str {
    type Error = OurError;

    fn try_into(self) -> Result<Coordinate, Self::Error> {
        let parts = self.split_once(" -> ");
        match parts {
            Some((p1, p2)) => {
                return Ok(Coordinate {
                    p1: p1.try_into()?,
                    p2: p2.try_into()?,
                });
            },
            None => {
                return Err(OurError::BadCoordinateData);
            }
        }
    }
}

fn get_input() -> &'static str {
    return "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
}

fn get_max(vec: &Vec<Coordinate>) -> (usize, usize) {
    let mut max_x = 0;
    let mut max_y = 0;

    for coord in vec {
        if coord.p1.x > max_x {
            max_x = coord.p1.x
        }
        if coord.p2.x > max_x {
            max_x = coord.p2.x
        }
        if coord.p1.y > max_y {
            max_y = coord.p1.y
        }
        if coord.p2.y > max_y {
            max_y = coord.p2.y
        }
    }

    return (max_x, max_y);
}

fn main() {
    let data = get_input()
        .lines()
        .flat_map(|line| line.try_into())
        .collect::<Vec<Coordinate>>();

    let (max_x, max_y) = get_max(&data);
    let mut graph: Vec<Vec<usize>> = vec![];
    graph.resize(max_x + 1, vec![]);

    for idx in 0..=max_x {
        graph[idx].resize(max_y + 1, 0);
    }

    for coord in &data {
        coord.mark(&mut graph);
    }

    println!("{:?}", graph);
}
