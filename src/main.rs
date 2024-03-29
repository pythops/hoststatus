use argh::FromArgs;
use std::path::PathBuf;
use std::process::exit;

use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

use tabled::{
    object::{Columns, Object, Rows},
    Alignment, ModifyObject, Style, Table, Tabled,
};

use openssl::ssl::{SslConnector, SslMethod};
use std::net::TcpStream;

enum HostStatus {
    Up,
    Down,
}

impl fmt::Display for HostStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            HostStatus::Up => write!(f, "Up"),
            HostStatus::Down => write!(f, "Down"),
        }
    }
}

#[derive(Tabled)]
struct HostInfos {
    hostname: String,
    status: HostStatus,
    #[tabled(rename = "TLS cert expiration date")]
    expires_at: String,
    infos: String,
}

#[derive(FromArgs)]
#[argh(description = "Simple cli to check hosts status")]
struct Cli {
    /// hostname (example: google.com)
    #[argh(option)]
    host: Option<String>,

    /// path to file containing hostnames
    #[argh(option)]
    path: Option<PathBuf>,
}

fn get_certificate_expiration_date(
    website: &str,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();

    let stream = TcpStream::connect(format!("{}:{}", website, "443"))?;
    let stream = connector.connect(website, stream)?;
    match stream.ssl().peer_certificate() {
        Some(cert) => {
            let not_after = cert.not_after();
            Ok(Some(not_after.to_string()))
        }
        None => Ok(None),
    }
}

fn get_hosts_infos(hostnames: Vec<String>) -> Vec<HostInfos> {
    let mut output: Vec<HostInfos> = Vec::new();
    for hostname in hostnames {
        match get_certificate_expiration_date(hostname.as_str()) {
            Ok(expiration_dates) => match expiration_dates {
                Some(expires_at) => {
                    output.push(HostInfos {
                        hostname,
                        status: HostStatus::Up,
                        expires_at,
                        infos: String::new(),
                    });
                }
                None => {
                    output.push(HostInfos {
                        hostname,
                        status: HostStatus::Up,
                        expires_at: String::new(),
                        infos: "Certificate not found".to_string(),
                    });
                }
            },
            Err(e) => {
                output.push(HostInfos {
                    hostname,
                    status: HostStatus::Down,
                    expires_at: String::new(),
                    infos: format!("Error: {}", e),
                });
            }
        }
    }
    output
}

fn main() -> anyhow::Result<()> {
    let args: Cli = argh::from_env();

    let output = match args {
        Cli {
            host: None,
            path: None,
        } => {
            println!("No option given. You need to specify one of the options");
            exit(1);
        }
        Cli {
            host: _,
            path: Some(path),
        } => {
            let f = File::open(path)?;
            let f = BufReader::new(f);
            let lines: Vec<String> = f.lines().collect::<Result<_, _>>().unwrap();
            get_hosts_infos(lines)
        }
        Cli {
            host: Some(host),
            path: None,
        } => get_hosts_infos(vec![host]),
    };

    let table = Table::new(output)
        .with(Style::ascii())
        .with(
            Rows::new(1..)
                .not(Columns::first())
                .modify()
                .with(Alignment::center()),
        )
        .with(Rows::new(..1).modify().with(Alignment::center()))
        .to_string();
    println!("{}", table);

    Ok(())
}
