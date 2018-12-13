extern crate regex;
use regex::Regex;

extern crate image;

fn parse_int(b : &str) -> i32 {
    b.trim_left().parse().unwrap()
}

fn visualize(scale: i32, positions_x: &Vec<i32>, positions_y: &Vec<i32>, filename: &str) -> Option<()> {
    let min_x = positions_x.iter().min()?;
    let max_x = positions_x.iter().max()?;

    let min_y = positions_y.iter().min()?;
    let max_y = positions_y.iter().max()?;

    let width = ((max_x - min_x + 1)/scale) as u32;
    let height = ((max_y - min_y + 1)/scale) as u32;

    println!("{} x {}", width, height);

    let mut imgbuf = image::GrayImage::new(width+1, height+1);

    let positive_positions_x: Vec<u32> = positions_x.iter().map(|x| ((x - min_x)/scale) as u32).collect();
    let positive_positions_y: Vec<u32> = positions_y.iter().map(|y| ((y - min_y)/scale) as u32).collect();

    for (x, y) in positive_positions_x.iter().zip(positive_positions_y.iter()) {
        imgbuf.put_pixel(*x, *y, image::Luma([255 as u8]))
    }

    imgbuf.save(filename).map(|x| Some(x)).unwrap_or_else(|_| None)?;

    Some(())
}

fn step(n: i32, positions: &Vec<i32>, velocities: &Vec<i32>) -> Vec<i32> {
    positions.iter().zip(velocities).map(|(p, v)| p + n*v).collect()
}

fn main() {
    let content = include_str!("../../inputs/day10/part1.txt");
    let scale = 1;

    let re = Regex::new(r"position=<([- \d]+),([- \d]+)> velocity=<([- \d]+),([- \d]+)>").unwrap();
    let mut positions_x = Vec::new();
    let mut positions_y = Vec::new();
    let mut velocities_x = Vec::new();
    let mut velocities_y = Vec::new();
    for cap in re.captures_iter(content) {
        positions_x.push(parse_int(&cap[1]));
        positions_y.push(parse_int(&cap[2]));
        velocities_x.push(parse_int(&cap[3]));
        velocities_y.push(parse_int(&cap[4]));
    }

    for n in 0..100 {
        let positions_x_p = step(n+10100, &positions_x, &velocities_x);
        let positions_y_p = step(n+10100, &positions_y, &velocities_y);

        let positions: Vec<(&i32, &i32)> = positions_x_p.iter().zip(positions_y_p.iter()).collect();
        println!("{:?}", positions);


        visualize(scale, &positions_x_p, &positions_y_p, &format!("outputs/fractal_{}.png", n)).unwrap();
    }
}
