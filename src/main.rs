struct Grid([[char; 100]; 100]);

impl Copy for Grid {}

impl Clone for Grid {
    fn clone(&self) -> Grid {
        *self
    }
}

impl Grid {
    fn new(input: &str) -> Grid {
        assert_eq!(input.lines().count(), 100);
        assert_eq!(100, input.lines().next().unwrap_or("").len());
        fn initiate(grid: &mut Grid, input: &str) {
            input.lines()
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
        if input.len() != 0 {
            initiate(&mut grid, input);
        }
        grid
    }

    fn set(&mut self, (i, j): (usize, usize), c: char) {
        assert!(['#', '.'].contains(&c));
        assert!(i < self.0.len());
        assert!(j < self.0.len());
        self.0[i][j] = c;
    }
}

trait Light {
    fn step(&mut self, stuck_corners: bool);
    fn brightness(&self) -> usize;
}

impl Light for Grid {
    fn brightness(&self) -> usize {
        self.0
            .iter()
            .map(|l| l.iter().filter(|&&c| c == '#').count())
            .sum()
    }

    fn step(&mut self, stuck_corners: bool) {
        let mut new_grid = self.clone();
        let nrows = self.0.len();
        let ncols = nrows;
        for (i, row) in self.0.iter().enumerate() {
            let next_i = if i != nrows - 1 { Some(i + 1) } else { None };
            let prev_i = if i != 0 { Some(i - 1) } else { None };
            for (j, light) in row.iter().enumerate() {
                let next_j = if j != ncols - 1 { Some(j + 1) } else { None };
                let prev_j = if j != 0 { Some(j - 1) } else { None };
                let lights_on = [(prev_i, prev_j),
                                 (prev_i, Some(j)),
                                 (prev_i, next_j),
                                 (Some(i), prev_j),
                                 (Some(i), next_j),
                                 (next_i, prev_j),
                                 (next_i, Some(j)),
                                 (next_i, next_j)]
                    .iter()
                    .filter(|&&(i, j)| {
                        i.is_some() && j.is_some() && self.0[i.unwrap()][j.unwrap()] == '#'
                    })
                    .count();
                match *light {
                    '#' if ![2, 3].contains(&lights_on) => new_grid.0[i][j] = '.',
                    '.' if lights_on == 3 => new_grid.0[i][j] = '#',
                    _ => (),
                }
            }
        }
        if stuck_corners {
            new_grid.0[0][0] = '#';
            new_grid.0[0][ncols - 1] = '#';
            new_grid.0[nrows - 1][0] = '#';
            new_grid.0[nrows - 1][ncols - 1] = '#';
        }
        *self = new_grid;
    }
}

fn main() {
    let mut grid = Grid::new(include_str!("../input.txt"));
    for _ in 1..101 {
        grid.step(false);
    }
    println!("No of ON lighs: {}", grid.brightness());

    let mut grid = Grid::new(include_str!("../input.txt"));
    grid.set((0, 0), '#');
    grid.set((0, 99), '#');
    grid.set((99, 0), '#');
    grid.set((99, 99), '#');
    for _ in 1..101 {
        grid.step(true);
    }
    println!("No of ON lighs with stuck corner: {}", grid.brightness());
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

    #[test]
    fn getting_brightness() {
        let g = Grid::new(include_str!("../input.txt"));
        assert_eq!(5076, g.brightness());
    }
}
