use reqwest::blocking;
use scraper::{ElementRef, Html, Selector};

use std::fmt;

#[derive(Debug)]
pub struct Preview {
    pub url: String,
    pub document: Html,
}

#[derive(Debug)]
pub struct PreviewResponse {
    pub description: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
    pub image: Option<String>,
}

impl fmt::Display for PreviewResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "\nUrl >> {}\nName >> {}\nTitle >> {}\nDescription >> {}\nImage >> {}",
            self.url
                .as_ref()
                .unwrap_or(&"Url not Avaliable".to_string()),
            self.name
                .as_ref()
                .unwrap_or(&"Name not Avaliable".to_string()),
            self.title
                .as_ref()
                .unwrap_or(&"Title not Avaliable".to_string()),
            self.description
                .as_ref()
                .unwrap_or(&"Description not Avaliable".to_string()),
            self.image
                .as_ref()
                .unwrap_or(&"Image not Avaliable".to_string())
        )
    }
}

impl Preview {
    pub fn new(url: &str) -> Preview {
        let document = Html::parse_document(&blocking::get(url).unwrap().text().unwrap());
        Preview {
            url: url.to_owned(),
            document,
        }
    }

    /// Fetch preview fetches all the supported properties
    pub fn fetch_preview(&self) -> PreviewResponse {
        let site_description = self.extract_description();
        let site_title = self.extract_title();
        let site_name = self.extract_site_name();
        let site_image = self.extract_image();
        let site_url = self.extract_site_url(&self.url);

        PreviewResponse {
            description: site_description,
            image: site_image,
            name: site_name,
            url: site_url,
            title: site_title,
        }
    }

    pub(crate) fn extract_description(&self) -> Option<String> {
        let og_description =
            self.extract_from_tag(&self.document, "meta", "property", "og:description");

        if og_description.is_none() {
            let meta_description =
                self.extract_from_tag(&self.document, "meta", "name", "description");
            if meta_description.is_none() {
                return None;
            }
            return Some(
                meta_description
                    .unwrap()
                    .value()
                    .attr("content")
                    .unwrap()
                    .to_owned(),
            );
        }
        return Some(
            og_description
                .unwrap()
                .value()
                .attr("content")
                .unwrap()
                .to_owned(),
        );
    }

    pub(crate) fn extract_title(&self) -> Option<String> {
        let og_title = match self.extract_from_tag(&self.document, "meta", "property", "og:title") {
            Some(title) => title.value().attr("content").unwrap(),
            None => {
                let meta_title = self.extract_from_tag(&self.document, "meta", "name", "title");
                if meta_title.is_none() {
                    let tag_title = self.extract_from_element(&self.document, "title");
                    if tag_title.is_none() {
                        return None;
                    }
                    return Some(tag_title.unwrap().inner_html());
                }
                return Some(
                    meta_title
                        .unwrap()
                        .value()
                        .attr("content")
                        .unwrap()
                        .to_owned(),
                );
            }
        };
        Some(og_title.to_owned())
    }

    pub(crate) fn extract_site_name(&self) -> Option<String> {
        let og_site_name =
            match self.extract_from_tag(&self.document, "meta", "property", "og:site_name") {
                Some(site_name) => site_name.value().attr("content").unwrap(),
                None => {
                    let meta_site_name =
                        self.extract_from_tag(&self.document, "meta", "name", "title");
                    if meta_site_name.is_none() {
                        let tag_title = self.extract_from_element(&self.document, "title");
                        if tag_title.is_none() {
                            return None;
                        }
                        return Some(tag_title.unwrap().inner_html());
                    };
                    return Some(
                        meta_site_name
                            .unwrap()
                            .value()
                            .attr("content")
                            .unwrap()
                            .to_owned(),
                    );
                }
            };
        Some(og_site_name.to_owned())
    }

    pub(crate) fn extract_image(&self) -> Option<String> {
        let og_image = match self.extract_from_tag(&self.document, "meta", "property", "og:image") {
            Some(img) => img.value().attr("content"),
            None => {
                let meta_image = self.extract_from_tag(&self.document, "link", "rel", "image_src");
                if meta_image.is_none() {
                    return None;
                }
                return Some(
                    meta_image
                        .unwrap()
                        .value()
                        .attr("content")
                        .unwrap()
                        .to_owned(),
                );
            }
        };
        Some(og_image.unwrap().to_owned())
    }

    pub(crate) fn extract_site_url(&self, link: &str) -> Option<String> {
        let og_site_url = match self.extract_from_tag(&self.document, "meta", "property", "og:url")
        {
            Some(og_url) => og_url.value().attr("content"),
            None => {
                let meta_site_url =
                    match self.extract_from_tag(&self.document, "link", "rel", "canonical") {
                        Some(meta_url) => meta_url.value().attr("content"),
                        None => {
                            return Some(link.to_owned());
                        }
                    };
                return Some(meta_site_url.unwrap().to_owned());
            }
        };
        Some(og_site_url.unwrap().to_owned())
    }

    pub(crate) fn extract_from_tag<'a>(
        &self,
        document: &'a Html,
        element_name: &'a str,
        attribute: &'a str,
        attribute_name: &'a str,
    ) -> Option<ElementRef<'a>> {
        let formtted_attr = format!("{}[{}='{}']", element_name, attribute, attribute_name);
        let selector = Selector::parse(&&formtted_attr).unwrap();
        let result = document.select(&selector).next();
        return result;
    }

    pub(crate) fn extract_from_element<'a>(
        &self,
        document: &'a Html,
        element: &'a str,
    ) -> Option<ElementRef<'a>> {
        let selector = Selector::parse(element).unwrap();
        let val = document.select(&selector).next();
        return val;
    }
}
