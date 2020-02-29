use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    //Real work it's done in the tests
    unimplemented!();
}

//Just a vector
struct Translation {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn origin() -> Self {
        Point { x: 0, y: 0 }
    }

    fn translate(&mut self, t: Translation) {
        self.x += t.x;
        self.y += t.y;
    }

    fn manhattan_distance_to_origin(&self) -> i32 {
        (self.x).abs() + (self.y).abs()
    }

    fn translate_from_str(&mut self, instruction: &str) {
        let dir: char = instruction.chars().next().unwrap();
        let distance: i32 = instruction
            .chars()
            .skip(1)
            .collect::<String>()
            .parse()
            .unwrap();
        let trans;
        match dir {
            'R' => {
                trans = Translation { x: distance, y: 0 };
            }
            'L' => {
                trans = Translation { x: -distance, y: 0 };
            }
            'U' => {
                trans = Translation { x: 0, y: distance };
            }
            'D' => {
                trans = Translation { x: 0, y: -distance };
            }
            _ => {
                trans = Translation { x: 0, y: 0 };
            }
        }
        self.translate(trans);
    }

    fn distance(&self, other: &Point) -> i32 {
        (((other.x - self.x) as f64).powf(2.0) + ((other.y - self.y) as f64).powf(2.0)).sqrt()
            as i32
    }
}

#[derive(Clone)]
enum Direction {
    Horizontal,
    Vertical,
}

struct Segment {
    start: Point,
    end: Point,
    dir: Direction,
    size: i32,
}

impl Segment {
    fn new(start: Point, end: Point) -> Self {
        //Precomputes the direction and size
        let (dir, size) = if start.x == end.x {
            (Direction::Vertical, (end.y - start.y).abs())
        } else {
            (Direction::Horizontal, (end.x - start.x).abs())
        };
        Segment {
            start,
            end,
            dir,
            size,
        }
    }

    fn intersection(&self, other: &Self) -> Option<Point> {
        let intersection: Point;
        match (self.dir.clone(), other.dir.clone()) {
            (Direction::Horizontal, Direction::Vertical) => {
                intersection = Point::new(other.start.x, self.start.y);
                if self.contains_point(&intersection) && other.contains_point(&intersection) {
                    Some(Point::new(other.start.x, self.end.y))
                } else {
                    None
                }
            }
            (Direction::Vertical, Direction::Horizontal) => {
                intersection = Point::new(self.start.x, other.start.y);
                if self.contains_point(&intersection) && other.contains_point(&intersection) {
                    Some(Point::new(self.start.x, other.start.y))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn contains_point(&self, point: &Point) -> bool {
        match self.dir {
            Direction::Horizontal => {
                self.start.y == point.y
                    && ((self.start.x..self.end.x).contains(&point.x)
                        || (self.end.x..self.start.x).contains(&point.x))
            }
            Direction::Vertical => {
                self.start.x == point.x
                    && ((self.start.y..self.end.y).contains(&point.y)
                        || (self.end.y..self.start.y).contains(&point.y))
            }
        }
    }
}

struct Wire(Vec<Segment>);

impl std::ops::Deref for Wire {
    type Target = Vec<Segment>;
    fn deref(&self) -> &Vec<Segment> {
        &self.0
    }
}

impl std::ops::DerefMut for Wire {
    fn deref_mut(&mut self) -> &mut Vec<Segment> {
        &mut self.0
    }
}

impl FromStr for Wire {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut wire: Wire = Wire(Vec::new());
        let mut old_point;
        let mut current_point = Point::origin();
        for instruction in s.split(',') {
            old_point = current_point.clone();
            current_point.translate_from_str(instruction);
            wire.push(Segment::new(old_point.clone(), current_point.clone()));
        }
        Ok(wire)
    }
}

impl Wire {
    //Returns the nearest point of intersection between two wires if exists, none otherwise.
    fn nearest_intersection(&self, other: &Wire) -> Option<Point> {
        let mut intersections: Vec<Point> = self.calculate_intersections(other);
        intersections.sort_by_key(|a| a.manhattan_distance_to_origin());
        if intersections[0] != Point::origin() {
            Some(intersections[0].clone())
        } else if intersections.len() >= 2 {
            Some(intersections[1].clone())
        } else {
            None
        }
    }

    //Given two wires, returns all the points in which they interesects.
    fn calculate_intersections(&self, other: &Wire) -> Vec<Point> {
        let mut intersections: Vec<Point> = Vec::new();
        for segment in self.iter() {
            for other_segment in other.iter() {
                let intersection = segment.intersection(&other_segment);
                if let Some(point) = intersection {
                    intersections.push(point);
                }
            }
        }
        intersections
    }

    //Calculate the signal delay between this wire and a point
    fn signal_delay(&self, point: &Point) -> i32 {
        let index: usize;
        let path = self
            .iter()
            .take_while(|s| !s.contains_point(point))
            .map(|s| s.size);

        index = path.clone().count();
        let final_part = self
            .get(index)
            .map(|s| s.start.distance(point))
            .unwrap_or(0);
        path.sum::<i32>() + final_part
    }

    //Calculate the minimum delay
    fn lowest_delay(&self, other: &Wire) -> i32 {
        let intersections = self.calculate_intersections(other);
        intersections
            .iter()
            .filter(|s| **s != Point::origin())
            .map(|inter| self.signal_delay(&inter) + other.signal_delay(&inter))
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example_1() {
        let first_wire_instructions = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
        let second_wire_instructions = "U62,R66,U55,R34,D71,R55,D58,R83";
        //Generate the wires
        let first_wire: Wire = Wire::from_str(first_wire_instructions).unwrap();
        let second_wire: Wire = Wire::from_str(second_wire_instructions).unwrap();
        //Calculate the intersections
        let point = first_wire.nearest_intersection(&second_wire);
        assert_eq!(point.unwrap().manhattan_distance_to_origin(), 159);
    }

    #[test]
    fn part_1_example_2() {
        let first_wire_instructions = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
        let second_wire_instructions = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        //Generate the wires
        let first_wire: Wire = Wire::from_str(first_wire_instructions).unwrap();
        let second_wire: Wire = Wire::from_str(second_wire_instructions).unwrap();
        //Calculate the intersections
        let point = first_wire.nearest_intersection(&second_wire);
        assert_eq!(point.unwrap().manhattan_distance_to_origin(), 135);
    }

    #[test]
    fn part_1_main_problem() {
        let first_wire_instructions = include_str!("../input/day3_wire1.txt");
        let second_wire_instructions = include_str!("../input/day3_wire2.txt");
        let first_wire: Wire = Wire::from_str(first_wire_instructions).unwrap();
        let second_wire: Wire = Wire::from_str(second_wire_instructions).unwrap();
        let point = first_wire.nearest_intersection(&second_wire);
        assert_eq!(point.unwrap().manhattan_distance_to_origin(), 403);
    }

    #[test]
    fn part_2_example_1() {
        let first_wire_instructions = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
        let second_wire_instructions = "U62,R66,U55,R34,D71,R55,D58,R83";
        let first_wire: Wire = Wire::from_str(first_wire_instructions).unwrap();
        let second_wire: Wire = Wire::from_str(second_wire_instructions).unwrap();
        let delay = first_wire.lowest_delay(&second_wire);
        assert_eq!(delay, 610);
    }

    #[test]
    fn part_2_example_2() {
        let first_wire_instructions = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
        let second_wire_instructions = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let first_wire: Wire = Wire::from_str(first_wire_instructions).unwrap();
        let second_wire: Wire = Wire::from_str(second_wire_instructions).unwrap();
        let delay = first_wire.lowest_delay(&second_wire);
        assert_eq!(delay, 410);
    }

    #[test]
    fn part_2_main_problem() {
        let first_wire_instructions = include_str!("../input/day3_wire1.txt");
        let second_wire_instructions = include_str!("../input/day3_wire2.txt");
        let first_wire: Wire = Wire::from_str(first_wire_instructions).unwrap();
        let second_wire: Wire = Wire::from_str(second_wire_instructions).unwrap();
        let delay = first_wire.lowest_delay(&second_wire);
        assert_eq!(delay, 4158);
    }
}
