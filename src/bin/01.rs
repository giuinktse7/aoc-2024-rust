use std::{collections::HashMap, iter::zip};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut left = vec![];
    let mut right = vec![];

    for line in lines {
        let [a, b]: [u32; 2] = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
            .try_into()
            .expect("Each line must contain exactly two numbers");

        left.push(a);
        right.push(b);
    }

    left.sort();
    right.sort();

    Some(zip(left, right).map(|(lhs, rhs)| lhs.abs_diff(rhs)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();

    // Low 16 bits is count, high 16 bits is factor
    let mut counter: HashMap<u32, u32> = HashMap::new();

    for line in lines {
        let [a, b]: [u32; 2] = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
            .try_into()
            .unwrap();

        counter.entry(a).and_modify(|e| *e += 1 << 16).or_insert(1 << 16);
        counter.entry(b).and_modify(|e| *e += 1).or_insert(1);
    }

    let sum: u32 = counter
        .iter()
        .map(|(&k, &val)| {
            let low = val & 0xFFFF;
            let high = val >> 16;
            low * high * k
        })
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
