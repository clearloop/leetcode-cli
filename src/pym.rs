//! This module is for python scripts.
//!
//! Seems like some error exists now, welocome pr to fix this : )
use crate::cache::Cache;
use crate::helper::load_script;
use pyo3::prelude::*;

/// Exec python scripts as filter
pub fn exec(module: &str) -> Result<Vec<String>, crate::Error> {
    let script = load_script(&module)?;
    let cache = Cache::new()?;

    // pygil
    let gil = Python::acquire_gil();
    let py = gil.python();
    let pym = PyModule::from_code(py, &script, "plan.py", "plan")?;

    // args
    let sps = serde_json::to_string(&cache.get_problems()?)?;
    let stags = serde_json::to_string(&cache.get_tags()?)?;

    // ret
    let res: Vec<String> = pym.call1("plan", (sps, stags))?.extract()?;

    Ok(res)
}
