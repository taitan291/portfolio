use crate::data::types::{Lang, Proficiency, Stack};

/// スキル一覧の 1 項目。ロゴは Simple Icons (CC0) の SVG を使う。
pub struct Skill {
    pub name: &'static str,
    pub image_url: &'static str,
    /// 対応する言語（あれば）。
    pub lang: Option<Lang>,
    /// スタックごとの習熟度。カードの色は先頭のスタックが優先される。
    pub stacks: &'static [StackLevel],
    /// ロゴのクレジット・商標表記。モーダルの下部に表示する。
    pub credit: &'static str,
}

/// あるスタックでの習熟度。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct StackLevel {
    pub stack: Stack,
    pub proficiency: Proficiency,
    /// 習熟度バーの長さ（0-100）。
    pub level: u8,
}

impl StackLevel {
    pub const fn new(stack: Stack, proficiency: Proficiency, level: u8) -> Self {
        Self {
            stack,
            proficiency,
            level,
        }
    }
}

pub const SKILLS: &[Skill] = &[
    Skill {
        name: "Astro",
        image_url: "https://cdn.simpleicons.org/astro/000000",
        lang: Some(Lang::Astro),
        stacks: &[StackLevel::new(Stack::Frontend, Proficiency::Learning, 60)],
        credit: "Astro および Astro のロゴは、各権利者の商標または著作物です。本サイトは Astro 公式とは無関係です。",
    },
    Skill {
        name: "Svelte",
        image_url: "https://cdn.simpleicons.org/svelte/000000",
        lang: Some(Lang::Svelte),
        stacks: &[StackLevel::new(Stack::Frontend, Proficiency::Learning, 50)],
        credit: "Svelte および Svelte のロゴは、各権利者の商標または著作物です。本サイトは Svelte 公式とは無関係です。",
    },
    Skill {
        name: "Rust",
        image_url: "https://cdn.simpleicons.org/rust/000000",
        lang: Some(Lang::Rust),
        stacks: &[
            StackLevel::new(Stack::Application, Proficiency::Beginner, 30),
            StackLevel::new(Stack::Frontend, Proficiency::Learning, 40),
        ],
        credit: "Rust および Rust のロゴは、Rust Foundation の商標です。本サイトは Rust Foundation とは無関係です。",
    },
    Skill {
        name: "Python",
        image_url: "https://cdn.simpleicons.org/python/000000",
        lang: Some(Lang::Python),
        stacks: &[StackLevel::new(
            Stack::Application,
            Proficiency::Intermediate,
            60,
        )],
        credit: "Python および Python のロゴは、Python Software Foundation の商標または登録商標です。本サイトは PSF とは無関係です。",
    },
    Skill {
        name: "Arduino",
        image_url: "https://cdn.simpleicons.org/arduino/000000",
        lang: None,
        stacks: &[StackLevel::new(Stack::Application, Proficiency::Beginner, 30)],
        credit: "Arduino および Arduino のロゴは、Arduino SA の商標または登録商標です。本サイトは Arduino 公式とは無関係です。",
    },
    Skill {
        name: "Git",
        image_url: "https://cdn.simpleicons.org/git/000000",
        lang: None,
        stacks: &[StackLevel::new(
            Stack::Infrastructure,
            Proficiency::Intermediate,
            65,
        )],
        credit: "Git および Git のロゴは、Software Freedom Conservancy の商標です。本サイトは Git プロジェクトとは無関係です。",
    },
    Skill {
        name: "GitHub",
        image_url: "https://cdn.simpleicons.org/github/000000",
        lang: None,
        stacks: &[StackLevel::new(
            Stack::Infrastructure,
            Proficiency::Intermediate,
            65,
        )],
        credit: "GitHub および GitHub のロゴは、GitHub, Inc. の商標または登録商標です。本サイトは GitHub とは無関係です。",
    },
];
