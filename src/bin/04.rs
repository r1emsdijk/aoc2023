use std::collections::{HashMap, HashSet};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let answer: u32 = input
        .lines()
        .map(|s| s.split(&[':', '|'][..]).collect::<Vec<&str>>())
        .map(|card| {
            let winning_set: HashSet<i32> = HashSet::from_iter(
                card[1][1..]
                    .split(" ")
                    .filter(|s| s.len() > 0)
                    .map(|c| c.parse::<i32>().expect("not a integer")),
            );
            let ours = HashSet::from_iter(
                card[2][1..]
                    .split(" ")
                    .filter(|s| s.len() > 0)
                    .map(|c| c.parse::<i32>().expect("integer")),
            );
            let count = winning_set.intersection(&ours).count() as u32;
            if count >= 1 {
                return u32::pow(2, (winning_set.intersection(&ours).count() as u32) - 1);
            }
            0
        })
        .sum();
    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map: HashMap<usize, i32> = HashMap::new();
    input
        .lines()
        .map(|s| s.split(&[':', '|'][..]).collect::<Vec<&str>>())
        .enumerate()
        .for_each(|(i, card)| {
            *map.entry(i).or_insert(0) += 1;
            let winning_set: HashSet<i32> = HashSet::from_iter(
                card[1][1..]
                    .split(" ")
                    .filter(|s| s.len() > 0)
                    .map(|c| c.parse::<i32>().expect("not a integer")),
            );
            let ours = HashSet::from_iter(
                card[2][1..]
                    .split(" ")
                    .filter(|s| s.len() > 0)
                    .map(|c| c.parse::<i32>().expect("integer")),
            );
            let count = winning_set.intersection(&ours).count() as u32;
            for j in 0..count {
                *map.entry(i + 1 + (j as usize)).or_insert(0) += *map.get(&i).expect("value");
            }
        });
    let answer: i32 = map.values().sum();
    Some(answer as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
