use crate::optil_args::Args;
use pace26checker::checks::checker::{CheckerError, check_instance_and_solution};
use pace26checker::digest::algo::digest_instance_with_approx;
use pace26checker::io::solution_reader::SolutionReaderError;
use pace26remote::job_description::JobDescription;
use pace26remote::job_transfer::{TransferFromServer, TransferToServer};

#[macro_export]
macro_rules! println_exit {
    () => {
        println!();
        std::process::exit(0);
    };
    ($($arg:tt)*) => {
        println!($($arg)*);
        std::process::exit(0);
    };
}

fn upload_solution_and_get_best_known(jd: JobDescription) -> u32 {
    let to_server = TransferToServer { jobs: vec![jd] };
    let client = reqwest::blocking::Client::new();
    let response: TransferFromServer = client
        .post("https://pace2026.imada.sdu.dk/api/solution")
        .json(&to_server)
        .send()
        .expect("Internal error -- STRIDE server error")
        .json()
        .expect("Internal error -- STRIDE response error");

    *response
        .best_scores
        .get(&to_server.jobs[0].idigest)
        .expect("Internal error -- STRIDE job missing")
}

pub struct CheckResult {
    pub own_score: u32,
    pub best_known: u32,
    pub num_leaves: u32,
}

pub fn check_user_solution(args: &Args) -> CheckResult {
    let check_result =
        check_instance_and_solution(&args.instance_input, &args.user_output, false, true);

    match check_result {
        Ok((inst, solution, _forest)) => {
            let instance = inst.expect("Internal error -- instance missing");

            let trees = instance
                .trees()
                .iter()
                .map(|(_, t)| t.clone())
                .collect::<Vec<_>>();

            let idigest =
                digest_instance_with_approx(trees, instance.num_leaves, instance.approx());

            let trees = solution
                .trees()
                .iter()
                .map(|(_, t)| t.clone())
                .collect::<Vec<_>>();

            let own_score = trees.len() as u32;
            let jd = JobDescription::valid(idigest, trees, None);
            let server_score = upload_solution_and_get_best_known(jd);
            let best_known = server_score.min(own_score);

            CheckResult {
                own_score,
                best_known,
                num_leaves: instance.num_leaves,
            }
        }
        Err(err) => match err {
            CheckerError::TreeInsertion { .. } => {
                println_exit!("Internal error -- tree insertion missing");
            }
            CheckerError::Mismatch {
                inst_lineno,
                sol_lineno,
            } => {
                println_exit!(
                    "2|Solution in line {} does not match instance line {}",
                    sol_lineno + 1,
                    inst_lineno + 1
                );
            }
            CheckerError::Io(_) => {
                println_exit!("Internal error -- I/O error");
            }
            CheckerError::InstanceReaderError(_) => {
                println_exit!("Internal error -- failed to read instance");
            }
            CheckerError::SolutionReaderError(e) => match e {
                SolutionReaderError::Io(_) => {
                    panic!("Internal error -- I/O error");
                }
                SolutionReaderError::VisitorError(e) => {
                    println_exit!("5|{e}");
                }
                SolutionReaderError::VisitorWarning(e) => {
                    println_exit!("6|{e}");
                }
                SolutionReaderError::EmptySolution => {
                    println_exit!("4|Empty solution");
                }
            },
        },
    }
}
