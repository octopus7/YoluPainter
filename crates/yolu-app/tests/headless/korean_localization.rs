//! Korean regression coverage. Existing Japanese/English fixtures keep their
//! original two-language expectations; shared catalog checks cover all locales.
use crate::common::drive::Drive;
use fluent_bundle::{FluentArgs, FluentBundle, FluentResource};
use yolu_app::lang::{system_lang_from, Lang};
use yolu_app::m2::UiOp;
use yolu_app::prefs::{self, PrefChoice};
use yolu_app::settings::{self, Settings};
use yolu_app::state::{Action, AppState};
use yolu_app::ui::menu::Entry;

#[test]
fn korean_environment_and_windows_display_language_are_detected() {
    let locale = |name: &str| (name == "LANG").then(|| "ko_KR.UTF-8".to_owned());
    assert_eq!(system_lang_from(locale, None), Lang::Ko);
    assert_eq!(system_lang_from(|_| None, Some(0x0412)), Lang::Ko);
    assert_eq!(system_lang_from(|_| None, Some(0x0812)), Lang::Ko);
    assert_eq!(system_lang_from(locale, Some(0x0411)), Lang::Ja);
    let explicit = |name: &str| (name == "YOLUPAINTER_LANG").then(|| "KO-kr".to_owned());
    assert_eq!(system_lang_from(explicit, Some(0x0409)), Lang::Ko);
    let english = |name: &str| (name == "YOLUPAINTER_LANG").then(|| "en".to_owned());
    assert_eq!(system_lang_from(english, Some(0x0412)), Lang::En);
}

#[test]
fn korean_is_selectable_and_language_settings_survive_restart() {
    let app = AppState::starter_in(Lang::En);
    let entries = prefs::entries(&app, PrefChoice::Language);
    assert!(entries.iter().any(|entry| matches!(entry,
        Entry::Item { label, action: Action::M2Ui(UiOp::Language(Lang::Ko)), enabled: true, .. }
        if label == "한국어"
    )));
    let dir = crate::common::tmp::test_dir("korean-language");
    let path = dir.join("settings.conf");
    for lang in [Lang::Ko, Lang::Ja, Lang::En, Lang::Ko] {
        let settings = Settings {
            lang,
            ..Settings::default()
        };
        settings::save(&path, &settings).unwrap();
        let saved = std::fs::read_to_string(&path).unwrap();
        assert!(saved.starts_with(&format!("language={}\n", lang.code())));
        let (loaded, errors) = settings::load_for_startup(&path, Lang::En);
        assert!(errors.is_empty(), "{errors:?}");
        assert_eq!(loaded.lang, lang);
    }
}

#[test]
fn korean_menu_labels_errors_and_new_layer_names_are_localized() {
    assert_eq!(
        yolu_app::m2::blend_label(Lang::Ko, yolu_core::BlendMode::Multiply),
        "곱하기"
    );
    assert_eq!(Lang::Ko.t("shell-file"), "파일");
    assert_eq!(Lang::Ko.t("shell-edit"), "편집");
    assert_eq!(Lang::Ko.t("prefs-language"), "언어");
    assert_eq!(
        Lang::Ko.reason(yolu_core::Reason::ColorBalanceRange),
        "색상 균형 (−100–100)"
    );
    assert_eq!(
        Lang::Ko.core_error(&yolu_core::CoreError::LayerNotFound),
        "레이어를 찾을 수 없습니다"
    );
    let app = AppState::starter_in(Lang::Ko);
    let names: Vec<_> = app
        .doc()
        .layers()
        .iter()
        .map(|layer| layer.name())
        .collect();
    assert_eq!(names, ["채우기 1", "레이어 1"]);
}

#[test]
fn structured_diagnostics_use_korean_and_keep_their_values() {
    use yolu_core::{generator, ClipboardRefusal, CoreError, InactiveReason};
    let lang = Lang::Ko;
    assert_eq!(
        lang.core_error(&CoreError::Clipboard(ClipboardRefusal::NoPixels)),
        "이 레이어에는 픽셀이 없습니다"
    );
    assert_eq!(
        lang.inactive_reason(&InactiveReason::Generator(generator::Inactive::NoModel)),
        "모델이 없습니다"
    );
    assert_eq!(
        lang.model_error(&yolu_model::ModelError::FileTooLarge {
            bytes: 2097152,
            limit: 1048576
        }),
        "파일이 너무 큽니다 (2.0 MiB, 최대 1 MiB)"
    );
    assert!(lang
        .project_note(&yolu_io::Note::Migrated { format: 5 })
        .contains("형식 5"));
    assert_eq!(
        lang.rig_error(&yolu_core::skin::RigError::BadParent { bone: 3 }),
        "본 3의 부모가 올바르지 않습니다"
    );
}

#[test]
fn every_korean_message_formats_without_fluent_runtime_errors() {
    let mut bundle = FluentBundle::new(vec!["ko".parse().unwrap()]);
    bundle.set_use_isolating(false);
    let mut messages = Vec::new();
    for path in super::ftl_keys::ftl_files("ko") {
        let source = std::fs::read_to_string(&path).unwrap();
        let resource = FluentResource::try_new(source).expect("valid Korean Fluent syntax");
        bundle.add_resource(resource).expect("unique Korean keys");
        let (parsed, errors) = super::ftl_keys::parse("ko", &path);
        assert!(errors.is_empty(), "{errors:?}");
        messages.extend(parsed);
    }
    assert!(messages.len() > 4700);
    for message in messages {
        let pattern = bundle.get_message(&message.key).unwrap().value().unwrap();
        for count in [0, 1, 2, 17] {
            let mut args = FluentArgs::new();
            for variable in &message.vars {
                args.set(variable.as_str(), count);
            }
            let mut errors = Vec::new();
            let rendered = bundle.format_pattern(pattern, Some(&args), &mut errors);
            assert!(errors.is_empty(), "{}: {errors:?}", message.key);
            assert!(!rendered.is_empty(), "{} is empty", message.key);
            assert!(
                !rendered.contains('\u{fffd}'),
                "{} contains a replacement character",
                message.key
            );
        }
    }
}

#[test]
fn bundled_fonts_can_lay_out_hangul_in_regular_bold_and_monospace() {
    let ctx = egui::Context::default();
    ctx.set_fonts(yolu_app::ui::fonts::definitions());
    for _ in 0..2 {
        let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
            for family in [
                egui::FontFamily::Proportional,
                egui::FontFamily::Monospace,
                egui::FontFamily::Name(yolu_app::ui::theme::BOLD.into()),
            ] {
                let id = egui::FontId::new(14.0, family);
                ui.fonts_mut(|fonts| {
                    for character in ['가', '한', '글', '힣', 'ㄱ', 'ㅏ', 'ᄒ', 'ᅡ', 'ᆫ']
                    {
                        assert!(fonts.has_glyph(&id, character), "missing {character}");
                    }
                    let galley = fonts.layout_no_wrap(
                        "한국어 브러시 레이어 123".into(),
                        id,
                        egui::Color32::WHITE,
                    );
                    assert!(galley.size().x > 40.0 && galley.size().y > 5.0);
                });
            }
        });
        output.textures_delta.clear();
    }
}
