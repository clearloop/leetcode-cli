use serde_json;
use leetcode_cli::cache::models::VerifyReslut;

fn main() {
    let r: Result<VerifyReslut, serde_json::Error> = serde_json::from_str(r#"{"status_code": 10, "lang": "rust", "run_success": true, "status_runtime": "0 ms", "memory": 2132000, "code_answer": ["[7,2]"], "code_output": [], "elapsed_time": 0, "task_finish_time": 1578173189094, "expected_status_code": 10, "expected_lang": "cpp", "expected_run_success": true, "expected_status_runtime": "4", "expected_memory": 8208000, "expected_code_answer": ["[0,1]"], "expected_code_output": [], "expected_elapsed_time": 17, "expected_task_finish_time": 1578170865173, "correct_answer": false, "total_correct": null, "total_testcases": null, "runtime_percentile": null, "status_memory": "2.1 MB", "memory_percentile": null, "pretty_lang": "Rust", "submission_id": "runcode_1578173185.216418_VjtMZiRkC5", "status_msg": "Accepted", "state": "SUCCESS"}"#);

    println!("{:?}", r);
}
