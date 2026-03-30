use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

////////////////////////////////////////////////////////////////////////////////////////////////////
mod exact {
    use super::*;
    use std::path::PathBuf;

    fn bin() -> PathBuf {
        PathBuf::from(env!("CARGO_BIN_EXE_exact"))
            .canonicalize()
            .unwrap()
    }

    #[test]
    fn test_ok() {
        let (success, stdout, _stderr) = run_binary(
            &bin(),
            &testcase_path("tiny01/instance.nw"),
            &testcase_path("tiny01/opt.sol"),
            2500,
        );
        assert!(success);
        assert!(stdout.contains("4|SUCCESS"));
    }

    #[test]
    fn test_subopt() {
        let (success, stdout, _stderr) = run_binary(
            &bin(),
            &testcase_path("tiny01/instance.nw"),
            &testcase_path("tiny01/subopt.sol"),
            2,
        );
        assert!(success);
        assert!(stdout.contains("5|SUCCESS"));
    }

    #[test]
    fn test_syntax() {
        let (success, stdout, _stderr) = run_binary(
            &bin(),
            &testcase_path("tiny01/instance.nw"),
            &testcase_path("tiny01/syntax_error.sol"),
            2,
        );
        assert!(success);
        assert!(!stdout.contains("|SUCCESS"));
        //assert!(stdout.contains("syntax")); TODO: pace26checker does not emit a syntax error yet
    }

    #[test]
    fn test_empty() {
        let (success, stdout, _stderr) = run_binary(
            &bin(),
            &testcase_path("tiny01/instance.nw"),
            &testcase_path("tiny01/empty.sol"),
            2,
        );
        assert!(success);
        assert!(!stdout.contains("|SUCCESS"));
        assert!(stdout.contains("Empty"));
    }

    #[test]
    fn test_infeasible() {
        let (success, stdout, _stderr) = run_binary(
            &bin(),
            &testcase_path("tiny01/instance.nw"),
            &testcase_path("tiny01/infeasible.sol"),
            2,
        );
        assert!(success);
        assert!(!stdout.contains("|SUCCESS"));
        assert!(stdout.contains("Found only"));
    }
}

mod heuristic {
    use super::*;
    use std::path::PathBuf;

    fn bin() -> PathBuf {
        PathBuf::from(env!("CARGO_BIN_EXE_heuristic"))
            .canonicalize()
            .unwrap()
    }

    #[test]
    fn test_ok() {
        let (success, stdout, _stderr) = run_binary(
            &bin(),
            &testcase_path("tiny01/instance.nw"),
            &testcase_path("tiny01/opt.sol"),
            2500,
        );
        assert!(success);
        assert!(stdout.contains("4|SUCCESS"));
    }

    #[test]
    fn test_subopt() {
        let (success, stdout, _stderr) = run_binary(
            &bin(),
            &testcase_path("tiny01/instance.nw"),
            &testcase_path("tiny01/subopt.sol"),
            2,
        );
        assert!(success);
        assert!(stdout.contains("5|SUCCESS"));
    }

    #[test]
    fn test_syntax() {
        let (success, stdout, _stderr) = run_binary(
            &bin(),
            &testcase_path("tiny01/instance.nw"),
            &testcase_path("tiny01/syntax_error.sol"),
            2,
        );
        assert!(success);
        assert!(!stdout.contains("|SUCCESS"));
        //assert!(stdout.contains("syntax")); TODO: pace26checker does not emit a syntax error yet
    }

    #[test]
    fn test_empty() {
        let (success, stdout, _stderr) = run_binary(
            &bin(),
            &testcase_path("tiny01/instance.nw"),
            &testcase_path("tiny01/empty.sol"),
            2,
        );
        assert!(success);
        assert!(!stdout.contains("|SUCCESS"));
        assert!(stdout.contains("Empty"));
    }

    #[test]
    fn test_infeasible() {
        let (success, stdout, _stderr) = run_binary(
            &bin(),
            &testcase_path("tiny01/instance.nw"),
            &testcase_path("tiny01/infeasible.sol"),
            2,
        );
        assert!(success);
        assert!(!stdout.contains("|SUCCESS"));
        assert!(stdout.contains("Found only"));
    }
}

mod lower_bound {
    use super::*;
    use std::path::PathBuf;

    fn bin() -> PathBuf {
        PathBuf::from(env!("CARGO_BIN_EXE_lower_bound"))
            .canonicalize()
            .unwrap()
    }

    #[test]
    fn test_ok() {
        let (success, stdout, _stderr) = run_binary(
            &bin(),
            &testcase_path("tiny01/instance.nw"),
            &testcase_path("tiny01/opt.sol"),
            2500,
        );
        assert!(success);
        assert!(stdout.contains("2500|SUCCESS"));
    }

    #[test]
    fn test_subopt() {
        let (success, stdout, _stderr) = run_binary(
            &bin(),
            &testcase_path("tiny01/instance.nw"),
            &testcase_path("tiny01/subopt.sol"),
            2,
        );
        assert!(success);
        assert!(stdout.contains("2|SUCCESS"));
    }

    #[test]
    fn test_syntax() {
        let (success, stdout, _stderr) = run_binary(
            &bin(),
            &testcase_path("tiny01/instance.nw"),
            &testcase_path("tiny01/syntax_error.sol"),
            2,
        );
        assert!(success);
        assert!(!stdout.contains("|SUCCESS"));
        //assert!(stdout.contains("syntax")); TODO: pace26checker does not emit a syntax error yet
    }

    #[test]
    fn test_empty() {
        let (success, stdout, _stderr) = run_binary(
            &bin(),
            &testcase_path("tiny01/instance.nw"),
            &testcase_path("tiny01/empty.sol"),
            2,
        );
        assert!(success);
        assert!(!stdout.contains("|SUCCESS"));
        assert!(stdout.contains("Empty"));
    }

    #[test]
    fn test_infeasible() {
        let (success, stdout, _stderr) = run_binary(
            &bin(),
            &testcase_path("tiny01/instance.nw"),
            &testcase_path("tiny01/infeasible.sol"),
            2,
        );
        assert!(success);
        assert!(!stdout.contains("|SUCCESS"));
        assert!(stdout.contains("Found only"));
    }
}

mod tiny {
    use super::*;

    fn bin() -> PathBuf {
        PathBuf::from(env!("CARGO_BIN_EXE_tiny"))
            .canonicalize()
            .unwrap()
    }

    #[test]
    fn test_ok() {
        let (success, stdout, _stderr) = run_binary(
            &bin(),
            &testcase_path("tiny01/instance.nw"),
            &testcase_path("tiny01/opt.sol"),
            2,
        );
        assert!(success);
        assert!(stdout.contains("4|SUCCESS"));
    }

    #[test]
    fn test_subopt() {
        let (success, stdout, _stderr) = run_binary(
            &bin(),
            &testcase_path("tiny01/instance.nw"),
            &testcase_path("tiny01/subopt.sol"),
            2,
        );
        assert!(success);
        assert!(stdout.contains("5|SUCCESS"));
    }

    #[test]
    fn test_syntax() {
        let (success, stdout, _stderr) = run_binary(
            &bin(),
            &testcase_path("tiny01/instance.nw"),
            &testcase_path("tiny01/syntax_error.sol"),
            2,
        );
        assert!(success);
        assert!(!stdout.contains("|SUCCESS"));
        //assert!(stdout.contains("syntax")); TODO: pace26checker does not emit a syntax error yet
    }

    #[test]
    fn test_empty() {
        let (success, stdout, _stderr) = run_binary(
            &bin(),
            &testcase_path("tiny01/instance.nw"),
            &testcase_path("tiny01/empty.sol"),
            2,
        );
        assert!(success);
        assert!(!stdout.contains("|SUCCESS"));
        assert!(stdout.contains("Empty"));
    }

    #[test]
    fn test_infeasible() {
        let (success, stdout, _stderr) = run_binary(
            &bin(),
            &testcase_path("tiny01/instance.nw"),
            &testcase_path("tiny01/infeasible.sol"),
            2,
        );
        assert!(success);
        assert!(!stdout.contains("|SUCCESS"));
        assert!(stdout.contains("Found only"));
    }
}

fn run_binary(
    binary: &Path,
    instance: &Path,
    solution: &Path,
    time: u64,
) -> (bool, String, String) {
    println!("Exec {binary:?} {instance:?} {solution:?} dummy {time} 0");
    let child = Command::new(binary)
        .arg(instance)
        .arg(solution)
        .arg("dummy")
        .arg(time.to_string())
        .arg("0")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let result = child.wait_with_output().unwrap();
    let stdout = String::from_utf8(result.stdout).unwrap();
    let stderr = String::from_utf8(result.stderr).unwrap();

    println!("Result: {:?}", result.status.code());
    println!("STDOUT: {:?}", stdout);
    println!("STDERR: {:?}", stderr);

    (result.status.success(), stdout, stderr)
}

fn testcase_path(relative: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("testcases")
        .join(relative)
}
