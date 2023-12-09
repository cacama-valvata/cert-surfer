//use ctclient::{CTClient};
use ctclient::internal;
use openssl::x509::{X509, X509Name};
use std::io::Write;
use base64::{prelude::*, engine::general_purpose};
use reqwest::{
    Url
};

fn main() {
    let public_key = general_purpose::STANDARD.decode(b"MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE0JCPZFJOQqyEti5M8j13ALN3CAVHqkVM4yyOcKWCu2yye5yYeqDpEXYoALIgtM3TmHtNlifmt+4iatGwLpF3eA==").unwrap();
    let url: Url = Url::parse("https://ct.googleapis.com/logs/argon2023/").unwrap();
    let client = internal::new_http_client().unwrap();
    let entries = internal::get_entries(&client, &url, 0..1000);
    for entry_result in entries {
        let entry = entry_result.unwrap();
        if !entry.is_pre_cert {
            if let Some(cert_data) = entry.x509_chain.first() {
                let cert = X509::from_der(cert_data).unwrap();
                let name = cert.subject_name();
                let name = name.entries().next().unwrap().data().as_utf8().unwrap();
                println!("{}", name);
            }
        }
    }

    // This lists all the new logs bug it doesn't go through the old ones.
    // I need a way to save the old ones.
    /*
  if std::env::args_os().len() != 1 {
    eprintln!("Expected no arguments.");
    std::process::exit(1);
  }

  // URL and public key copy-pasted from https://www.gstatic.com/ct/log_list/v2/all_logs_list.json .
  // Google's CT log updates very quickly so we use it here.
  let public_key = general_purpose::STANDARD.decode(b"MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE0JCPZFJOQqyEti5M8j13ALN3CAVHqkVM4yyOcKWCu2yye5yYeqDpEXYoALIgtM3TmHtNlifmt+4iatGwLpF3eA==").unwrap();
  // let public_key = ::decode("MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAE6Tx2p1yKY4015NyIYvdrk36es0uAc1zA4PQ+TGRY+3ZjUTIYY9Wyu+3q/147JG4vNVKLtDWarZwVqGkg6lAYzA==").unwrap();
  const URL: &str = "https://ct.googleapis.com/logs/argon2023/";
  let mut client = CTClient::new_from_latest_th(URL, &public_key).unwrap();
  loop {
    let update_result = client.update(Some(|certs: &[X509]| {
      let leaf = &certs[0];
      let ca = &certs[1];
      let canames = certutils::get_common_names(ca).unwrap();
      let caname = &canames[0];
      if let Ok(domains) = certutils::get_dns_names(leaf) {
        print!("{}: ", caname);
        let mut first = true;
        for d in domains.into_iter() {
          if !first {
            print!(", ");
          }
          print!("{}", d);
          first = false;
        }
        print!("\n");
      }
    }));
    if update_result.is_err() {
      eprintln!("Error: {}", update_result.unwrap_err());
    }
    std::io::stdout().flush().unwrap();
  }
  */
}
