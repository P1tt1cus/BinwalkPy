use binwalk::Binwalk;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use std::collections::HashMap;
use std::path::Path;

// pub fn configure(
//     target_file_name: Option<String>,
//     output_directory: Option<String>,
//     include: Option<Vec<String>>,
//     exclude: Option<Vec<String>>,
//     signatures: Option<Vec<signatures::common::Signature>>,
//     full_search: bool,
// ) -> Result<Binwalk, BinwalkError>


// / // Don't scan for these file signatures
// / let exclude_filters: Vec<String> = vec!["jpeg".to_string(), "png".to_string()];
// /
// / let binwalker = Binwalk::configure(None,
// /                                    None,
// /                                    None,
// /                                    Some(exclude_filters),
// /                                    None,
// /                                    false)?;


#[pyfunction]
fn extract(
    file_path: String,
    output_path: Option<String>,
    include: Option<Vec<String>>,
    exclude: Option<Vec<String>>,
    full_search: Option<bool>,
) -> PyResult<Vec<HashMap<String, String>>> {

   // Check if input file exists
    if !Path::new(&file_path).exists() {
        return Err(PyRuntimeError::new_err("Input file does not exist"));
    }

    // Initialize binwalk
    let binwalker = Binwalk::configure(
        Some(file_path),
        output_path, // Requires admin privileges due to symbolic links
        include,
        exclude,
        None,
        full_search.unwrap_or(false),
    )
    .expect("Failed to configure binwalk");

    // Read the file data
    let file_data = std::fs::read(&binwalker.base_target_file)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;

    // Perform scan
    let scan_results = binwalker.scan(&file_data);

    // Perform extraction
    let extraction_results = binwalker.extract(
        &file_data,
        &binwalker.base_target_file,
        &scan_results,
    );

    // Prepare results
    let mut results = Vec::new();
    for (key, value) in extraction_results.iter() {
        let mut result_map = HashMap::new();
        result_map.insert("key".to_string(), key.clone());
        let size_str = value.size.map_or("Unknown".to_string(), |s| s.to_string());
        result_map.insert("size".to_string(), size_str);
        result_map.insert("success".to_string(), value.success.to_string());
        result_map.insert("extractor".to_string(), value.extractor.clone());
        result_map.insert("output_directory".to_string(), value.output_directory.clone());
        results.push(result_map);
    }

    Ok(results)
}

#[pyfunction]
fn scan_file(file_path: &str) -> PyResult<Vec<HashMap<String, String>>> {

    let file_data = std::fs::read(&Path::new(file_path)).map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    let binwalker = Binwalk::new();
    let mut results = Vec::new();
    for result in binwalker.scan(&file_data) {
        let mut result_map = HashMap::new();
        result_map.insert("description".to_string(), result.description.clone());
        result_map.insert("id".to_string(), result.id.clone());
        result_map.insert("name".to_string(), result.name.clone());
        result_map.insert("confidence".to_string(), result.confidence.clone().to_string());
        result_map.insert("offset".to_string(), result.offset.to_string());
        result_map.insert("size".to_string(), result.size.to_string());
        results.push(result_map);
    }
    Ok(results)
}


#[pymodule]
fn binwalkpy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    env_logger::init(); 
    m.add_function(wrap_pyfunction!(scan_file, m)?)?;
    m.add_function(wrap_pyfunction!(extract, m)?)?;
    Ok(())
}