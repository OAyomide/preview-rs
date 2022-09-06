fn main() {
    let prev = preview_rs::Preview::new("https://t.co/Wzr65fyjHA");
    let preview = prev.fetch_preview();

    println!("Here is the preview of this URL: {}", &preview);
    println!("Here is the preview of this URL: {:#?}", &preview);
}
