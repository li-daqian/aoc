use aoc_runner_derive::aoc;
use z3::ast::{Ast, Int, Real};

fn parse(input: &str) -> Vec<Vec<f64>> {
    input
        .lines()
        .map(|line| {
            let (position, velocity) = line.split_once('@').unwrap();
            let mut position = position
                .split(',')
                .map(|s| s.trim().parse::<f64>().unwrap())
                .collect::<Vec<_>>();
            let velocity = velocity
                .split(',')
                .map(|s| s.trim().parse::<f64>().unwrap())
                .collect::<Vec<_>>();
            position.extend(velocity);
            position
        })
        .collect()
}

fn solve_part1(input: &str, (min_x, max_x, min_y, max_y): (f64, f64, f64, f64)) -> usize {
    let hailstones = parse(input);

    hailstones
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            hailstones
                .iter()
                .enumerate()
                .skip(i + 1)
                .map(move |(_j, b)| (a, b))
        })
        .filter(|(a, b)| {
            let (x1, y1, _z1, vx1, vy1, _vz1) = (a[0], a[1], a[2], a[3], a[4], a[5]);
            let (x2, y2, _z2, vx2, vy2, _vz2) = (b[0], b[1], b[2], b[3], b[4], b[5]);
            let m1 = vy1 / vx1;
            let m2 = vy2 / vx2;
            if (m2 - m1).abs() <= f64::EPSILON {
                // Parallel lines
                return false;
            }
            let x = (y2 - y1 + m1 * x1 - m2 * x2) / (m1 - m2);
            let y = (m1 * m2 * (x2 - x1) + m2 * y1 - m1 * y2) / (m2 - m1);
            if vx1.signum() != (x - x1).signum() || vx2.signum() != (x - x2).signum() {
                // Not in the same direction
                return false;
            }
            x >= min_x && x <= max_x && y >= min_y && y <= max_y
        })
        .count()
}

#[aoc(day24, part1)]
pub fn part1(input: &str) -> usize {
    solve_part1(
        input,
        (
            200000000000000.0,
            400000000000000.0,
            200000000000000.0,
            400000000000000.0,
        ),
    )
}

#[aoc(day24, part2)]
pub fn part2(input: &str) -> usize {
    let hailstones = parse(input);

    let ctx = z3::Context::new(&z3::Config::new());
    let s = z3::Solver::new(&ctx);
    let [fx, fy, fz, fdx, fdy, fdz] =
        ["fx", "fy", "fz", "fdx", "fdy", "fdz"].map(|v| Real::new_const(&ctx, v));
    let zero = Int::from_i64(&ctx, 0).to_real();
    for (i, hailstone) in hailstones[..3].iter().enumerate() {
        let (x, y, z, dx, dy, dz) = (
            hailstone[0],
            hailstone[1],
            hailstone[2],
            hailstone[3],
            hailstone[4],
            hailstone[5],
        );
        let [x, y, z, dx, dy, dz] =
            [x, y, z, dx, dy, dz].map(|v| Int::from_i64(&ctx, v as _).to_real());
        let t = Real::new_const(&ctx, format!("t{i}"));
        s.assert(&t.ge(&zero));
        s.assert(&((&x + &dx * &t)._eq(&(&fx + &fdx * &t))));
        s.assert(&((&y + &dy * &t)._eq(&(&fy + &fdy * &t))));
        s.assert(&((&z + &dz * &t)._eq(&(&fz + &fdz * &t))));
    }
    assert_eq!(s.check(), z3::SatResult::Sat);
    let res = s
        .get_model()
        .unwrap()
        .eval(&(&fx + &fy + &fz), true)
        .unwrap();
    res.to_string().strip_suffix(".0").unwrap().parse().unwrap()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    const SAMPLE: &str = indoc! {r"
        19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3
    "};

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(SAMPLE, (7.0, 27.0, 7.0, 27.0)), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 47);
    }
}
