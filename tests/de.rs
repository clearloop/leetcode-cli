#[macro_use]
extern crate log;
use leetcode_cli::cache::models::VerifyResult;
use serde_json;

#[test]
fn de_from_submit_success() {
    let r: Result<VerifyResult, serde_json::Error> = serde_json::from_str(
        r#"{"status_code": 10, "lang": "rust", "run_success": true, "status_runtime": "0 ms", "memory": 2300000, "question_id": "1", "elapsed_time": 0, "compare_result": "11111111111111111111111111111", "code_output": "", "std_output": "", "last_testcase": "", "task_finish_time": 1578193674018, "total_correct": 29, "total_testcases": 29, "runtime_percentile": 100, "status_memory": "2.3 MB", "memory_percentile": 100, "pretty_lang": "Rust", "submission_id": "291285717", "status_msg": "Accepted", "state": "SUCCESS"}"#,
    );
    assert!(r.is_ok());
}

#[test]
fn de_from_test_limit_exceed() {
    let r: Result<VerifyResult, serde_json::Error> = serde_json::from_str(
        r#"{"status_code": 13, "lang": "rust", "run_success": false, "status_runtime": "N/A", "memory": 2048000, "code_answer": [], "code_output": ["ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "ever in loop.", "... 374392 more lines"], "elapsed_time": 0, "task_finish_time": 1578215847058, "total_correct": null, "total_testcases": null, "runtime_percentile": null, "status_memory": "N/A", "memory_percentile": null, "pretty_lang": "Rust", "submission_id": "runcode_1578215840.1441765_Bj7ADXgHrl", "status_msg": "Output Limit Exceeded", "state": "SUCCESS"}"#,
    );
    assert!(r.is_ok());
}

#[test]
fn de_from_test_success() {
    let r: Result<VerifyResult, serde_json::Error> = serde_json::from_str(
        r#"{"status_code": 10, "lang": "rust", "run_success": true, "status_runtime": "0 ms", "memory": 2040000, "code_answer": ["[0,1]"], "code_output": [], "elapsed_time": 0, "task_finish_time": 1578201833478, "expected_status_code": 10, "expected_lang": "cpp", "expected_run_success": true, "expected_status_runtime": "0", "expected_memory": 8296000, "expected_code_answer": ["[0,1]"], "expected_code_output": [], "expected_elapsed_time": 20, "expected_task_finish_time": 1578201003754, "correct_answer": true, "total_correct": null, "total_testcases": null, "runtime_percentile": null, "status_memory": "2 MB", "memory_percentile": null, "pretty_lang": "Rust", "submission_id": "runcode_1578201829.4103167_XbDDrj9Ihb", "status_msg": "Accepted", "state": "SUCCESS"}"#,
    );
    assert!(r.is_ok());
}

#[test]
fn de_from_float_pencentile() {
    env_logger::init();
    let r: Result<VerifyResult, serde_json::Error> = serde_json::from_str(
        r#"{"status_code": 10, "lang": "rust", "run_success": true, "status_runtime": "4 ms", "memory": 2716000, "question_id": "203", "elapsed_time": 0, "compare_result": "11111111111111111111111111111111111111111111111111111111111111111", "code_output": "", "std_output": "", "last_testcase": "", "task_finish_time": 1578590021187, "total_correct": 65, "total_testcases": 65, "runtime_percentile": 76.9231, "status_memory": "2.7 MB", "memory_percentile": 100, "pretty_lang": "Rust", "submission_id": "292701790", "status_msg": "Accepted", "state": "SUCCESS"}"#,
    );
    debug!("{:?}", &r);
    assert!(r.is_ok());
}
