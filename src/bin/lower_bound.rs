use pace2026judge::check::check_user_solution;
use pace2026judge::optil_args::Args;

fn main() {
    let args = Args::from_args();
    let _ = check_user_solution(&args);

    let time = if args.user_time.is_nan() {
        0
    } else {
        args.user_time.round() as u64
    };
    println!("{time}|SUCCESS");
}
