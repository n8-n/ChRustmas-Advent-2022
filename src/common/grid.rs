use std::fmt::Display;

pub struct Grid<T> {
    elements: Vec<T>,
    columns: usize,
    rows: usize,
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
        self.columns = columns;
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

        println!("start:{start}, end:{end}");

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
}

impl<T: Display + std::fmt::Debug + Clone> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.elements
            .iter()
            .enumerate()
            .step_by(self.columns)
            .for_each(|(index, _)| {
                let end = index + self.rows;

                self.elements[index..end].iter().for_each(|e| {
                    write!(f, "{} ", e).unwrap();
                });
                write!(f, "\n").unwrap();
            });
        
        Ok(())
    }
}

//
//
//
#[cfg(test)]
// #[rustfmt::skip]
mod tests {
    use super::*;

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
        let mut grid = Grid::<u8>::new();
        let row0 = vec![0, 0, 1, 4];
        let row3 = vec![99, 2, 1, 4];
        grid.add_row(row0.clone());
        grid.add_row(vec![1, 3, 1, 4]);
        grid.add_row(vec![8, 7, 1, 4]);
        grid.add_row(row3.clone());

        assert_eq!(row0, grid.get_row(0).unwrap());
        assert_eq!(row3, grid.get_row(3).unwrap());
        assert_eq!(None, grid.get_row(20));
    }

    #[test]
    fn test_get_column() {
        let mut grid = Grid::<u8>::new();
        grid.add_row(vec![0, 0, 1, 4]);
        grid.add_row(vec![1, 3, 1, 4]);
        grid.add_row(vec![8, 7, 1, 4]);
        grid.add_row(vec![99, 2, 1, 4]);

        println!("{}", grid);

        assert_eq!(vec![0, 1, 8, 99], grid.get_column(0).unwrap());
        assert_eq!(vec![4, 4, 4, 4], grid.get_column(3).unwrap());
        assert_eq!(None, grid.get_column(20));
    }
}
