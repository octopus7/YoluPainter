//! 使う人向けの文書（リポジトリの `docs/`）を実行ファイルに埋め込み、MCP の資料 `yolupainter://docs/<名前>` として返す。
//!
//! 埋め込むので、入れてある版の文書がそのまま返る（ネットを読まず、`.mcpb` の中の実行ファイル 1 つでも同じ）。開発の手順（`DEVELOPMENT.md`・
//! `RELEASING.md`）は入れない。`docs/` に文書を足したら、ここへ足す（試験が、足し忘れを断る）。

use yolu_ops::Lang;

/// 資料の URI の前半。
pub const URI_PREFIX: &str = "yolupainter://docs/";

/// One document in Japanese, English, and Korean; unavailable translations are None.
pub struct Doc {
    /// 資料の名前（`yolupainter://docs/<name>`）。
    pub name: &'static str,
    pub ja: Option<&'static str>,
    pub en: Option<&'static str>,
    pub ko: Option<&'static str>,
}

impl Doc {
    /// 既定の言語の本文（英語があれば英語、無ければ日本語）。
    pub fn default_text(&self) -> (&'static str, Lang) {
        match (self.en, self.ja) {
            (Some(en), _) => (en, Lang::En),
            (None, Some(ja)) => (ja, Lang::Ja),
            (None, None) => unreachable!("文書には少なくとも 1 つの言語がある"),
        }
    }
    pub fn text(&self, lang: Lang) -> Option<&'static str> {
        match lang {
            Lang::Ja => self.ja,
            Lang::En => self.en,
            Lang::Ko => self.ko,
        }
    }
}

/// 埋め込む文書（`docs/` の `.md` から開発用を除いた全部）。
pub fn all() -> &'static [Doc] {
    macro_rules! ja {
        ($file:literal) => {
            Some(include_str!(concat!("../../../docs/", $file)))
        };
    }
    macro_rules! en {
        ($file:literal) => {
            Some(include_str!(concat!("../../../docs/en/", $file)))
        };
    }
    macro_rules! ko {
        ($file:literal) => {
            Some(include_str!(concat!("../../../docs/ko/", $file)))
        };
    }
    static DOCS: &[Doc] = &[
        Doc {
            name: "guide",
            ja: ja!("GUIDE.md"),
            en: en!("GUIDE.md"),
            ko: ko!("GUIDE.md"),
        },
        Doc {
            name: "guide-start",
            ja: ja!("GUIDE_START.md"),
            en: en!("GUIDE_START.md"),
            ko: ko!("GUIDE_START.md"),
        },
        Doc {
            name: "guide-paint",
            ja: ja!("GUIDE_PAINT.md"),
            en: en!("GUIDE_PAINT.md"),
            ko: ko!("GUIDE_PAINT.md"),
        },
        Doc {
            name: "guide-select",
            ja: ja!("GUIDE_SELECT.md"),
            en: en!("GUIDE_SELECT.md"),
            ko: ko!("GUIDE_SELECT.md"),
        },
        Doc {
            name: "guide-layers",
            ja: ja!("GUIDE_LAYERS.md"),
            en: en!("GUIDE_LAYERS.md"),
            ko: ko!("GUIDE_LAYERS.md"),
        },
        Doc {
            name: "guide-settings",
            ja: ja!("GUIDE_SETTINGS.md"),
            en: en!("GUIDE_SETTINGS.md"),
            ko: ko!("GUIDE_SETTINGS.md"),
        },
        Doc {
            name: "guide-keys",
            ja: ja!("GUIDE_KEYS.md"),
            en: en!("GUIDE_KEYS.md"),
            ko: ko!("GUIDE_KEYS.md"),
        },
        Doc {
            name: "cli",
            ja: ja!("CLI.md"),
            en: en!("CLI.md"),
            ko: ko!("CLI.md"),
        },
        Doc {
            name: "mcp",
            ja: ja!("MCP.md"),
            en: en!("MCP.md"),
            ko: ko!("MCP.md"),
        },
        Doc {
            name: "unity",
            ja: ja!("UNITY.md"),
            en: en!("UNITY.md"),
            ko: ko!("UNITY.md"),
        },
        Doc {
            name: "install",
            ja: ja!("INSTALL.md"),
            en: en!("INSTALL.md"),
            ko: ko!("INSTALL.md"),
        },
        Doc {
            name: "building",
            ja: ja!("BUILDING.md"),
            en: en!("BUILDING.md"),
            ko: ko!("BUILDING.md"),
        },
        Doc {
            name: "psd",
            ja: ja!("PSD.md"),
            en: None,
            ko: ko!("PSD.md"),
        },
        Doc {
            name: "brush",
            ja: ja!("BRUSH.md"),
            en: None,
            ko: ko!("BRUSH.md"),
        },
        Doc {
            name: "brush-import",
            ja: ja!("BRUSH_IMPORT.md"),
            en: None,
            ko: ko!("BRUSH_IMPORT.md"),
        },
        Doc {
            name: "subtools",
            ja: ja!("SUBTOOLS.md"),
            en: None,
            ko: ko!("SUBTOOLS.md"),
        },
        Doc {
            name: "gradient-map",
            ja: ja!("GRADIENT_MAP.md"),
            en: None,
            ko: ko!("GRADIENT_MAP.md"),
        },
        Doc {
            name: "preview",
            ja: ja!("PREVIEW.md"),
            en: None,
            ko: ko!("PREVIEW.md"),
        },
        Doc {
            name: "recovery",
            ja: ja!("RECOVERY.md"),
            en: None,
            ko: ko!("RECOVERY.md"),
        },
        Doc {
            name: "save-for-distribution",
            ja: ja!("SAVE_FOR_DISTRIBUTION.md"),
            en: None,
            ko: ko!("SAVE_FOR_DISTRIBUTION.md"),
        },
        Doc {
            name: "ylp-format",
            ja: ja!("YLP_FORMAT.md"),
            en: None,
            ko: ko!("YLP_FORMAT.md"),
        },
        Doc {
            name: "ylp-decisions",
            ja: ja!("YLP_DECISIONS.md"),
            en: None,
            ko: ko!("YLP_DECISIONS.md"),
        },
        Doc {
            name: "livelink",
            ja: ja!("LIVELINK.md"),
            en: en!("LIVELINK.md"),
            ko: ko!("LIVELINK.md"),
        },
        Doc {
            name: "guide-fill",
            ja: ja!("GUIDE_FILL.md"),
            en: en!("GUIDE_FILL.md"),
            ko: ko!("GUIDE_FILL.md"),
        },
        Doc {
            name: "guide-paths",
            ja: ja!("GUIDE_PATHS.md"),
            en: en!("GUIDE_PATHS.md"),
            ko: ko!("GUIDE_PATHS.md"),
        },
        Doc {
            name: "guide-3d",
            ja: ja!("GUIDE_3D.md"),
            en: en!("GUIDE_3D.md"),
            ko: ko!("GUIDE_3D.md"),
        },
        Doc {
            name: "guide-files",
            ja: ja!("GUIDE_FILES.md"),
            en: en!("GUIDE_FILES.md"),
            ko: ko!("GUIDE_FILES.md"),
        },
    ];
    DOCS
}

/// 資料の名前（`guide`・`guide.ja`・`guide.en`・`guide.ko`）から、文書と言語を引く。名前だけなら既定の言語。
pub fn find(name: &str) -> Option<(&'static Doc, Lang, &'static str)> {
    let (stem, lang) = match name.rsplit_once('.') {
        Some((stem, "ja")) => (stem, Some(Lang::Ja)),
        Some((stem, "en")) => (stem, Some(Lang::En)),
        Some((stem, "ko")) => (stem, Some(Lang::Ko)),
        _ => (name, None),
    };
    let doc = all().iter().find(|d| d.name == stem)?;
    match lang {
        Some(lang) => Some((doc, lang, doc.text(lang)?)),
        None => {
            let (text, lang) = doc.default_text();
            Some((doc, lang, text))
        }
    }
}

/// 本文の最初の見出し（`# ` の行）。
pub fn title_of(text: &str) -> &str {
    text.lines()
        .find_map(|l| l.strip_prefix("# "))
        .map_or("YoluPainter", str::trim)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_resolve_with_and_without_a_language() {
        let (doc, lang, _) = find("guide").unwrap();
        assert_eq!(
            (doc.name, lang),
            ("guide", Lang::En),
            "英語があれば既定は英語"
        );
        assert_eq!(find("guide.ja").unwrap().1, Lang::Ja);
        let (_, lang, korean) = find("guide.ko").unwrap();
        assert_eq!(lang, Lang::Ko);
        assert_eq!(korean, include_str!("../../../docs/ko/GUIDE.md"));
        assert!(all().iter().all(|doc| doc.text(Lang::Ko).is_some()));
        assert_eq!(find("psd").unwrap().1, Lang::Ja, "日本語だけの文書は日本語");
        assert!(find("psd.en").is_none());
        assert!(find("nothing").is_none());
        assert!(find("guide.fr").is_none());
        assert!(find("../guide").is_none());
    }

    #[test]
    fn every_bundled_doc_is_embedded_and_nothing_else() {
        let docs = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs");
        let left_out = ["DEVELOPMENT.md", "RELEASING.md"];
        let mut on_disk: Vec<String> = Vec::new();
        for (dir, prefix) in [
            (docs.clone(), ""),
            (docs.join("en"), "en/"),
            (docs.join("ko"), "ko/"),
        ] {
            for entry in std::fs::read_dir(dir).unwrap() {
                let entry = entry.unwrap();
                if entry.file_type().unwrap().is_file() {
                    let name = entry.file_name().into_string().unwrap();
                    if !left_out.contains(&name.as_str()) {
                        on_disk.push(format!("{prefix}{name}"));
                    }
                }
            }
        }
        let mut embedded: Vec<String> = Vec::new();
        for doc in all() {
            let file = doc.name.to_ascii_uppercase().replace('-', "_");
            if doc.ja.is_some() {
                embedded.push(format!("{file}.md"));
            }
            if doc.en.is_some() {
                embedded.push(format!("en/{file}.md"));
            }
            if doc.ko.is_some() {
                embedded.push(format!("ko/{file}.md"));
            }
        }
        on_disk.sort();
        embedded.sort();
        assert_eq!(on_disk, embedded, "docs/ の文書（開発用を除く）と埋め込みの一覧が合わない。crates/yolu-mcp/src/docs.rs に足す");
    }

    #[test]
    fn titles_come_from_the_first_heading() {
        assert_eq!(title_of("text\n# Title \nmore"), "Title");
        for doc in all() {
            let (text, _) = doc.default_text();
            assert_ne!(title_of(text), "YoluPainter", "{} に見出しがある", doc.name);
        }
    }
}
