use pace2026judge::check::check_user_solution;
use pace2026judge::optil_args::Args;

fn main() {
    let args = Args::from_args();
    let result = check_user_solution(&args);
    println!("{}|SUCCESS", result.own_score);
}
