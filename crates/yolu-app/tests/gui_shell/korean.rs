//! The language choice must change the rendered app, not only the catalog.
use crate::common;
use crate::common::drive::Drive;
use egui_kittest::kittest::Queryable;
use yolu_app::lang::Lang;
use yolu_app::m2::UiOp;
use yolu_app::prefs::{Category, PrefsAction};
use yolu_app::state::Action;

#[test]
fn korean_can_be_selected_and_switched_back_in_the_settings_window() {
    let mut h = common::app_default(1440.0, 900.0, 64);
    h.act(Action::M2Ui(UiOp::Language(Lang::En)));
    h.act(Action::Prefs(PrefsAction::OpenAt(Category::General)));
    h.run();
    let at = h.get_by_label("Language: English").rect().center();
    common::click(&mut h, at);
    let at = common::popup_item(&h, "한국어").center();
    common::click(&mut h, at);
    h.run();
    assert_eq!(h.lang(), Lang::Ko);
    assert_eq!(h.app().settings().lang, Lang::Ko);
    let _ = h.get_by_label("언어: 한국어");
    assert!(common::menu_title(&h, "파일").width() > 10.0);
    assert!(common::menu_title(&h, "편집").width() > 10.0);

    // Optional local visual inspection without committing a machine-specific
    // screenshot or requiring snapshots for existing Japanese/English fixtures.
    if let Ok(path) = std::env::var("YOLUPAINTER_TEST_SCREENSHOT") {
        h.render().expect("render Korean UI").save(path).unwrap();
    }

    for (current, next, lang) in [
        ("언어: 한국어", "日本語", Lang::Ja),
        ("言語: 日本語", "English", Lang::En),
        ("Language: English", "한국어", Lang::Ko),
    ] {
        let at = h.get_by_label(current).rect().center();
        common::click(&mut h, at);
        let at = common::popup_item(&h, next).center();
        common::click(&mut h, at);
        h.run();
        assert_eq!(h.lang(), lang);
    }
    h.act(Action::Prefs(PrefsAction::Close));
    h.run();
    assert!(common::menu_title(&h, "파일").width() > 10.0);
}
