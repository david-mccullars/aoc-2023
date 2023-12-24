use advent_of_code::*;
use itertools::Itertools;
use nalgebra::*;
use z3::ast::*;
use z3::*;

advent_of_code::solution!(24);

type Hail = (u64, u64, u64, i64, i64, i64);

pub fn part_one(input: &str) -> Option<u32> {
    _part_one(input, 200000000000000.0, 400000000000000.0)
}

pub fn _part_one(input: &str, min: f64, max: f64) -> Option<u32> {
    let hail = parse(input);

    let intersection = hail
        .iter()
        .cartesian_product(hail.iter())
        .filter(|(h0, h1)| h0.cmp(h1) == std::cmp::Ordering::Greater)
        .filter(|(h0, h1)| {
            let (x0, y0, _, dx0, dy0, _) = h0;
            let (x1, y1, _, dx1, dy1, _) = h1;

            let coefficients =
                Matrix2::new(*dx0 as f64, -(*dx1 as f64), *dy0 as f64, -(*dy1 as f64));

            let results = Vector2::new((*x1 as f64) - (*x0 as f64), (*y1 as f64) - (*y0 as f64));

            let lu = LU::new(coefficients);
            if let Some(solution) = lu.solve(&results) {
                let x = (*x0 as f64) + (*dx0 as f64 * solution[0]);
                let y = (*y0 as f64) + (*dy0 as f64 * solution[0]);
                solution[0] >= 0.0
                    && solution[1] >= 0.0
                    && min <= x
                    && x <= max
                    && min <= y
                    && y <= max
            } else {
                false
            }
        })
        .count();

    Some(intersection as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let hail = parse(input);

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    // Variables
    let x = Real::new_const(&ctx, "x");
    let y = Real::new_const(&ctx, "y");
    let z = Real::new_const(&ctx, "z");
    let dx = Real::new_const(&ctx, "dx");
    let dy = Real::new_const(&ctx, "dy");
    let dz = Real::new_const(&ctx, "dz");

    let t0 = Real::new_const(&ctx, "t0");
    let (x2, y2, z2, dx2, dy2, dz2) = to_real(&ctx, &hail[0]);
    solver.assert(&(&x + &t0 * &(&dx - &dx2))._eq(&x2));
    solver.assert(&(&y + &t0 * &(&dy - &dy2))._eq(&y2));
    solver.assert(&(&z + &t0 * &(&dz - &dz2))._eq(&z2));

    let t1 = Real::new_const(&ctx, "t1");
    let (x2, y2, z2, dx2, dy2, dz2) = to_real(&ctx, &hail[1]);
    solver.assert(&(&x + &t1 * &(&dx - &dx2))._eq(&x2));
    solver.assert(&(&y + &t1 * &(&dy - &dy2))._eq(&y2));
    solver.assert(&(&z + &t1 * &(&dz - &dz2))._eq(&z2));

    let t2 = Real::new_const(&ctx, "t2");
    let (x2, y2, z2, dx2, dy2, dz2) = to_real(&ctx, &hail[2]);
    solver.assert(&(&x + &t2 * &(&dx - &dx2))._eq(&x2));
    solver.assert(&(&y + &t2 * &(&dy - &dy2))._eq(&y2));
    solver.assert(&(&z + &t2 * &(&dz - &dz2))._eq(&z2));

    // Check if the constraints are satisfiable
    let result = solver.check();

    match result {
        SatResult::Sat => {
            let model = solver.get_model().unwrap();
            let sum = to_i64(&model, &x) + to_i64(&model, &y) + to_i64(&model, &z);
            Some(sum as u64)
        }
        _ => {
            eprintln!("No solution found!");
            None
        }
    }
}

parser!(
    Hail,
    r"^(\d+),\s+(\d+),\s+(\d+)\s+@\s+([^,]+),\s+([^,]+),\s+([^,]+)$",
    u64_parser!(),
    u64_parser!(),
    u64_parser!(),
    i64_parser!(),
    i64_parser!(),
    i64_parser!()
);

fn parse(input: &str) -> Vec<Hail> {
    input
        .lines()
        .map(|line| parse_Hail(line).unwrap())
        .collect()
}

fn to_real<'a>(
    ctx: &'a Context,
    hail: &'a Hail,
) -> (Real<'a>, Real<'a>, Real<'a>, Real<'a>, Real<'a>, Real<'a>) {
    (
        Int::from_u64(ctx, hail.0).to_real(),
        Int::from_u64(ctx, hail.1).to_real(),
        Int::from_u64(ctx, hail.2).to_real(),
        Int::from_i64(ctx, hail.3).to_real(),
        Int::from_i64(ctx, hail.4).to_real(),
        Int::from_i64(ctx, hail.5).to_real(),
    )
}

fn to_i64(model: &Model, value: &Real) -> i64 {
    let i = model.eval(value, true).unwrap().as_real().unwrap();
    assert_eq!(i.1, 1);
    i.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = _part_one(
            &advent_of_code::template::read_file("examples", DAY),
            7.0,
            27.0,
        );
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(47));
    }
}
