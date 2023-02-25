use error_chain::error_chain;
use select::document::Document;
use select::predicate::Name;
use std::io::{self, Write}; 

error_chain! {
      foreign_links {
          ReqError(reqwest::Error);
          IoError(std::io::Error);
      }
}

#[tokio::main]
async fn main() -> Result<()> {
  print!("Enter URL: ");
  io::stdout().flush().unwrap();

  let mut url = String::new();
  io::stdin().read_line(&mut url)?;

  let res = reqwest::get(url.trim())
  .await?
  .text()
  .await?;

  Document::from(res.as_str())
    .find(Name("a"))
    .filter_map(|n| n.attr("href"))
    .for_each(|x| println!("{}", x));

  Ok(())
}
