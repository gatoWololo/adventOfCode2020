use anyhow;
use anyhow::Context;

#[derive(Copy, Clone, Debug)]
enum Terrain {
    Tree,
    Empty,
}

/// Representation of our terrain with automatic wrap around.
#[derive(Debug)]
struct TerrainMap {
    v: Vec<Vec<Terrain>>,
    /// Width of map.
    width: usize,
}

impl TerrainMap {
    fn new(v: Vec<Vec<Terrain>>) -> TerrainMap {
        let length = v.len();
        let width = v[0].len();
        // Will crash on empty vec. Too bad!
        TerrainMap { v, width: width }
    }

    /// Access map with wrap around.
    pub fn index(&self, x: usize, y: usize) -> Option<Terrain> {
        Some(self.v.get(y)?[x % self.width])
    }
}

fn make_tree(input: &str) -> TerrainMap {
    let mut v = Vec::new();
    for line in input.lines() {
        let entries: Vec<Terrain> = line
            .chars()
            .map(|c| match c {
                '.' => Terrain::Empty,
                '#' => Terrain::Tree,
                c => panic!("Unknown input: {}", c),
            })
            .collect();

        v.push(entries);
    }

    TerrainMap::new(v)
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("./src/day3/input.txt").context("input not found.")?;
    let tm = make_tree(&input);

    let c1 = count_trees(&tm, 3, 1);
    let c2 = count_trees(&tm, 1, 1);
    let c3 = count_trees(&tm, 5, 1);
    let c4 = count_trees(&tm, 7, 1);
    let c5 = count_trees(&tm, 1, 2);
    println!("Trees in path: {}", c1 * c2 * c3 * c4 * c5);
    Ok(())
}

fn count_trees(tm: &TerrainMap, right: usize, down: usize) -> usize {
    let mut trees = 0;
    let mut i = 0;
    let mut j = 0;

    loop {
        match tm.index(i, j) {
            None => return trees,
            Some(Terrain::Tree) => trees += 1,
            _ => {}
        }
        i += right;
        j += down;
    }
    trees
}

#[cfg(test)]
mod tests {
    use crate::{count_trees, make_tree};

    #[test]
    fn part_one_example() {
        let str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
";
        let tm = make_tree(str);
        dbg!(&tm);
        assert_eq!(7, count_trees(&tm, 3, 1));
    }

    #[test]
    fn part_two_example() {
        let str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
";
        let tm = make_tree(str);
        let count1 = count_trees(&tm, 3, 1);
        let count2 = count_trees(&tm, 1, 1);
        let count3 = count_trees(&tm, 5, 1);
        let count4 = count_trees(&tm, 7, 1);
        let count5 = count_trees(&tm, 1, 2);
        assert_eq!(count1 * count2 * count3 * count4 * count5, 336);
    }
}
