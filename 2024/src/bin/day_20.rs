use std::collections::HashMap;

fn input() -> (Vec<Vec<Tile>>, (usize, usize), (usize, usize)) {
    let raw = util::get_day_input(20);
    let mut start = None;
    let mut end = None;
    let mut map = Vec::new();
    for (y, line) in raw.trim().lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start = Some((x, y));
                    row.push(Tile::Track);
                }
                'E' => {
                    end = Some((x, y));
                    row.push(Tile::Track);
                }
                '#' => row.push(Tile::Wall),
                '.' => row.push(Tile::Track),
                _ => panic!("Unexpected character: {c}"),
            }
        }
        map.push(row);
    }
    (
        map,
        start.expect("no start position"),
        end.expect("no end position"),
    )
}

enum Tile {
    Track,
    Wall,
}

fn find_path(map: &[Vec<Tile>], start: (usize, usize), end: (usize, usize)) -> Vec<(usize, usize)> {
    let mut path = vec![start];
    let mut current = start;
    while current != end {
        let mut possible_moves = Vec::with_capacity(1);
        let (x, y) = current;
        if x > 0 && matches!(map[y][x - 1], Tile::Track) {
            possible_moves.push((x - 1, y));
        }
        if x < map[y].len() - 1 && matches!(map[y][x + 1], Tile::Track) {
            possible_moves.push((x + 1, y));
        }
        if y > 0 && matches!(map[y - 1][x], Tile::Track) {
            possible_moves.push((x, y - 1));
        }
        if y < map.len() - 1 && matches!(map[y + 1][x], Tile::Track) {
            possible_moves.push((x, y + 1));
        }
        possible_moves.retain(|pos| Some(pos) != path.iter().nth_back(1));
        assert!(
            possible_moves.len() == 1,
            "Multiple paths found at {current:?}"
        );
        current = possible_moves[0];
        path.push(current);
    }
    path
}

fn main() {
    const TARGET_CHEAT_DISTANCE: usize = 100;

    let (map, start, end) = input();
    {
        let _timer = util::PerfTimer::new("Part 1");
        let path = find_path(&map, start, end);
        let inverse_path: HashMap<(usize, usize), usize> =
            path.iter().enumerate().map(|(i, &pos)| (pos, i)).collect();

        let mut cheats: u128 = 0;
        for (i, &(x, y)) in path.iter().enumerate() {
            let mut possible_cheats = Vec::new();
            if x > 1 && matches!(map[y][x - 2], Tile::Track) {
                possible_cheats.push((x - 2, y));
            }
            if x < map[y].len() - 2 && matches!(map[y][x + 2], Tile::Track) {
                possible_cheats.push((x + 2, y));
            }
            if y > 1 && matches!(map[y - 2][x], Tile::Track) {
                possible_cheats.push((x, y - 2));
            }
            if y < map.len() - 2 && matches!(map[y + 2][x], Tile::Track) {
                possible_cheats.push((x, y + 2));
            }
            if x > 0 && y > 0 && matches!(map[y - 1][x - 1], Tile::Track) {
                possible_cheats.push((x - 1, y - 1));
            }
            if x < map[y].len() - 1 && y > 0 && matches!(map[y - 1][x + 1], Tile::Track) {
                possible_cheats.push((x + 1, y - 1));
            }
            if x > 0 && y < map.len() - 1 && matches!(map[y + 1][x - 1], Tile::Track) {
                possible_cheats.push((x - 1, y + 1));
            }
            if x < map[y].len() - 1 && y < map.len() - 1 && matches!(map[y + 1][x + 1], Tile::Track)
            {
                possible_cheats.push((x + 1, y + 1));
            }

            for cheat in possible_cheats {
                if i + TARGET_CHEAT_DISTANCE + 2 <= inverse_path[&cheat] {
                    cheats += 1;
                }
            }
        }
        println!("Part 1: {cheats}");
    }

    {
        let _timer = util::PerfTimer::new("Part 2");
        let path = find_path(&map, start, end);
        let mut cheats: u128 = 0;
        for (i1, &(x1, y1)) in path.iter().enumerate() {
            for (i2, &(x2, y2)) in path.iter().enumerate() {
                let cheat_length = x1.abs_diff(x2) + y1.abs_diff(y2);
                if cheat_length > 20 {
                    continue;
                }

                if i1 + cheat_length + TARGET_CHEAT_DISTANCE <= i2 {
                    cheats += 1;
                }
            }
        }

        println!("Part 2: {cheats}");
    }
}
