advent_of_code::solution!(2);
use advent_of_code::util::day02_util::Report;

pub fn part_one(input: &str) -> Option<usize> {
    let n_safe_reports = input
        .lines()
        .into_iter()
        .filter(|line| {
            let levels: Vec<i32> = line
                .split_whitespace()
                .map(|level| level.parse::<i32>().unwrap())
                .collect();

            let sgn = (levels[1] - levels[0]).signum();

            levels.windows(2).all(|w| match w {
                [a, b] => {
                    let diff = b - a;
                    let abs = diff.abs();

                    diff.signum() == sgn && 1 <= abs && abs <= 3
                }
                _ => false,
            })
        })
        .count();

    Some(n_safe_reports)
}

fn get_sgn(s: &Report, levels: &[i32], deleted_index: usize) -> i32 {
    if deleted_index == 0 {
        return (levels[2] - levels[1]).signum();
    } else if deleted_index == 1 {
        return (levels[2] - levels[0]).signum();
    } else {
        return s.sgn;
    }
}

// fn combinations_n_minus_one<T: Clone>(vec: &[T]) -> impl Iterator<Item = Vec<T>> + '_ {
//     vec.iter().enumerate().map(move |(i, _)| {
//         vec.iter()
//             .enumerate()
//             .filter_map(|(j, item)| if i != j { Some(item.clone()) } else { None })
//             .collect()
//     })
// }

// fn check_if_safe_naive(levels: &[i32]) -> bool {
//     combinations_n_minus_one(levels).any(|xs| {
//         let sgn = (xs[1] - xs[0]).signum();

//         xs.windows(2).all(|w| match w {
//             [a, b] => {
//                 let diff = b - a;
//                 let abs = diff.abs();

//                 diff.signum() == sgn && 1 <= abs && abs <= 3
//             }
//             _ => false,
//         })
//     })
// }

fn check_report(report: &mut Report) -> bool {
    let size = report.len();
    for i in 1..size {
        if report.removed_index == i {
            continue;
        }

        // If we are at the second last element and we haven't removed anything, we can just skip the last element.
        if i == size - 1 && report.removed_index == 0 {
            // Just for debugging
            // if !report.safe(i) {
            //     if !report.remove(i + 1) {
            //         report.fail_index = i;
            //         return false;
            //     }
            // }
            // End just for debugging
            return true;
        }

        if !report.safe(i) {
            // Delete the one before us if possible and try again
            let sgn_if_prev_removed = get_sgn(report, &report.xs, i - 1);
            let can_try_remove_prev = report.safe_with_sgn(sgn_if_prev_removed, i);
            if i != 1 && can_try_remove_prev && report.remove(i - 1) {
                continue;
            }

            if !(report.remove(i) || report.remove(i + 1)) {
                report.fail_index = i + 1;
                // Just for debugging
                // report.removed_index = i + 1;
                // report.removed = report.xs[i + 1];
                // End just for debugging
                return false;
            }
        }
    }

    true
}

pub fn part_two(input: &str) -> Option<usize> {
    let n_safe_reports = input
        .lines()
        .into_iter()
        .filter(|line| {
            let levels: Vec<i32> = line
                .split_whitespace()
                .map(|level| level.parse::<i32>().unwrap())
                .collect();

            let mut report = Report::default();

            report.xs = levels;
            report.sgn = report.slope(1);

            check_report(&mut report)
        })
        .count();

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
