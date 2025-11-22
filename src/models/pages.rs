use rshtml::{RsHtml};

#[derive(RsHtml)]
pub struct HomePage {
    pub title: String,
}

#[derive(RsHtml)]
pub struct IndexPage {}
