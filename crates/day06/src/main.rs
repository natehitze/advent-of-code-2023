use sscanf::sscanf;

fn main() {
    let input = "Time:        47     84     74     67
Distance:   207   1394   1209   1014";
    let mut lines = input.lines();
    let line1 = lines.next().unwrap();
    let line2 = lines.next().unwrap();

    let (_, time1, _, time2, _, time3, _, time4) = sscanf!(line1, "{str}{i64}{str}{i64}{str}{i64}{str}{i64}").unwrap();
    let (_, distance1, _, distance2, _, distance3, _, distance4) = sscanf!(line2, "{str}{i64}{str}{i64}{str}{i64}{str}{i64}").unwrap();

    let game1_wins = get_win_possibilities(time1, distance1);
    println!("1: {game1_wins}");
    let game2_wins = get_win_possibilities(time2, distance2);
    println!("2: {game2_wins}");
    let game3_wins = get_win_possibilities(time3, distance3);
    println!("3: {game3_wins}");
    let game4_wins = get_win_possibilities(time4, distance4);
    println!("4: {game4_wins}");

    let margin = game1_wins * game2_wins * game3_wins * game4_wins;
    println!("part 1 answer: {margin}");

    let part2_time = line1.split_once(':').unwrap().1.replace(' ', "").parse::<i64>().unwrap();
    let part2_distance = line2.split_once(':').unwrap().1.replace(' ', "").parse::<i64>().unwrap();
    let part2_answer = get_win_possibilities(part2_time, part2_distance);
    println!("part 2 answer: {part2_answer}");
}

fn get_win_possibilities(time: i64, min_distance: i64) -> i64 {
    let mut wins = 0;
    for hold in 0..time {
        if get_distance_by_button_time(hold, time) > min_distance {
            wins += 1;
        }
    }

    return wins;
}

fn get_distance_by_button_time(hold_ms: i64, total_time_ms: i64) -> i64 {
    let distance_per_ms = hold_ms;
    let remaining_time = total_time_ms - hold_ms;

    return distance_per_ms * remaining_time;
}