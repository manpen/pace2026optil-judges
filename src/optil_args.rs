use std::path::PathBuf;
use std::str::FromStr;

pub struct Args {
    pub instance_input: PathBuf,
    pub reference_output: PathBuf,
    pub user_output: PathBuf,
    pub user_time: u64,
    pub perf_time: u64,
}

impl Args {
    pub fn from_args() -> Args {
        let args = std::env::args().skip(1).take(5).collect::<Vec<_>>();
        assert_eq!(args.len(), 5);

        let instance_input = PathBuf::from(&args[0]);
        let user_output = PathBuf::from(&args[1]);
        let reference_output = PathBuf::from(&args[2]);
        let user_time = u64::from_str(&args[3]).expect("failed to parse time");
        let perf_time = u64::from_str(&args[4]).expect("failed to parse time");

        assert!(instance_input.is_file());
        assert!(user_output.is_file());

        Args {
            instance_input,
            user_output,
            reference_output,
            user_time,
            perf_time,
        }
    }
}
