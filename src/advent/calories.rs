use crate::util::io;

pub fn highest_total_calories_from_file(filename: &str) -> Vec<u32> {
    let lines = io::read_file_as_vector(filename).expect("Could not read file");

    let mut elf_calories = Vec::<u32>::new();
    let mut cal_add: u32 = 0;

    for l in lines {
        if l.is_empty() {
            elf_calories.push(cal_add);
            cal_add = 0;
        } else {
            let cal: u32 = l.trim().parse().expect("Cannot parse line as number");
            cal_add += cal;
        }
    }
    elf_calories.push(cal_add);

    elf_calories.sort();

    elf_calories
}

//
//
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calories_file_read_and_sum() {
        let calories = highest_total_calories_from_file("resources/test/01_calories.txt");

        assert_eq!(5, calories.len());
        let expected_values: Vec<u32> = vec![4000, 6000, 10000, 11000, 24000];
        assert_eq!(expected_values, calories);
    }
}
