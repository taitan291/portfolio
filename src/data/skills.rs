use crate::data::types::{Lang, Proficiency, Stack};

/// One entry of the skills list. ロゴは Simple Icons (CC0) のSVGを使用。
pub struct Skill {
    pub name: &'static str,
    pub image_url: &'static str,
    /// Language this skill corresponds to, if any.
    pub lang: Option<Lang>,
    pub skills: &'static [(Stack, Proficiency, u8)],
    /// Credit / trademark notice for the logo, shown at the bottom of the modal.
    pub credit: &'static str,
}

pub const SKILLS: &[Skill] = &[
    Skill {
        name: "Astro",
        image_url: "https://cdn.simpleicons.org/astro/000000",
        lang: Some(Lang::Astro),
        skills: &[(Stack::Frontend, Proficiency::Learning, 60)],
        credit: "Astro および Astro のロゴは、各権利者の商標または著作物です。本サイトは Astro 公式とは無関係です。",
    },
    Skill {
        name: "Svelte",
        image_url: "https://cdn.simpleicons.org/svelte/000000",
        lang: Some(Lang::Svelte),
        skills: &[(Stack::Frontend, Proficiency::Learning, 50)],
        credit: "Svelte および Svelte のロゴは、各権利者の商標または著作物です。本サイトは Svelte 公式とは無関係です。",
    },
    Skill {
        name: "Rust",
        image_url: "https://cdn.simpleicons.org/rust/000000",
        lang: Some(Lang::Rust),
        skills: &[
            (Stack::Application, Proficiency::Beginner, 30),
            (Stack::Frontend, Proficiency::Learning, 40),
        ],
        credit: "Rust および Rust のロゴは、Rust Foundation の商標です。本サイトは Rust Foundation とは無関係です。",
    },
    Skill {
        name: "Python",
        image_url: "https://cdn.simpleicons.org/python/000000",
        lang: Some(Lang::Python),
        skills: &[(Stack::Application, Proficiency::Intermediate, 60)],
        credit: "Python および Python のロゴは、Python Software Foundation の商標または登録商標です。本サイトは PSF とは無関係です。",
    },
    Skill {
        name: "Arduino",
        image_url: "https://cdn.simpleicons.org/arduino/000000",
        lang: None,
        skills: &[(Stack::Application, Proficiency::Beginner, 30)],
        credit: "Arduino および Arduino のロゴは、Arduino SA の商標または登録商標です。本サイトは Arduino 公式とは無関係です。",
    },
    Skill {
        name: "Git",
        image_url: "https://cdn.simpleicons.org/git/000000",
        lang: None,
        skills: &[(Stack::Infrastructure, Proficiency::Intermediate, 65)],
        credit: "Git および Git のロゴは、Software Freedom Conservancy の商標です。本サイトは Git プロジェクトとは無関係です。",
    },
    Skill {
        name: "GitHub",
        image_url: "https://cdn.simpleicons.org/github/000000",
        lang: None,
        skills: &[(Stack::Infrastructure, Proficiency::Intermediate, 65)],
        credit: "GitHub および GitHub のロゴは、GitHub, Inc. の商標または登録商標です。本サイトは GitHub とは無関係です。",
    },
];
