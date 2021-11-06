use std::borrow::Cow;

use eframe::{
    egui::{
        CentralPanel,
        FontDefinitions,
        FontFamily,
        ScrollArea,
        Ui,
        Vec2,
    },
    epi::App,
    run_native,
    NativeOptions,
};

struct Headlines {
    articles: Vec<NewsCardData>,
}


impl Headlines {
    pub fn new() -> Headlines {
        let iter = (0..20).map(|a| NewsCardData {
            title: format!("title{}", a),
            desc: format!("description{}", a),
            url: format!("url{}", a),
        });
        Headlines {
            articles: Vec::from_iter(iter),
        }
    }

    fn configure_fonts(
        &self,
        ctx: &eframe::egui::CtxRef,
    ) {
        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert(
            "MicrosoftYiBaiti".to_string(),
            Cow::Borrowed(include_bytes!("../../msyi.ttf")),
        );
        font_def.family_and_size.insert(
            eframe::egui::TextStyle::Heading,
            (FontFamily::Proportional, 35.),
        );
        font_def.family_and_size.insert(
            eframe::egui::TextStyle::Body,
            (FontFamily::Proportional, 20.),
        );
        font_def
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "MicrosoftYiBaiti".to_string());
        ctx.set_fonts(font_def);
    }
}

impl App for Headlines {
    fn setup(
        &mut self,
        _ctx: &eframe::egui::CtxRef,
        _frame: &mut eframe::epi::Frame<'_>,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        self.configure_fonts(_ctx);
    }
    fn update(
        &mut self,
        ctx: &eframe::egui::CtxRef,
        _frame: &mut eframe::epi::Frame<'_>,
    ) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ScrollArea::auto_sized().show(ui, |ui| {
                for a in &self.articles {
                    ui.label(&a.title);
                    ui.label(&a.url);
                    ui.label(&a.desc);
                }
            })
        });
    }

    fn name(&self) -> &str {
        "Headline"
    }
}

struct NewsCardData {
    title: String,
    desc: String,
    url: String,
}

fn main() {
    let app = Headlines::new();
    let mut win_options = NativeOptions::default();
    win_options.initial_window_size = Some(Vec2::new(540., 960.));
    run_native(Box::new(app), win_options)
}
