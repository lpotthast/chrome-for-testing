# chrome-for-testing

Implementation of the **chrome-for-testing** JSON API.

--- 

Blog post: [https://developer.chrome.com/blog/chrome-for-testing](https://developer.chrome.com/blog/chrome-for-testing)

Availability: [https://googlechromelabs.github.io/chrome-for-testing/](https://googlechromelabs.github.io/chrome-for-testing/)

API-endpoint-definition: [https://github.com/GoogleChromeLabs/chrome-for-testing#json-api-endpoints](https://github.com/GoogleChromeLabs/chrome-for-testing#json-api-endpoints)

---

Provides serde-enabled type definitions for responses-parsing and convenience functions for accessing the endpoints
through `reqwest`.

## chrome-for-testing-manager

You may also want to check
out [https://github.com/lpotthast/chrome-for-testing-manager](https://github.com/lpotthast/chrome-for-testing-manager),
a crete building upon this one to allow easy selection and installation of chrome-for-testing versions. Also comes with
support for the [thirtyfour](https://crates.io/crates/thirtyfour) crate.
