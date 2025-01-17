# chrome-for-testing

Provides serde-enabled type definitions covering the **chrome-for-testing** JSON API responses,
and convenience functions for accessing the API endpoints through `reqwest`.

--- 

Blog post: [https://developer.chrome.com/blog/chrome-for-testing](https://developer.chrome.com/blog/chrome-for-testing)

Availability: [https://googlechromelabs.github.io/chrome-for-testing/](https://googlechromelabs.github.io/chrome-for-testing/)

API-endpoint-definition: [https://github.com/GoogleChromeLabs/chrome-for-testing#json-api-endpoints](https://github.com/GoogleChromeLabs/chrome-for-testing#json-api-endpoints)

---

## Example

```rust
let client = reqwest::Client::new();
let versions = chrome_for_testing::last_known_good_versions::request(client).await.unwrap();
```

## chrome-for-testing-manager

You may also want to check
out [https://github.com/lpotthast/chrome-for-testing-manager](https://github.com/lpotthast/chrome-for-testing-manager),
a crate building upon this one to allow easy selection and installation of chrome-for-testing versions.
It also comes with support for the [thirtyfour](https://crates.io/crates/thirtyfour) crate.
