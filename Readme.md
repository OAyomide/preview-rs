## Preview-rs

A rust crate to fetch preview metadata for a URL.

Things left to be done:

- [ ] More fallback options especially for images.
- [ ] Support more multimedia options than images (video, png, etc).
- [ ] Allow passing custom user-agent in the request.

### Usage

```rust
    let prev = preview_rs::Preview::new("https://deezer.com");
    let preview = prev.fetch_preview();
    println!("Here is the preview of this URL: {:?}", &preview);

    /** PreviewResponse {
       description: Some("You bring the passion, we bring the music! Access more than 73 million tracks, anytime, anywhere!"),
       title: Some("Deezer | Listen to music | Online music streaming platform"),
       url: Some("http://www.deezer.com"),
       name: Some("Deezer"),
       image: Some("https://e-cdns-files.dzcdn.net/img/common/opengraph-logo.png")
     }
     **/
```
