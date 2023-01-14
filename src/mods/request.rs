use std::io::{self, Write};
use std::path::Path;
use std::{collections::HashMap, time::Duration};
use reqwest::header::HeaderMap;
use reqwest::header::{HeaderValue, CONTENT_LENGTH, RANGE};
use reqwest::StatusCode;
use std::fs::File;
use std::str::FromStr;

pub fn sync_getwebpage(
    url: &str,
    user_agent: &str,
    cookie: &str,
    headers: Option<HeaderMap>,
) -> Result<(HashMap<String,String>,String), ()> {
    let client_builder = reqwest::blocking::Client::builder();
    let mut client = if let Ok(value) = client_builder
        .brotli(true)
        .gzip(true)
        .deflate(true)
        .timeout(Duration::from_secs(20))
        .user_agent(user_agent)
        .build()
    {
        value
    } else {
        return Err(());
    }
    .get(url);
    if let Some(value) = headers {
        client = client
            .headers(value)
            .header("cookie", cookie)
            .header("Accept-Encoding", "gzip, deflate, br");
    }
    let rsp_raw_data = if let Ok(value) = client.send() {
        value
    } else {
        return Err(());
    };
    match rsp_raw_data.status().as_u16() {
        404 | 429 => return Err(()),
        _ => (),
    }
    let rsp_headers: HashMap<String, String> = rsp_raw_data
        .headers()
        .iter()
        .map(|(k, v)| (k.as_str().to_owned(), v.to_str().unwrap_or("").to_owned()))
        .collect();
    let rsp_body = if let Ok(value) = rsp_raw_data.text() {
        value
    } else {
        return Err(());
    };
    Ok((rsp_headers, rsp_body))
}



// download

struct PartialRangeIter {
  start: u128,
  end: u128,
  buffer_size: u128,
}

impl PartialRangeIter {
  pub fn new(start: u128, end: u128, buffer_size: u128) -> Result<Self,&'static str> {
    if buffer_size == 0 {
      return Err("invalid buffer_size, give a value greater than zero.");
    }
    Ok(PartialRangeIter {
      start,
      end,
      buffer_size,
    })
  }
}

impl Iterator for PartialRangeIter {
  type Item = HeaderValue;
  fn next(&mut self) -> Option<Self::Item> {
    if self.start > self.end {
      None
    } else {
      let prev_start = self.start;
      self.start += std::cmp::min(self.buffer_size, self.end - self.start + 1);
      Some(HeaderValue::from_str(&format!("bytes={}-{}", prev_start, self.start - 1)).expect("string provided by format!"))
    }
  }
}

pub fn sync_download<P: AsRef<Path>>(url: &str,path: P) -> Result<(), ()> {
  const CHUNK_SIZE: u128 = 1024*1024*8;
    
  let client = reqwest::blocking::Client::new();
  let response = if let Ok(value) = client.head(url).send(){
    value
  } else {
    return Err(());
  };
  let length = response
    .headers()
    .get(CONTENT_LENGTH)
    .ok_or("response doesn't include the content length").unwrap();
  let length = u128::from_str(length.to_str().unwrap()).map_err(|_| "invalid Content-Length header").unwrap();
  let mut output_file = File::create(path.as_ref()).unwrap();
  println!("starting download {}...",path.as_ref().to_str().unwrap());
  print!("\n");
  for range in PartialRangeIter::new(0, length - 1, CHUNK_SIZE).unwrap() {
    print!("\x1b[\x01\x41\x1b[2K\x1b[1Drange {:?}\n", range);
    io::stdout().flush().unwrap_or_default();
    let mut response = client.get(url).header(RANGE, range).send().unwrap();
    
    let status = response.status();
    if !(status == StatusCode::OK || status == StatusCode::PARTIAL_CONTENT) {
      return Err(());
    }
    std::io::copy(&mut response, &mut output_file).unwrap();
  }
    
  let content = response.text().unwrap();
  std::io::copy(&mut content.as_bytes(), &mut output_file).unwrap();

  println!("Finished with success!");
  Ok(())
}