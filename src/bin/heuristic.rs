use pace2026judge::check::check_user_solution;
use pace2026judge::optil_args::Args;

#[allow(dead_code)]
pub fn compute_pace_heuristic_score(
    solver_score: usize,
    best_known: usize,
    num_leaves: usize,
) -> f64 {
    let solver_score = solver_score as i64;
    let best_known = (best_known as i64).min(solver_score);
    let upper = (num_leaves as i64).min(2 * best_known);

    ((upper - solver_score) as f64 / (upper - best_known) as f64)
        .max(0.0)
        .powi(2)
}

fn main() {
    let args = Args::from_args();
    let result = check_user_solution(&args);
    println!("{}|SUCCESS", result.own_score);
}
