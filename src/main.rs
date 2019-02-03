use plygui::*;
use plygui_scintilla as scintilla;

fn create_frame(name: &str, child: Box<dyn Control>) -> Box<dyn Control> {
	let mut frame = imp::Frame::with_label(name);    
    frame.set_child(Some(child));
    frame.into_control()
}

fn create_splitted(first: Box<dyn Control>, second: Box<dyn Control>) -> Box<dyn Control> {
	let mut splitted = imp::Splitted::with_content(first, second, layout::Orientation::Horizontal);
	splitted.set_layout_width(layout::Size::MatchParent);
    splitted.set_layout_height(layout::Size::WrapContent);
    splitted.into_control()
}

fn create_console() -> Box<dyn Control> {
	use crate::scintilla::{Console, NewConsole};
	
	let mut sc = scintilla::imp::Console::new(false);
    sc.set_layout_width(layout::Size::MatchParent);
    sc.set_layout_height(layout::Size::MatchParent);
    sc.on_resize(Some(
        (|sc: &mut dyn Member, _: u16, _: u16| {
            let co = sc.as_any_mut().downcast_mut::<scintilla::imp::Console>().unwrap();
            #[cfg(target_os = "windows")]
            co.exec("cmd /C dir");
            #[cfg(not(target_os = "windows"))]
            co.exec("ls -l");
         }).into(),
    ));
    sc.into_control()
}

fn create_scintilla(text: &str) -> Box<dyn Control> {
	use crate::scintilla::{Scintilla, NewScintilla};
	
	let mut sc = scintilla::imp::Scintilla::with_content(text);
    sc.set_layout_width(layout::Size::MatchParent);
    sc.set_layout_height(layout::Size::MatchParent);
    sc.on_resize(Some(
        (|sc: &mut dyn Member, w: u16, h: u16| {
            println!("SCINTILLA HAS RESIZED to {}/{}", w, h);
            let sc = sc.as_any_mut().downcast_mut::<plygui_scintilla::imp::Scintilla>().unwrap();
            sc.set_margin_width(0, 25);
         }).into(),
    ));
    sc.into_control()
}

fn root() -> Box<dyn Control> {
	create_splitted(create_frame("Left", create_scintilla("Hello")), create_frame("Right", create_console()))
}

fn main() {
    let mut application = plygui::imp::Application::init_with_name("Plygui test");
    let mut window = application.new_window("plygui!!", WindowStartSize::Exact(1024, 768), WindowMenu::None);
    window.on_resize(Some(
        (|_: &mut dyn Member, w: u16, h: u16| {
             println!("win resized to {}/{}", w, h);
         }).into(),
    ));
    window.set_child(Some(root()));	
    application.start();
}
