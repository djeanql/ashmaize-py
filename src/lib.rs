use pyo3::prelude::*;
use std::sync::Arc;

use ashmaize::{Rom, RomGenerationType};

#[pyclass]
struct PyRom {
    inner: Arc<Rom>,
}

#[pymethods]
impl PyRom {
    /// Hash a single preimage with default parameters (8 loops, 256 instructions)
    fn hash(&self, preimage: &str) -> PyResult<String> {
        self.hash_with_params(preimage, 8, 256)
    }
    
    /// Hash with custom loop/instruction parameters
    fn hash_with_params(&self, preimage: &str, nb_loops: u32, nb_instrs: u32) -> PyResult<String> {
        let salt = preimage.as_bytes();
        let hash = ashmaize::hash(salt, &self.inner, nb_loops, nb_instrs);
        Ok(hex::encode(hash))
    }

    /// Hash multiple preimages in batch (FASTEST - all in Rust, minimal Python overhead)
    fn hash_batch(&self, preimages: Vec<&str>) -> PyResult<Vec<String>> {
        self.hash_batch_with_params(preimages, 8, 256)
    }

    /// Hash batch with custom parameters
    fn hash_batch_with_params(&self, preimages: Vec<&str>, nb_loops: u32, nb_instrs: u32) -> PyResult<Vec<String>> {
        let results: Vec<String> = preimages
            .iter()
            .map(|preimage| {
                let salt = preimage.as_bytes();
                let hash = ashmaize::hash(salt, &self.inner, nb_loops, nb_instrs);
                hex::encode(hash)
            })
            .collect();
        
        Ok(results)
    }
}

/// Build a ROM from a key string (FullRandom generation)
#[pyfunction]
#[pyo3(signature = (key, size=1073741824))]
fn build_rom(py: Python, key: &str, size: usize) -> PyResult<PyRom> {
    // Use FullRandom generation type
    let gen_type = RomGenerationType::FullRandom;
    
    // Release GIL during ROM building (can take a while)
    let rom = py.allow_threads(|| {
        Rom::new(key.as_bytes(), gen_type, size)
    });
    
    Ok(PyRom { inner: Arc::new(rom) })
}

/// Build a ROM from a key string using TwoStep generation (faster)
#[pyfunction]
#[pyo3(signature = (key, size=1073741824, pre_size=16777216, mixing_numbers=4))]
fn build_rom_twostep(py: Python, key: &str, size: usize, pre_size: usize, mixing_numbers: u32) -> PyResult<PyRom> {
    // Use TwoStep generation type (faster)
    let gen_type = RomGenerationType::TwoStep {
        pre_size,
        mixing_numbers: mixing_numbers as usize
    };
    
    // Release GIL during ROM building (can take a while)
    let rom = py.allow_threads(|| {
        Rom::new(key.as_bytes(), gen_type, size)
    });
    
    Ok(PyRom { inner: Arc::new(rom) })
}

/// Module initialization
#[pymodule]
fn ashmaize_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyRom>()?;
    m.add_function(wrap_pyfunction!(build_rom, m)?)?;
    m.add_function(wrap_pyfunction!(build_rom_twostep, m)?)?;
    Ok(())
}
