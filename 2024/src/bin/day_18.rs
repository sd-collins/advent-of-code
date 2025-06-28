use std::collections::HashSet;

use util::PerfTimer;

const MAX: u32 = 70;

fn input() -> Vec<(u32, u32)> {
    util::get_day_input(18)
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

fn get_min_path(corrupted: &HashSet<(u32, u32)>) -> Option<usize> {
    let mut visited: HashSet<(u32, u32)> = HashSet::from([(0, 0)]);
    let mut prev: Vec<(u32, u32)> = vec![(0, 0)];
    let mut step = 1;
    while !prev.is_empty() {
        let mut next = Vec::new();
        for (x, y) in prev {
            for (new_x, new_y) in [
                (x.saturating_sub(1), y),
                ((x + 1).min(MAX), y),
                (x, y.saturating_sub(1)),
                (x, (y + 1).min(MAX)),
            ] {
                if (new_x, new_y) == (MAX, MAX) {
                    return Some(step);
                }
                if !corrupted.contains(&(new_x, new_y)) && visited.insert((new_x, new_y)) {
                    next.push((new_x, new_y));
                }
            }
        }
        prev = next;
        step += 1;
    }
    None
}

/// Finds the input for which the predicate first returns true.
/// The predicate must be monotonic, i.e. it must return true for all inputs greater
/// than the first input for which it returns true.
fn bisect(min: usize, max: usize, predicate: impl Fn(usize) -> bool) -> usize {
    let mut low = min;
    let mut high = max;
    while low < high {
        let mid = low.midpoint(high);
        if predicate(mid) {
            high = mid;
        } else {
            low = mid + 1;
        }
    }
    assert!(low == high);
    low
}

fn main() {
    let falling_bytes = input();
    {
        let _timer = PerfTimer::new("Part 1");
        let part_1 = get_min_path(&falling_bytes.iter().copied().take(1024).collect()).unwrap();
        println!("Part 1: {part_1}");
    }
    {
        let _timer = PerfTimer::new("Part 2");
        let count = bisect(1024, falling_bytes.len(), |n| {
            get_min_path(&falling_bytes.iter().copied().take(n).collect()).is_none()
        });
        let (x, y) = falling_bytes[count - 1];
        println!("Part 2: {x},{y}");
    }
}
