struct Grid([[char; 100]; 100]);

impl Grid {
    fn new(inp: &str) -> Grid {
        fn initiate(grid: &mut Grid, inp: &str) {
            inp.lines()
                .enumerate()
                .map(|(x, line): (usize, &str)| {
                    line.chars()
                        .enumerate()
                        .filter(|&(_, s)| s == '#')
                        .map(|(y, s): (usize, char)| grid.0[x][y] = s)
                        .count();
                })
                .count();
        }
        let mut grid = Grid([['.'; 100]; 100]);

        // If input string is empty, return a blank grid with default value.
        if inp.len() != 0 {
            initiate(&mut grid, inp);
        }
        grid
    }
}

pub trait Light {
    fn switch(&mut self);
    fn brightness(&self) -> usize;
}

impl Light for Grid {
    fn brightness(&self) -> usize {
        self.0
            .iter()
            .map(|l| l.iter().filter(|&&c| c == '#').count())
            .fold(0, |accum, c| accum + c)
        // self.0
        //     .iter()
        //     .map(|l| l.iter().map(|&c| if c == '#' { 1 } else { 0 }).sum::<usize>())
        //     .sum()
    }

    fn switch(&mut self) {
        unimplemented!();
    }
}
fn main() {
    let grid = Grid::new(include_str!("../input.txt"));

    for (_, row) in grid.0.iter().enumerate() {
        for col in row.iter() {
            print!("{}", col);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initiate_grid() {
        let g = Grid::new(include_str!("../input.txt"));
        let expected = "..####.####.##.#.##....#...##....#..#....#..###..#...#..###.#####.....\
                        #####..##.#.#.#.#.#.##.####...";
        let expected = expected.chars().collect::<Vec<char>>();
        assert_eq!(expected[..], g.0[54][..]);

        let expected = "##.#.##.#...#.###.##.##.##.##..##.##...#..##.#..#######.#..#...#.#.##..#..\
                        ..##.#..####.###........#.";
        let expected = expected.chars().collect::<Vec<char>>();
        assert_eq!(expected[..], g.0[90][..]);
    }
}