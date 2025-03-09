use std::ffi::CString;
use pyo3::types::PyDict;
use pyo3::prelude::*;

pub enum Language {
    Dal,
    Python,
}

pub trait Runtime {
    fn language(&self) -> Language;

    fn eval(&self, program: &str, expr: Option<&str>) -> Result<String, String>;
}

pub struct PythonRuntime {
}

impl PythonRuntime {
    pub fn new() -> Self {
        PythonRuntime {
        }
    }
}

impl Runtime for PythonRuntime {

    fn language(&self) -> Language {
        Language::Python
    }

    fn eval(&self, program: &str, expr: Option<&str>) -> Result<String, String> {
        // we .run() the program with some globals and empty locals
        // and then if Some(expr) we return .eval() using the globals and locals otherwise we return "None"


        Python::with_gil(|py| {
            let globals = PyDict::new(py);
            let locals = PyDict::new(py);

            let program = CString::new(program).expect("CString::new failed");
            let run_result = py.run(&program, Some(&globals), Some(&locals));

            match run_result {
                Ok(_) => {
                    let result = match expr {
                        Some(expr) => {
                            let expr = CString::new(expr).expect("CString::new failed");
                            let eval_result = py.eval(&expr, Some(&globals), Some(&locals));
                            match eval_result {
                                Ok(eval_result) => Ok(eval_result.to_string()),
                                Err(e) => Err(format!("Python execution error: {}", e))
                            }
                        },
                        None => Ok("None".to_string())
                    };


                    match result {
                        Ok(result) => Ok(result.to_string()),
                        Err(e) => Err(format!("Python execution error: {}", e))
                    }
                },
                Err(e) => Err(format!("Python execution error: {}", e))
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pyo3::prepare_freethreaded_python;

    #[test]
    fn test_python_runtime_execution_success() {
        prepare_freethreaded_python();
        let runtime = PythonRuntime::new();
        let result = runtime.eval("x = 5 + 3\nprint(x)", Some("x"));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "8");
    }
    
    #[test]
    fn test_python_runtime_execution_failure() {
        prepare_freethreaded_python();
        let runtime = PythonRuntime::new();
        let result = runtime.eval("invalid python code", None);
    
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Python execution error"));
    }
}