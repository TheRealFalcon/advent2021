use std::{collections::HashMap, str::FromStr};

use itertools::{zip, Itertools, Zip};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Segment {
    point1: Point,
    point2: Point,
}

#[derive(Debug)]
struct CoveredPoint {
    point: Point,
    count: u32,
}

impl FromStr for Segment {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (point1_str, point2_str) = s.split(" -> ").collect_tuple().unwrap();

        let (point1_x, point1_y) = point1_str
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect_tuple()
            .unwrap();
        let point1 = Point {
            x: point1_x,
            y: point1_y,
        };

        let (point2_x, point2_y) = point2_str
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect_tuple()
            .unwrap();
        let point2 = Point {
            x: point2_x,
            y: point2_y,
        };

        Ok(Segment { point1, point2 })
    }
}

fn get_points1(segment: &Segment) -> Vec<Point> {
    let point1 = &segment.point1;
    let point2 = &segment.point2;
    if point1.x == point2.x {
        let range = if point1.y > point2.y {
            point2.y..=point1.y
        } else {
            point1.y..=point2.y
        };
        range.map(|y| Point { x: point1.x, y }).collect()
    } else if point1.y == point2.y {
        let range = if point1.x > point2.x {
            point2.x..=point1.x
        } else {
            point1.x..=point2.x
        };
        range.map(|x| Point { x, y: point1.y }).collect()
    } else {
        Vec::new()
    }
}

fn get_points2(segment: &Segment) -> Vec<Point> {
    let point1 = &segment.point1;
    let point2 = &segment.point2;
    if point1.x == point2.x {
        let range = if point1.y > point2.y {
            point2.y..=point1.y
        } else {
            point1.y..=point2.y
        };
        range.map(|y| Point { x: point1.x, y }).collect()
    } else if point1.y == point2.y {
        let range = if point1.x > point2.x {
            point2.x..=point1.x
        } else {
            point1.x..=point2.x
        };
        range.map(|x| Point { x, y: point1.y }).collect()
    } else if (point1.x as i32 - point2.x as i32).abs() == (point1.y as i32 - point2.y as i32).abs()
    {
        let value = (point1.x as i32 - point2.x as i32).abs() as u32;

        // Why Rust...why are your range types incompatible???
        let range: Vec<(u32, u32)> = if point1.x + value == point2.x && point1.y + value == point2.y
        {
            zip(point1.x..=point2.x, point1.y..=point2.y).collect()
        } else if point1.x + value == point2.x && point1.y - value == point2.y {
            zip(point1.x..=point2.x, (point2.y..=point1.y).rev()).collect()
        } else if point1.x - value == point2.x && point1.y + value == point2.y {
            zip(
                (point1.x - value..=point1.x).rev(),
                point2.y - value..=point2.y,
            )
            .collect()
        } else if point1.x - value == point2.x && point1.y - value == point2.y {
            zip(point2.x..=point1.x, point2.y..=point1.y).collect()
        } else {
            panic!("Mark!");
        };
        range.iter().map(|(x, y)| Point { x: *x, y: *y }).collect()
    } else {
        Vec::new()
    }
}

fn common_impl(input: &Vec<String>, get_points: fn(&Segment) -> Vec<Point>) {
    let segments: Vec<Segment> = input.iter().map(|x| x.parse().unwrap()).collect();

    let points: Vec<Point> = segments.iter().map(|x| get_points(x)).flatten().collect();
    let mut point_counts: HashMap<Point, u32> = HashMap::new();
    for point in points {
        let count = point_counts.entry(point).or_insert(0);
        *count += 1;
    }

    let mut total_count = 0;
    for (_point, count) in &point_counts {
        if *count > 1 {
            total_count += 1;
        }
    }
    dbg!(total_count);
}

pub fn part1(input: &Vec<String>) {
    common_impl(input, get_points1);
}

pub fn part2(input: &Vec<String>) {
    common_impl(input, get_points2);
}
