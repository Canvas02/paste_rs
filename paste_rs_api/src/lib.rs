// Copyright 2022 Canvas02.
// SPDX-License-Identifier: MIT

#![deny(dead_code)]
#![deny(unused_variables)]

pub mod util;

use crate::util::*;
use reqwest::Client;

const PASTE_RS_URL: &str = "https://paste.rs/";

/// Used to make a new paste
/// Example:
/// ```rust
/// use paste_rs_api::new_paste;
///
/// #[tokio::main]
///async fn main() {
///     let url = new_paste("Hello World".to_string()).await.unwrap();
///
///     let res = url.make_request().await.unwrap();
/// }
/// ```
pub async fn new_paste(data: String) -> Result<Url, reqwest::Error> {
    let client = Client::new();
    let res = client
        .post(PASTE_RS_URL)
        .body(data)
        .header("Content-Type", "text/plain")
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    Ok(Url::new(&*res).unwrap())

    // !Deprecated
    // // match res.error_for_status() {
    // //     Ok(res) => {
    // //         let res_text = res.text().await?;
    // //         Ok(Url::new(&*res_text).unwrap())
    // //     }
    // //     Err(err) => Err(err),
    // // }
}

pub struct Url(String);

impl std::fmt::Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let url = self.get_url();
        write!(f, "{}", url)
    }
}

impl Url {
    /// Create a new `Url` struct
    /// Example:
    /// ```rust
    /// use paste_rs_api::Url;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let url = Url::new("osx").unwrap();
    ///
    ///     let res = url.make_request().await.unwrap();
    /// }
    /// ```
    pub fn new(val: &str) -> Result<Url, String> {
        // cheching if the argument is an url
        if is_url(val) && is_paste_rs_url(val) {
            Ok(Url(val.to_string()))
        } else if is_url(val) && !is_paste_rs_url(val) {
            Err("The url is not a https://paste.rs/ url".to_string())
        } else if !is_url(val) && is_paste_rs_url(val) {
            Ok(Url(format!("https://{}", val)))
        } else if val.len() == 3 {
            Ok(Url(format!("{}{}", PASTE_RS_URL, val)))
        } else {
            Err("Invalid argument".to_string())
        }
    }
    /// Method to get the url inside the struct
    pub const fn get_url(&self) -> &String {
        let Url(url) = self;
        url
    }

    /// Method to get the id of the Url
    pub fn get_id(&mut self) -> String {
        let Url(url) = self;
        url.replace_range(0..PASTE_RS_URL.len(), "");
        url.clone()
    }

    /// Method to make the request
    /// Example:
    ///
    /// ```rust
    /// use paste_rs_api::Url;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let url = Url::new("osx").unwrap();
    ///
    ///     let res = url.make_request().await.unwrap();
    /// }
    /// ```
    pub async fn make_request(&self) -> Result<String, reqwest::Error> {
        let res = reqwest::get(self.get_url())
            .await?
            .error_for_status()?
            .text()
            .await;
        res
    }
}
