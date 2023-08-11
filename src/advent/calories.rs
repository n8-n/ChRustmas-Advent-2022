use super::io::*;

pub fn highest_total_calories_from_file(filename: &str) -> Vec<i32> {
    let lines = read_file_as_vector(filename);

    let mut elf_calories = Vec::<i32>::new();
    let mut cal_add = 0;

    for l in lines {
        if l.is_empty() {
            elf_calories.push(cal_add);
            cal_add = 0;
        } else {
            let cal: i32 = l.trim().parse().expect("Cannot parse line as number");
            cal_add += cal;
        }
    }
    elf_calories.push(cal_add);

    elf_calories.sort();

    elf_calories
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calories_file_read_and_sum() {
        let calories = highest_total_calories_from_file("resources/test/calories_test.txt");

        assert_eq!(5, calories.len());
        let expected_values: Vec<i32> = vec![4000, 6000, 10000, 11000, 24000];
        assert_eq!(expected_values, calories);
    }
}
