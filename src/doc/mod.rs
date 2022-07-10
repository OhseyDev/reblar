pub mod md;
pub mod html;
pub mod tex;

pub enum Document {
    Html(html::Document),
    Markdown(md::Document),
    Tex(tex::Document),
}
