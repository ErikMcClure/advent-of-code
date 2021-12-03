use std::collections::HashMap;
use std::fs;

struct Image {
    id: i32,
    rows: [u16; 10],
    edges: [u16; 4], // LTRB - left top right bottom
}

fn rev(x: u16) -> u16 {
    let mut y = 0;
    for i in 0..10 {
        y |= ((x & (1 << i)) >> i) << (9 - i);
    }

    return y;
}

fn parse_image(x: &str) -> (i32, Image) {
    let mut pieces = x.split(':');
    let id: i32 = pieces
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .next()
        .unwrap()
        .parse()
        .unwrap();

    let rowlist = pieces.next().unwrap().trim().split_whitespace().map(|x| {
        x.chars()
            .fold(0, |v, x| if x == '#' { (v << 1) | 1 } else { (v << 1) | 0 })
    });

    let mut img = Image {
        id: id,
        rows: [0u16; 10],
        edges: [0u16; 4],
    };

    let mut i = 0;
    for row in rowlist {
        img.rows[i] = row;
        i += 1;
    }
    if i != 10 {
        panic!("Unexpected image height");
    }

    img.edges[0] = rev(img
        .rows
        .iter()
        .fold(0, |v, x| (v << 1) | ((x & (1 << 9)) >> 9)));
    img.edges[1] = img.rows[0];
    img.edges[2] = img.rows.iter().fold(0, |v, x| (v << 1) | (x & 1));
    img.edges[3] = rev(img.rows[9]);

    return (img.id, img);
}

fn insert_image(img: &Image, x: usize, y: usize, r: i8, f: bool) {}
pub fn main() {
    let contents = fs::read_to_string("aoc20.txt").unwrap();
    let images: HashMap<i32, Image> = contents.split("\n\n").map(parse_image).collect();
    let mut nodes: HashMap<i32, Vec<i32>> = images.iter().map(|x| (*x.0, Vec::new())).collect();

    for src in images.iter() {
        'outer: for image in images.iter().filter(|x| x.0 != src.0) {
            for i in 0..4 {
                for j in 0..4 {
                    if src.1.edges[i] == image.1.edges[j] || // flip
                        src.1.edges[i] == rev(image.1.edges[j])
                    {
                        nodes.get_mut(src.0).unwrap().push(*image.0);
                        continue 'outer;
                    }
                }
            }
        }
    }

    let corner = nodes.iter().filter(|x| x.1.len() == 2).next().unwrap();
    let side = (nodes.len() as f64).sqrt() as i32;
    let image: Vec<u8> = Vec::new();
    image.resize_default(side * side * 8);
    let start = corner 
    let mut cur = &corner;
    for x in 0..side {
        for y in 0..side {
            
        }
    }
    print!("{}: {} ({})", corner.0, corner.1.len());

    /*for node in nodes {
        print!("{}: {}", node.0, node.1.len());
        let e = &images[&node.0];
        println!(
            " | {} {} {} {}",
            e.edges[0], e.edges[1], e.edges[2], e.edges[3]
        );
    }*/
}
