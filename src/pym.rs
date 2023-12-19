//! This module is for python scripts.
//!
//! Seems like some error exists now, welocome pr to fix this : )
use crate::{cache::Cache, helper::load_script, Result};
use pyo3::prelude::*;

/// Exec python scripts as filter
pub fn exec(module: &str) -> Result<Vec<String>> {
    pyo3::prepare_freethreaded_python();
    let script = load_script(&module)?;
    let cache = Cache::new()?;

    // args
    let sps = serde_json::to_string(&cache.get_problems()?)?;
    let stags = serde_json::to_string(&cache.get_tags()?)?;

    // pygil
    Python::with_gil(|py| {
        let pym = PyModule::from_code(py, &script, "plan.py", "plan")?;
        pym.getattr("plan")?.call1((sps, stags))?.extract()
    })
    .map_err(Into::into)
}
