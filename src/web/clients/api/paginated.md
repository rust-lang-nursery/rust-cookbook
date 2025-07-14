## Consume a paginated RESTful API

[![reqwest-badge]][reqwest] [![serde-badge]][serde] [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding]

Wraps a paginated web API in a convenient Rust iterator. The iterator lazily
fetches the next page of results from the remote server as it arrives at the end of each page.

```rust,edition2021,no_run
// cargo-deps: reqwest="0.11", serde="1"
mod paginated {
  use reqwest::Result;
  use reqwest::header::USER_AGENT;
  use serde::Deserialize;

  #[derive(Deserialize)]
  struct ApiResponse {
      dependencies: Vec<Dependency>,
      meta: Meta,
  }

  #[derive(Deserialize)]
  pub struct Dependency {
      pub crate_id: String,
      pub id: u32,
  }

  #[derive(Deserialize)]
  struct Meta {
      total: u32,
  }

  pub struct ReverseDependencies {
      crate_id: String,
      dependencies: <Vec<Dependency> as IntoIterator>::IntoIter,
      client: reqwest::blocking::Client,
      page: u32,
      per_page: u32,
      total: u32,
  }

  impl ReverseDependencies {
      pub fn of(crate_id: &str) -> Result<Self> {
          Ok(ReverseDependencies {
                 crate_id: crate_id.to_owned(),
                 dependencies: vec![].into_iter(),
                 client: reqwest::blocking::Client::new(),
                 page: 0,
                 per_page: 100,
                 total: 0,
             })
      }

      fn try_next(&mut self) -> Result<Option<Dependency>> {
          if let Some(dep) = self.dependencies.next() {
              return Ok(Some(dep));
          }

          if self.page > 0 && self.page * self.per_page >= self.total {
              return Ok(None);
          }

          self.page += 1;
          let url = format!("https://crates.io/api/v1/crates/{}/reverse_dependencies?page={}&per_page={}",
                            self.crate_id,
                            self.page,
                            self.per_page);
          println!("{}", url);

          let response = self.client.get(&url).header(
                     USER_AGENT,
                     "cookbook agent",
                 ).send()?.json::<ApiResponse>()?;
          self.dependencies = response.dependencies.into_iter();
          self.total = response.meta.total;
          Ok(self.dependencies.next())
      }
  }

  impl Iterator for ReverseDependencies {
      type Item = Result<Dependency>;

      fn next(&mut self) -> Option<Self::Item> {
          match self.try_next() {
              Ok(Some(dep)) => Some(Ok(dep)),
              Ok(None) => None,
              Err(err) => Some(Err(err)),
          }
      }
  }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for dep in paginated::ReverseDependencies::of("serde")? {
        let dependency = dep?;
        println!("{} depends on {}", dependency.id, dependency.crate_id);
    }
    Ok(())
}
