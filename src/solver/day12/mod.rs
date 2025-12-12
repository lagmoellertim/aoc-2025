mod part1;
mod part2;

#[derive(Debug)]
enum GridCell {
    Empty,
    Present,
}

type PresentShape = Vec<Vec<GridCell>>;

struct Region {
    width: i64,
    height: i64,

    required_quantities: Vec<(usize, i64)>,
}

struct PackagingManifest {
    shapes: Vec<PresentShape>,
    regions: Vec<Region>,
}

fn parse_packaging_manifest(input: &str) -> PackagingManifest {
    let parts = input.split("\n\n").collect::<Vec<_>>();

    let (region_block, shape_blocks) = parts.split_last().unwrap();

    let shapes = shape_blocks
        .iter()
        .map(|shape| {
            shape
                .lines()
                .skip(1)
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' => GridCell::Empty,
                            '#' => GridCell::Present,
                            _ => unreachable!(),
                        })
                        .collect()
                })
                .collect()
        })
        .collect();

    let regions = region_block
        .lines()
        .map(|line| {
            let (dimensions_str, quantities_str) = line.split_once(": ").unwrap();

            let (width_str, height_str) = dimensions_str.split_once("x").unwrap();
            let width = width_str.parse::<i64>().unwrap();
            let height = height_str.parse::<i64>().unwrap();

            let required_quantities = quantities_str
                .split_whitespace()
                .enumerate()
                .map(|(shape_idx, quantity_str)| (shape_idx, quantity_str.parse::<i64>().unwrap()))
                .collect::<Vec<_>>();

            Region {
                width,
                height,
                required_quantities,
            }
        })
        .collect();

    PackagingManifest {
        shapes,
        regions,
    }
}
