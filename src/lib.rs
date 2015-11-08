
extern crate pulldown_cmark;

use pulldown_cmark::Parser;
use pulldown_cmark::{Options, OPTION_ENABLE_TABLES, OPTION_ENABLE_FOOTNOTES};
use pulldown_cmark::html;

pub fn render_html(text: &str) -> String {
    let mut opts = Options::empty();
    opts.insert(OPTION_ENABLE_TABLES);
    opts.insert(OPTION_ENABLE_FOOTNOTES);
    let mut s = String::with_capacity(text.len() * 3 / 2);
    let p = Parser::new_ext(&text, opts);
    html::push_html(&mut s, p);
    s
}
