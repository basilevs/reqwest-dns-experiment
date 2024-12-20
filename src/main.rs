use reqwest::blocking::ClientBuilder;
use anyhow::{Result, anyhow};
use reqwest::Error as ReqError;
use std::error::Error as StdError;

fn main() -> Result<()> {
    assert!(!is_transient_result(&ping("https://google.com/meh")), "HTTP 404 is expected");
    assert!(!is_transient_result(&ping("https://google.com/")), "HTTP 200 is expected");
    assert!(is_transient_result(&ping("https://unknown_host1.com/meh")), "DNS failure is expected");
    assert!(is_transient_result(&ping("https://240.0.0.1/meh")), "Connection failure is expected");
    return Ok(())
}

fn is_transient_result(result: &Result<String>) -> bool {
    return match  result {
        Ok(_) => false,
        Err(e) => is_transient_error(e.as_ref()), 
    }
}

fn is_transient_error(e1: &(dyn StdError + 'static)) -> bool {
    print!("{}: {}\n", std::any::type_name_of_val(e1), e1);

    if let Some::<&ReqError>(e3) = e1.downcast_ref::<ReqError>() {
        println!("Downcasted to ReqError\n");
        if e3.is_timeout() {
            return true;
        }
        if e3.is_connect() {
            return true;
        }
        if let Some(source) =  e3.source() {
            return is_transient_error(source);
        }
    }

    if let Some(e5) = e1.source() {
        return is_transient_error(e5)
    }
    
    return false
}

fn ping(url: &str) -> Result<String> {
    return Ok(ClientBuilder::default().build()?.get(url).send()?.text()?);
}