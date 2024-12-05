use core::panic;
use std::{cmp::min, hint::black_box, mem::swap};

advent_of_code::solution!(2);

fn is_safe_level_transition(sgn: i32, a: &i32, b: &i32) -> bool {
    let diff = b - a;
    let abs = diff.abs();

    diff.signum() == sgn && 1 <= abs && abs <= 3
}

// 7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
// 1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
// 9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
// 1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
// 8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
// 1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.
// So, in this example, *2* reports are safe.
// Safe :=
// - The levels are either all increasing or all decreasing.
// - Any two adjacent levels differ by at least one and at most three.
pub fn part_one(input: &str) -> Option<usize> {
    // How many reports are safe?
    let n_safe_reports = input
        .lines()
        .into_iter()
        .filter(|line| {
            let levels: Vec<i32> = line
                .split_whitespace()
                .map(|level| level.parse::<i32>().unwrap())
                .collect();

            // println!("\n{:?}", levels);

            let sgn = (levels[1] - levels[0]).signum();

            levels.windows(2).all(|w| match w {
                [a, b] => is_safe_level_transition(sgn, a, b),
                _ => false,
            })
        })
        .count();

    Some(n_safe_reports)
}

fn show_levels(levels: &Vec<i32>, separator: &str) -> String {
    levels
        .iter()
        .map(|level| level.to_string())
        .collect::<Vec<String>>()
        .join(format!("{} ", separator).as_str())
}

fn print_result(s: &State, levels: &Vec<i32>, is_safe: bool) {
    let levels_str = show_levels(levels, "");
    // print!("{} ", levels_str);

    for (i, level) in levels.iter().enumerate() {
        if !is_safe && i == s.removed_index && i == s.fail_index {
            print!("[{}] ! ", level);
        } else if i == s.removed_index {
            print!("[{}], ", level);
        } else if !is_safe && i == s.fail_index {
            print!("{} ! ", level);
        } else {
            print!("{}, ", level);
        }
    }

    if is_safe {
        print!("[SAFE]");
    } else {
        print!("[UNSAFE]");
    }

    println!();

    // print!(
    //     "\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t{}\n",
    //     levels_str
    // );

    println!("----------------------------------------");
}

#[derive(Clone)]
struct State {
    removed: i32,
    removed_index: usize,
    sgn: i32,
    fail_index: usize,
    remove_count: usize,
}

fn remove(s: &mut State, levels: &[i32], i: usize) -> bool {
    let new_sgn = match i {
        0 => (levels[2] - levels[1]).signum(),
        1 => (levels[2] - levels[0]).signum(),
        _ => (levels[1] - levels[0]).signum(),
    };
    // It is always okay to undo a removal of the first element, IF it is compatible with its next element
    if s.removed_index == 0 {
        let k = if i == 1 { 2 } else { 1 };
        // let sgn = (levels[k] - levels[0]).signum();
        if !is_safe_level_transition(new_sgn, &levels[0], &levels[k]) {
            // Can't undo the removal of the first element
            return false;
        }
    } else {
        if i != s.removed_index + 1 {
            return false;
        }

        if s.remove_count > 1 {
            return false;
        }
    }

    let can_remove = match [
        levels.get(i.wrapping_sub(2)).copied(),
        levels.get(i.wrapping_sub(1)).copied(),
        levels.get(i).copied(),
        levels.get(i + 1).copied(),
    ] {
        [Some(p2), Some(p1), Some(_), Some(n)] => {
            // p2 [p1] x n --> p2 p1 [x] n
            // p2 -> p1 must be safe
            // p1 -> n must be safe
            is_safe_level_transition(new_sgn, &p2, &p1)
                && is_safe_level_transition(new_sgn, &p1, &n)
        }
        [None, Some(p1), Some(_), Some(n)] => {
            // [p1] x n --> p1 [x] n
            // p1 -> n must be safe
            is_safe_level_transition(new_sgn, &p1, &n)
        }

        [Some(p2), Some(p1), Some(_), None] => {
            // p2 [p1] x --> p2 p1 [x]
            // p2 -> p1 must be safe
            is_safe_level_transition(new_sgn, &p2, &p1)

            // We are fine here
        }

        x => panic!("Invalid state: {:?}", x),
    };

    if !can_remove {
        return false;
    }

    s.removed_index = i;
    s.removed = levels[i];

    s.remove_count += 1;
    s.sgn = new_sgn;

    true
}

fn crazy_fn(a: Option<&i32>, b: &i32, c: Option<&i32>) {}

fn keep(a: i32) {}

fn get_sgn(s: &State, levels: &[i32], deleted_index: usize) -> i32 {
    if deleted_index == 0 {
        return (levels[2] - levels[1]).signum();
    } else if deleted_index == 1 {
        return (levels[2] - levels[0]).signum();
    } else {
        return s.sgn;
    }
}

fn combinations_n_minus_one<T: Clone>(vec: &[T]) -> impl Iterator<Item = Vec<T>> + '_ {
    vec.iter().enumerate().map(move |(i, _)| {
        vec.iter()
            .enumerate()
            .filter_map(|(j, item)| if i != j { Some(item.clone()) } else { None })
            .collect()
    })
}

fn check_if_safe_naive(levels: &[i32]) -> bool {
    combinations_n_minus_one(levels).any(|xs| {
        let sgn = (xs[1] - xs[0]).signum();

        xs.windows(2).all(|w| match w {
            [a, b] => is_safe_level_transition(sgn, a, b),
            _ => false,
        })
    })
}

fn check_if_safe(s: &mut State, levels: &[i32]) -> bool {
    let groups: Vec<(Option<&i32>, Option<&i32>, &i32, Option<&i32>)> = levels
        .iter()
        .enumerate()
        .map(|(i, curr)| {
            let prev2 = i.checked_sub(2).map(|i| &levels[i]);
            let prev = i.checked_sub(1).map(|i| &levels[i]);
            let next = levels.get(i + 1);
            (prev2, prev, curr, next)
        })
        .collect();

    let last_index = levels.len() - 1;

    let mut skip_index = levels.len();

    let mut i = 1;
    while i < groups.len() {
        let group = &groups[i];

        if i == skip_index {
            // println!("Skipping index: {}", i);
            i += 1;
            continue;
        }
        // If we are at the second last element and we haven't removed anything, we can just skip the last element.
        if i == last_index - 1 && s.removed_index == 0 {
            // Just for debugging
            if !is_safe_level_transition(s.sgn, group.2, group.3.unwrap()) {
                if !remove(s, levels, i + 1) {
                    s.fail_index = i;
                    return false;
                }
            }
            // End just for debugging
            return true;
        }

        let is_safe = match group {
            (None, None, x0, Some(next)) => is_safe_level_transition(s.sgn, x0, next),
            (None, Some(_), x, Some(next)) => is_safe_level_transition(s.sgn, x, next),
            (Some(_), Some(_), x, Some(next)) => is_safe_level_transition(s.sgn, x, next),
            (Some(prev2), Some(prev1), x, None) => {
                let prev_was_removed = s.removed_index == i - 1;
                let prev = if prev_was_removed { prev2 } else { prev1 };
                // If prev was deleted, we must compare against prev2
                is_safe_level_transition(s.sgn, &prev, x)
            }
            _ => panic!("Invalid group"),
        };

        let _prev = group.1;
        let _curr = group.2;
        let _next = group.3;
        black_box(crazy_fn(_prev, _curr, _next));

        if !is_safe {
            // Delete the one before us if possible and try again
            let sgn = get_sgn(s, levels, i - 1);
            let hm = _next
                .map(|n| is_safe_level_transition(sgn, _curr, n))
                .unwrap_or(false);
            if i != 1 && hm && remove(s, levels, i - 1) {
                let k = 2;
                black_box(keep(k));

                // No index increment here
                continue;
            }

            let mut _s = s.clone();
            let removed = remove(s, levels, i);
            if !removed {
                if s.removed_index != 0 {
                    s.fail_index = i;
                    return false;
                }

                // If remove failed but we haven't removed anything yet, then try to remove the next element.
                swap(s, &mut _s);
                if s.removed_index == 0 {
                    if !remove(s, levels, i + 1) {
                        s.fail_index = i + 1;
                        // Just for debugging
                        s.removed_index = i + 1;
                        s.removed = levels[i + 1];
                        // End just for debugging
                        return false;
                    }
                    skip_index = i + 1;
                }
            }
        }

        black_box(crazy_fn(_prev, _curr, _next));
        i += 1;
    }

    true
}

fn check_heuristics(s: &mut State, levels: &Vec<i32>) -> bool {
    // Heuristic 0:
    // If none of the next two are within 3, then we are busted
    {
        let failed = levels.windows(3).any(|w| {
            if let &[a, b, c] = w {
                min((a - b).abs(), (a - c).abs()) > 3 && (b - c).abs() > 3
            } else {
                false
            }
        });
        if failed {
            // println!("Failed heuristic (0):\t\t{}", show_levels(&levels, " "));
            return false;
        }
    }

    // Heuristic 1:
    {
        // The first three lock the sign
        for i in 0..3 {
            if is_safe_level_transition(s.sgn, &levels[i], &levels[i + 1]) {
                continue;
            }

            // println!(
            //     "Removing: {}, ({}, {}, {}) {}",
            //     i,
            //     levels[i],
            //     levels[i + 1],
            //     levels[i + 2],
            //     s.sgn
            // );
            remove(s, &levels, i);
        }

        let locked_sgn = s.sgn;
        let incorrect_sgn = levels
            .windows(2)
            .skip(3)
            .filter(|x| {
                if let &[a, b] = x {
                    let bad = (b - a).signum() != locked_sgn;
                    if bad {
                        // println!("Bad: {} {}", a, b);
                    }
                    bad
                } else {
                    false
                }
            })
            .count();

        if incorrect_sgn > 0 {
            // println!(
            //     "Failed heuristic (1, > 0):\t\t{}",
            //     show_levels(&levels, " ")
            // );
            return false;
        }
    }

    return true;
}

pub fn part_two(input: &str) -> Option<usize> {
    let s_zero = State {
        removed: 0,
        removed_index: 0,
        sgn: 0,
        fail_index: 0,
        remove_count: 0,
    };

    let n_safe_reports = input
        .lines()
        .into_iter()
        .filter(|line| {
            let levels: Vec<i32> = line
                .split_whitespace()
                .map(|level| level.parse::<i32>().unwrap())
                .collect();

            let mut s = s_zero.clone();

            s.sgn = (levels[2] - levels[1]).signum();

            s.removed_index = 0;
            s.remove_count = 0;

            // Assume the first value is deleted
            // That delete can be overridden without exhausting the shift rule

            let mut temp_s = s.clone();
            // let heuristic_safe = check_heuristics(&mut temp_s, &levels);

            // if !heuristic_safe {
            //     return false;
            // }

            let is_safe = check_if_safe(&mut s, levels.as_ref());
            // let is_safe = check_if_safe_naive(levels.as_ref());

            // If heuristic wants to remove but we don't, log it
            // if !heuristic_safe && is_safe {
            //     println!(
            //         "Mismatch: [heuristic_safe: {}] != [is_safe: {}]",
            //         heuristic_safe, is_safe
            //     );
            // }
            print_result(&s, &levels, is_safe);

            is_safe
        })
        .count();

    // > 534
    // < 543
    // 537 is wrong
    // 535 is wrong
    Some(n_safe_reports)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("single", DAY));
        assert_eq!(result, Some(4));
    }
}
