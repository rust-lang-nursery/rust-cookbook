## Consume a paginated RESTful API

[![reqwest-badge]][reqwest] [![serde-badge]][serde] [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding]

Wraps a paginated web API in a convenient Rust iterator. The iterator lazily
fetches the next page of results from the remote server as it arrives at the end
of each page.

```rust,edition2018,no_run
use reqwest::Result;
use serde::Deserialize;

#[derive(Deserialize)]
struct ApiResponse {
    dependencies: Vec<Dependency>,
    meta: Meta,
}

#[derive(Deserialize)]
struct Dependency {
    crate_id: String,
}

#[derive(Deserialize)]
struct Meta {
    total: u32,
}

struct ReverseDependencies {
    crate_id: String,
    dependencies: <Vec<Dependency> as IntoIterator>::IntoIter,
    client: reqwest::blocking::Client,
    page: u32,
    per_page: u32,
    total: u32,
}

impl ReverseDependencies {
    fn of(crate_id: &str) -> Result<Self> {
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

        let response = self.client.get(&url).send()?.json::<ApiResponse>()?;
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

fn main() -> Result<()> {
    for dep in ReverseDependencies::of("serde")? {
        println!("reverse dependency: {}", dep?.crate_id);
    }
    Ok(())
}
```
