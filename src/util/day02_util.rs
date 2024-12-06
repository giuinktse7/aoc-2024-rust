#[derive(Clone, Default)]
pub struct Report {
    pub xs: Vec<i32>,
    pub removed: i32,
    pub removed_index: usize,
    pub sgn: i32,
    pub fail_index: usize,
    pub remove_count: usize,
}

pub fn is_safe_step(sgn: i32, v1: i32, v2: i32) -> bool {
    let abs = (v2 - v1).abs();
    (v2 - v1).signum() == sgn && 1 <= abs && abs <= 3
}

impl Report {
    fn next_valid_unsafe(&self, i: usize) -> usize {
        if i == self.removed_index {
            i + 1
        } else {
            i
        }
    }

    pub fn len(&self) -> usize {
        self.xs.len()
    }

    pub fn slope_2(&self, i1: usize, i2: usize) -> i32 {
        let k1 = self.next_valid_unsafe(i1);
        let k2 = self.next_valid_unsafe(i2);
        (self.xs[k2] - self.xs[k1]).signum()
    }

    pub fn slope(&self, i1: usize) -> i32 {
        let k1 = self.next_valid_unsafe(i1);
        match k1.checked_add(1) {
            Some(k2) => (self.xs[k2] - self.xs[k1]).signum(),
            None => self.sgn,
        }
    }

    pub fn safe(&self, i: usize) -> bool {
        self.safe_with_sgn(self.sgn, i)
    }

    pub fn safe_with_sgn(&self, sgn: i32, i: usize) -> bool {
        if i == self.len() - 1 {
            return true;
        }

        let (i1, i2) = match i {
            0 if self.removed_index == 0 => (i + 1, i + 2),
            _ if self.removed_index == i => (i - 1, i + 1),
            _ if self.removed_index == i + 1 => (i, i + 2),
            _ => (i, i + 1),
        };

        is_safe_step(sgn, self.xs[i1], self.xs[i2])
    }

    pub fn safe_2(&self, i1: usize, i2: usize) -> bool {
        let slope = self.slope_2(i1, i2);
        let abs = (self.xs[i1] - self.xs[i2]).abs();
        slope == self.sgn && 1 <= abs && abs <= 3
    }

    pub fn remove(&mut self, i: usize) -> bool {
        let new_sgn = match i {
            0 => (self.xs[2] - self.xs[1]).signum(),
            1 => (self.xs[2] - self.xs[0]).signum(),
            _ => (self.xs[1] - self.xs[0]).signum(),
        };

        // It is always okay to undo a removal of the first element, IF it is compatible with its next element
        match self.removed_index {
            0 => {
                let k = if i == 1 { 2 } else { 1 };
                if !is_safe_step(new_sgn, self.xs[0], self.xs[k]) {
                    return false;
                }
            }
            _ => {
                // A remove can only be moved one step to the right (except if the currently removed element is the first one)
                if i != self.removed_index + 1 {
                    return false;
                }
            }
        }

        let last_index = self.len() - 1;

        let can_remove = match i {
            _ if i == last_index => is_safe_step(new_sgn, self.xs[i - 2], self.xs[i - 1]),
            1 => is_safe_step(new_sgn, self.xs[0], self.xs[2]),
            _ => {
                is_safe_step(new_sgn, self.xs[i - 1], self.xs[i + 1])
                    && is_safe_step(new_sgn, self.xs[i - 2], self.xs[i - 1])
            }
        };

        if !can_remove {
            return false;
        }

        self.removed_index = i;
        self.removed = self.xs[i];

        self.remove_count += 1;
        self.sgn = new_sgn;

        true
    }
}

pub fn show_levels(levels: &Vec<i32>, separator: &str) -> String {
    levels
        .iter()
        .map(|level| level.to_string())
        .collect::<Vec<String>>()
        .join(format!("{} ", separator).as_str())
}

pub fn print_result(s: &Report, levels: &Vec<i32>, is_safe: bool) {
    let levels_str = show_levels(levels, "");
    print!("{} ", levels_str);

    // for (i, level) in levels.iter().enumerate() {
    //     if !is_safe && i == s.removed_index && i == s.fail_index {
    //         print!("[{}] ! ", level);
    //     } else if i == s.removed_index {
    //         print!("[{}], ", level);
    //     } else if !is_safe && i == s.fail_index {
    //         print!("{} ! ", level);
    //     } else {
    //         print!("{}, ", level);
    //     }
    // }

    if is_safe {
        print!("[SAFE]");
    } else {
        print!("[UNSAFE]");
    }

    println!();

    print!(
        "\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t{}\n",
        levels_str
    );

    println!("----------------------------------------");
}
