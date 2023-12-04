use std::collections::HashSet;

advent_of_code::solution!(3);

type Part = (usize, usize, usize, usize);

fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}
fn part_from(lines: &[Vec<char>], i: usize, j: usize) -> Option<Part> {
    if !lines[i][j].is_ascii_digit() {
        return None;
    }

    let mut start = j;
    while start > 0 && lines[i][start - 1].is_ascii_digit() {
        start -= 1;
    }

    let mut end = j;
    while end < lines[i].len() - 1 && lines[i][end + 1].is_ascii_digit() {
        end += 1;
    }

    let number: String = lines[i][start..end + 1].iter().collect();
    let number = number.parse().ok()?;
    Some((i, start, end, number))
}

fn in_bounds(lines: &[Vec<char>], i: usize, j: usize) -> bool {
    lines.get(i).and_then(|l| l.get(j)).is_some()
}

pub fn part_one(input: &str) -> Option<u32> {
    let data: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut parts: HashSet<Part> = HashSet::new();
    for (i, line) in data.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if !is_symbol(*char) {
                continue;
            }
            let neighbors = [
                (i - 1, j - 1),
                (i - 1, j),
                (i - 1, j + 1),
                (i, j - 1),
                (i, j + 1),
                (i + 1, j - 1),
                (i + 1, j),
                (i + 1, j + 1),
            ];
            let adjacent: HashSet<Part> = neighbors
                .iter()
                .filter(|(i, j)| in_bounds(&data, *i, *j))
                .flat_map(|(i, j)| part_from(&data, *i, *j))
                .collect();
            parts.extend(&adjacent);
        }
    }
    Some(parts.iter().map(|p| p.3 as u32).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let data: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut parts: HashSet<Part> = HashSet::new();
    let mut gears: Vec<u32> = Vec::new();
    for (i, line) in data.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if !is_symbol(*char) {
                continue;
            }
            let neighbors = [
                (i - 1, j - 1),
                (i - 1, j),
                (i - 1, j + 1),
                (i, j - 1),
                (i, j + 1),
                (i + 1, j - 1),
                (i + 1, j),
                (i + 1, j + 1),
            ];
            let adjacent: HashSet<Part> = neighbors
                .iter()
                .filter(|(i, j)| in_bounds(&data, *i, *j))
                .flat_map(|(i, j)| part_from(&data, *i, *j))
                .collect();
            parts.extend(&adjacent);

            if *char == '*' && adjacent.len() == 2 {
                gears.push(adjacent.iter().map(|p| p.3 as u32).product())
            }
        }
    }
    Some(gears.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
