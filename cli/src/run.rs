use clap::ValueEnum;
use nuun::nuun_client::NuunClient;
use nuun::{RunRequest, Language as ProtoLanguage};

pub mod nuun {
    tonic::include_proto!("nuun");
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Language {
    Python,
    Javascript,
    Dal,
}

impl From<Language> for ProtoLanguage {
    fn from(language: Language) -> Self {
        match language {
            Language::Python => ProtoLanguage::Python,
            Language::Dal => ProtoLanguage::Dal,
            Language::Javascript => ProtoLanguage::Javascript,
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Language::Python => write!(f, "python"),
            Language::Javascript => write!(f, "javascript"),
            Language::Dal => write!(f, "dal"),
        }
    }
}

pub async fn run(
    debug: bool,
    language: Language,
    program: String,
    expr: Option<String>,
    host: String,
    port: u16
) {
    let addr = format!("http://{}:{}", host, port);

    let proto_language: ProtoLanguage = language.into();

    match NuunClient::connect(addr).await {
        Ok(mut client) => {
            let run_request = RunRequest {
                language: proto_language as i32,
                program,
                expr,
            };

            let request = tonic::Request::new(run_request);

            match client.run(request).await {
                Ok(response) => {
                    let run_response = response.into_inner();

                    match run_response.result {
                        Some(nuun::run_response::Result::Output(output)) => {
                            println!("Output: {:?}", output);
                        }
                        Some(nuun::run_response::Result::Error(error)) => {
                            eprintln!("Error: {:?}", error);
                        }
                        None => {
                            eprintln!("No result");
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}
