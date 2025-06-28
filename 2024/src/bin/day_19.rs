use itertools::Itertools;
use util::PerfTimer;

fn input() -> (Vec<String>, Vec<String>) {
    let input = util::get_day_input(19);
    let mut lines = input.trim().lines();
    let towels = lines
        .next()
        .unwrap()
        .split(", ")
        .map(String::from)
        .collect::<Vec<_>>();

    let designs = lines.skip(1).map(String::from).collect::<Vec<_>>();

    (towels, designs)
}

fn design_possible(towels: &[String], design: &str) -> bool {
    if design.is_empty() {
        return true;
    }
    for towel in towels {
        if design.starts_with(towel) && design_possible(towels, &design[towel.len()..]) {
            return true;
        }
    }
    false
}

fn count_designs(towels: &[&str], design: &str) -> u128 {
    let mut partial_counts_by_len = vec![0u128; design.len()];
    partial_counts_by_len[0] = 1;
    let mut completed_counts: u128 = 0;
    for i in 0..design.len() {
        if partial_counts_by_len[i] == 0 {
            continue;
        }
        for &towel in towels {
            if design[i..].starts_with(towel) {
                let next_index = i + towel.len();
                if next_index == design.len() {
                    completed_counts += partial_counts_by_len[i];
                } else {
                    partial_counts_by_len[next_index] += partial_counts_by_len[i];
                }
            }
        }
    }
    completed_counts
}

fn main() {
    let (towels, designs) = input();

    {
        let _timer = PerfTimer::new("Part 1");

        let filtered_towels = towels
            .iter()
            .filter(|towel| {
                let other_towels = towels.iter().filter(|t| t != towel).cloned().collect_vec();
                !design_possible(&other_towels, towel)
            })
            .cloned()
            .collect_vec();

        let part_1 = designs
            .iter()
            .filter(|design| design_possible(&filtered_towels, design))
            .count();
        println!("Part 1: {part_1}");
    }

    {
        let _timer = PerfTimer::new("Part 2");
        let towels = towels.iter().map(String::as_str).collect_vec();
        let part_2 = designs
            .iter()
            .map(|design| count_designs(&towels, design))
            .sum::<u128>();

        println!("Part 2: {part_2}");
    }
}
