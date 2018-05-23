extern crate plygui;
extern crate plygui_scintilla;

use plygui::*;
use plygui_scintilla::*;

fn main() {
    let mut application = Application::with_name("Plygui test");

    let mut window = application.new_window("plygui!!", WindowStartSize::Exact(200, 200), WindowMenu::None);

    window.on_resize(Some(
        (|_: &mut UiMember, w: u16, h: u16| {
             println!("win resized to {}/{}", w, h);
         }).into(),
    ));

    let mut sc = Scintilla::new();
    sc.set_layout_width(layout::Size::MatchParent);
    sc.set_layout_height(layout::Size::MatchParent);
    sc.set_layout_padding(layout::BoundarySize::AllTheSame(15).into());
    sc.on_resize(Some(
        (|sc: &mut UiMember, w: u16, h: u16| {
            println!("SCINTILLA HAS RESIZED to {}/{}", w, h);
            let sc: &mut Scintilla = sc.as_any_mut().downcast_mut::<Scintilla>().unwrap();
            sc.set_margin_width(0, 25);
         }).into(),
    ));
    
    window.set_child(Some(sc.into_control()));

    //window.set_child(Some(button));
    //window.set_child(Some(sc));

    application.start();
}
