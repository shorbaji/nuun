use runtime::{Language, Runtime, PythonRuntime};
use pyo3::prepare_freethreaded_python;

struct Runner {
    runtime: Box<dyn Runtime>,
}

impl Runner {
    pub fn new(language: Language) -> Self {
        match language {
            Language::Python => {
                prepare_freethreaded_python();

                Runner {
                    runtime: Box::new(PythonRuntime::new()),
                }
            },
            _ => unimplemented!("Language not implemented")
        }
    }
}


fn main() {
    let runner = Runner::new(Language::Python);
    
    match runner.runtime.eval("x=42", Some("x")) {
        Ok(s) => println!("{}", s),
        Err(e) => eprintln!("{}", e),
    };

    match runner.runtime.eval("y=7", Some("x + y")) {
        Ok(s) => println!("{}", s),
        Err(e) => eprintln!("{}", e),
    };

    let program = r#"
def double(x):
    return x * 2
    
def quadruple(x):
    return double(double(x))
"#;

    match runner.runtime.eval(program, Some("quadruple(y)")) {
        Ok(s) => println!("{}", s),
        Err(e) => eprintln!("{}", e),
    };

}
