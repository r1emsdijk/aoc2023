advent_of_code::solution!(1);

fn parse_digits(t_num: &str) -> Vec<u32> {
    t_num.chars().filter_map(|a| a.to_digit(10)).collect()
}
pub fn part_one(input: &str) -> Option<u32> {
    let data: Vec<Vec<u32>> = input.split("\n").map(|s| parse_digits(s)).collect();
    let result = data.into_iter().map(|a| match a.len() {
        2.. => Vec::from([a[0], a[a.len() - 1]])
            .iter()
            .fold(0, |acc, elem| acc * 10 + *elem),
        1 => Vec::from([a[0], a[0]])
            .iter()
            .fold(0, |acc, elem| acc * 10 + *elem),
        0 => 0,
    });

    Some(result.sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = input
        .split("\n")
        .map(|line| {
            line.to_string()
                .replace("zero", "zero0zero")
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|v| {
            if v.len() < 1 {
                return 0;
            }
            10 * v[0] + v[v.len() - 1]
        })
        .sum();
    Some(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }
}
