## Preview-rs

A rust crate to fetch preview metadata for a URL.

Things left to be done:

- [ ] More fallback options for images.
- [ ] Support more multimedia options than images (video, png, etc).
- [ ] Allow passing custom user-agent in the request.

### Usage

```rust
    let prev = preview_rs::Preview::new("https://deezer.com");
    let preview = prev.fetch_preview();
    println!("Here is the preview of this URL: {:?}", &preview);
```
