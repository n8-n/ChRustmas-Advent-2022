use std::fmt::Display;

/// Grid is a 2-dimensional, row-major ordered array. Column size is fixed, but it can have as many rows as you want.
/// Rows are added with the `add_row` function.
///
/// Column size can be set on instatiation using `with_column_size`. If column size is not set, it will be set as the
/// length of the first row added to the grid.
#[derive(Debug)]
pub struct Grid<T> {
    pub elements: Vec<T>,
    pub columns: usize,
    pub rows: usize,
}

#[allow(dead_code)]
impl<T: Clone> Grid<T> {
    pub fn new() -> Self {
        Grid {
            elements: Vec::<T>::new(),
            columns: 0,
            rows: 0,
        }
    }

    pub fn with_column_size(mut self, columns: usize) -> Self {
        if self.columns == 0 {
            self.columns = columns;
        }

        self
    }

    pub fn add_row(&mut self, row: Vec<T>) {
        let row_len = row.len();

        if self.columns == 0 {
            self.columns = row_len;
        } else if row_len != self.columns {
            eprintln!("Row length does not equal column length of grid.");
            return;
        }

        self.elements.extend(row.clone());
        self.rows += 1;
    }

    pub fn get_row(&self, row: usize) -> Option<Vec<T>> {
        if row > self.rows {
            return None;
        }
        let start = self.columns * row;
        let end = start + self.rows;

        Some(self.elements[start..end].to_vec())
    }

    pub fn get_column(&self, column: usize) -> Option<Vec<T>> {
        if column > self.columns {
            return None;
        }

        let column_vec = self.elements[column..]
            .iter()
            .step_by(self.columns)
            .cloned()
            .collect::<Vec<_>>();

        Some(column_vec)
    }

    // Get grid without first and last rows and columns.
    pub fn get_inner_grid(&self) -> Self {
        let mut new_grid = Grid::<T>::new();
        let start = self.columns;
        let end = self.elements.len() - self.columns;

        self.elements[start..end]
            .chunks(self.columns)
            .for_each(|row| {
                let end_column = row.len() - 1;
                new_grid.add_row(row[1..end_column].to_vec());
            });

        new_grid
    }

    /// Convert a element vector index to a xy coordinate point
    pub fn index_to_point(&self, index: usize) -> Option<Point> {
        if index > self.elements.len() {
            return None;
        }

        let x = index % self.columns;
        let y: usize = (index as f32 / self.rows as f32).floor() as usize;

        Some(Point { x, y })
    }

    pub fn get_element(&self, point: Point) -> Option<&T> {
        if point.x > self.columns || point.y > self.rows {
            return None;
        }

        let index: usize = (self.rows * point.y) + point.x;
        self.elements.get(index)
    }

    pub fn get_adjacent_points(&self, point: Point) -> Vec<Point> {
        let mut points = Vec::<Point>::new();

        if point.x > self.columns || point.y > self.rows {
            return points;
        }

        let is_in_bounds = |p: Point| p.x < self.columns && p.y < self.rows;

        for i in -1..=1 {
            for j in -1..=1 {
                // skip diagonals for now...
                if i == j {
                    continue;
                }

                if (point.x as isize + i < 0) || (point.y as isize + j < 0) {
                    continue;
                }

                let p = Point {
                    x: point.x + i as usize,
                    y: point.y + j as usize,
                };
                if is_in_bounds(p) {
                    points.push(p);
                }
            }
        }

        points
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl<T: Display + std::fmt::Debug + Clone> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.elements
            .iter()
            .enumerate()
            .step_by(self.columns)
            .for_each(|(index, _)| {
                let end = index + self.columns;

                self.elements[index..end].iter().for_each(|e| {
                    write!(f, "{}", e).unwrap();
                });
                writeln!(f).unwrap();
            });

        Ok(())
    }
}

impl<T: PartialEq> PartialEq for Grid<T> {
    fn eq(&self, other: &Self) -> bool {
        self.elements == other.elements
    }
}

//
//
//
#[cfg(test)]
// #[rustfmt::skip]
mod tests {
    use super::*;

    fn get_test_grid() -> Grid<u8> {
        let mut grid = Grid::<u8>::new();
        grid.add_row(vec![0, 0, 1, 5, 4]);
        grid.add_row(vec![1, 3, 1, 7, 4]);
        grid.add_row(vec![8, 7, 1, 10, 4]);
        grid.add_row(vec![99, 2, 1, 12, 4]);
        grid.add_row(vec![9, 20, 61, 2, 7]);
        grid
    }

    #[test]
    fn test_grid_creation() {
        let mut grid = Grid::<u8>::new();
        grid.add_row(vec![0, 0, 1, 4]);
        grid.add_row(vec![1, 3, 1, 4]);

        assert_eq!(2, grid.rows);
        assert_eq!(4, grid.columns);

        grid.add_row(vec![0, 0]);
        assert_eq!(2, grid.rows);

        let mut grid = Grid::<u8>::new().with_column_size(3);

        grid.add_row(vec![0, 0, 1, 4]);
        assert_eq!(0, grid.rows);
    }

    #[test]
    fn test_get_row() {
        let grid = get_test_grid();

        assert_eq!(vec![0, 0, 1, 5, 4], grid.get_row(0).unwrap());
        assert_eq!(vec![9, 20, 61, 2, 7], grid.get_row(4).unwrap());
        assert_eq!(None, grid.get_row(20));
    }

    #[test]
    fn test_get_column() {
        let grid = get_test_grid();
        assert_eq!(vec![0, 1, 8, 99, 9], grid.get_column(0).unwrap());
        assert_eq!(vec![4, 4, 4, 4, 7], grid.get_column(4).unwrap());
        assert_eq!(None, grid.get_column(20));
    }

    #[test]
    fn test_get_inner_grid() {
        let grid = get_test_grid();
        let mut expected_grid = Grid::<u8>::new();
        expected_grid.add_row(vec![3, 1, 7]);
        expected_grid.add_row(vec![7, 1, 10]);
        expected_grid.add_row(vec![2, 1, 12]);

        let inner_grid = grid.get_inner_grid();
        assert_eq!(expected_grid, inner_grid);
    }

    #[test]
    fn test_element_to_point() {
        let grid = get_test_grid();
        assert_eq!(Point { x: 4, y: 0 }, grid.index_to_point(4).unwrap());
        assert_eq!(Point { x: 3, y: 1 }, grid.index_to_point(8).unwrap());
        assert_eq!(Point { x: 0, y: 0 }, grid.index_to_point(0).unwrap());
        assert!(grid.index_to_point(10000).is_none())
    }

    #[test]
    fn test_get_element_from_point() {
        let grid = get_test_grid();
        assert_eq!(0, grid.get_element(Point { x: 0, y: 0 }).unwrap().clone());
        assert_eq!(7, grid.get_element(Point { x: 3, y: 1 }).unwrap().clone());
        assert_eq!(20, grid.get_element(Point { x: 1, y: 4 }).unwrap().clone());
        assert_eq!(7, grid.get_element(Point { x: 4, y: 4 }).unwrap().clone());
        assert_eq!(9, grid.get_element(Point { x: 0, y: 4 }).unwrap().clone());
    }

    #[test]
    fn test_get_adjacent_points() {
        // TODO
    }
}
