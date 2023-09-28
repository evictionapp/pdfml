use pdfml::render::render;

fn main() {
    let foo = include_str!("../../foo.pdfml");

    render(foo);
}
