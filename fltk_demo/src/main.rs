use fltk::{
    app, button,
    enums::{Color, Font, FrameType},
    prelude::{GroupExt, WidgetBase, WidgetExt},
    window,
};
use fltk_theme::{ThemeType, WidgetTheme};

fn main() {
    //let s = String::from("abc");
    //let v = Vec::with_capacity(5);
    let a = app::App::default().with_scheme(app::Scheme::Gleam);

    let theme = WidgetTheme::new(ThemeType::Aero);
    theme.apply();

    let mut win = window::Window::default()
        .with_size(400, 300)
        .with_label("My Window");
    // win.set_color(Color::White);

    let mut btn = button::Button::new(160, 200, 80, 30, "Click");
    // theme_button(&mut &mut btn);

    btn.set_callback(move |b| {
        b.set_label("cliecked");
    });

    win.end();
    win.show();
    a.run().unwrap();
}

fn theme_button(btn: &mut button::Button) {
    btn.clear_visible_focus();
    btn.set_color(Color::from_rgb(255, 0, 0).inactive());
    btn.set_selection_color(Color::from_rgb(255, 0, 0).darker());
    btn.set_frame(FrameType::RFlatBox);
}
