fn main() {
    let tests: &[&[usize]] = &[
        &[1, 2, 2, 2, 2],
        &[0, 3, 5, 7, 9],
        &[0, 2, 4, 8, 16],
        &[1, 2, 1],
        // &[0, 2, 1, 0, 0],
        &[0, 6, 10, 14, 18],
        &[1, 4, 8, 12, 16],
        // &[0, 1, 2],
        &[0, 1],
        &[0, 1, 2],
        &[0, 1, 2, 1],
        &[0, 1],
        &[0, 2],
        &[0, 3],
        &[0, 4],
    ];

    for (i, t) in tests.iter().enumerate() {
        println!("{:?}", t);
        dbg!(exp_score(t));
        dbg!(exp_score_2(t));
        dbg!(non_exp_score(t));
    }
}

fn exp_score_2(counts: &[usize]) -> f64 {
    if counts.is_empty() {
        return 0f64;
    }

    assert!(counts[0] <= 1, "Can't have direct contact dupes!");
    if counts[0] == 1 {
        return 1.0f64;
    }

    let mut score = 0f64;

    for (i, ct) in counts.iter().enumerate() {
        if *ct == 0 {
            continue;
        }

        let depth = i + 1;

        let a = 2f64.powf(i as f64);
        let b = 1f64 / a;
        let c = 2f64.powf(*ct as f64);
        let d = 1f64 / c;
        let e = b * (1f64 - d);

        score += e;
    }

    score
}

fn exp_score(counts: &[usize]) -> f64 {
    if counts.is_empty() {
        return 0f64;
    }

    assert!(counts[0] <= 1, "Can't have direct contact dupes!");
    if counts[0] == 1 {
        return 1.0f64;
    }

    let mut score = 0f64;

    for (i, ct) in counts.iter().enumerate() {
        if *ct == 0 {
            continue;
        }

        let depth = i + 1;

        let a = 1f64 / 2f64.powf(i as f64);
        let b = 1f64 / 2f64.powf(depth as f64);
        let c = 1f64 / 2f64.powf((ct - 1) as f64);
        let d = a - (b * c);
        score += d;
    }

    score
}

fn non_exp_score(counts: &[usize]) -> f64 {
    if counts.is_empty() {
        return 0f64;
    }

    assert!(counts[0] <= 1, "Can't have direct contact dupes!");

    // Find largest non-zero count
    if counts.iter().all(|c| *c == 0) {
        return 0f64;
    }

    let mut depth_idx = 10000000;

    for (i, ct) in counts.iter().enumerate() {
        if *ct != 0 {
            depth_idx = i;
            break;
        }
    }

    let depth = depth_idx + 1;

    let a = 1f64 / 2f64.powf((counts[depth_idx] - 1) as f64);
    let b = 1f64 - a;
    let c = (depth as f64) - b;
    let d = 1f64 / c;

    d
}
