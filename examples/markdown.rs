use fltk::{prelude::*, *};
use pulldown_cmark::{html, Options, Parser};

fn main() {
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut buf = text::TextBuffer::default();
    let mut win = window::Window::default().with_size(800, 600);
    let mut pack = group::Pack::default()
        .with_size(790, 590)
        .center_of_parent()
        .with_type(group::PackType::Horizontal);
    pack.set_spacing(5);
    let mut editor = text::TextEditor::default().with_size(395, 0);
    editor.set_buffer(buf.clone());
    let mut wv_win = window::Window::default().with_size(390, 0);
    pack.end();
    win.end();
    win.show();

    let mut wv = fltk_webview::Webview::create(false, &mut wv_win);
    wv.navigate("data:text/html,<html></html>");

    buf.add_modify_callback({
        let mut wv = wv.clone();
        move |_, _, _, _, _| {
            let txt = editor.buffer().unwrap().text();
            let mut options = Options::empty();
            options.insert(Options::ENABLE_STRIKETHROUGH);
            let parser = Parser::new_ext(&txt, options);
            let mut html_output: String = String::with_capacity(txt.len() * 3 / 2);
            html::push_html(&mut html_output, parser);
            html_output = String::from("data:text/html,") + &html_output;
            wv.navigate(&html_output);
        }
    });

    a.run().unwrap();
}
