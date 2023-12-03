use std::cmp::max;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let data: Vec<Vec<&str>> = input
        .lines()
        .map(|s| s.split(&[':', ';', ','][..]).collect::<Vec<&str>>())
        .collect();

    let mut sum = 0;
    for game in data {
        let game_id: u32 = game[0]
            .strip_prefix("Game ")?
            .parse::<u32>()
            .expect("Game ID");
        let mut possible = true;
        for g in &game[1..] {
            let amount_color = &g.trim().split(" ").collect::<Vec<&str>>();
            let amount = amount_color[0].parse::<i32>().expect("Not an integer");
            possible = match &amount_color[1].chars().nth(0).unwrap() {
                'b' => amount <= 14,
                'g' => amount <= 13,
                'r' => amount <= 12,
                _ => panic!("Unknown color"),
            };
            if !possible {
                break;
            }
        }
        if possible {
            sum += game_id;
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data: Vec<Vec<&str>> = input
        .lines()
        .map(|s| s.split(&[':', ';', ','][..]).collect::<Vec<&str>>())
        .collect();

    let mut sum = 0;
    for game in data {
        let mut green = -1;
        let mut red = -1;
        let mut blue = -1;
        for g in &game[1..] {
            let amount_color = &g.trim().split(" ").collect::<Vec<&str>>();
            let amount = amount_color[0].parse::<i32>().expect("Not an integer");
            match &amount_color[1].chars().nth(0).unwrap() {
                'b' => blue = max(amount, blue),
                'g' => green = max(amount, green),
                'r' => red = max(amount, red),
                _ => panic!("Unknown color"),
            };
        }
        sum += green * red * blue
    }
    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
