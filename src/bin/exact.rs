use pace2026judge::check::check_user_solution;
use pace2026judge::optil_args::Args;

fn main() {
    let args = Args::from_args();
    let result = check_user_solution(&args);

    if result.own_score == result.best_known {
        println!("{}|SUCCESS", args.user_time as f64 / 1000.0);
    } else {
        println!(
            "3|Suboptimal solution. Best publicly known {}. Got {}",
            result.own_score, result.best_known
        );
    }
}
