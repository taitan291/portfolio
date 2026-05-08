use crate::data::types::{Proficiency, Stack};

/// One entry of the skills list.
pub struct Skill {
    pub name: &'static str,
    pub image_url: &'static str,
    pub skills: &'static [(Stack, Proficiency, u8)],
    /// Credit / trademark notice for the logo, shown at the bottom of the modal.
    pub credit: &'static str,
}

pub const SKILLS: &[Skill] = &[
    Skill {
        name: "Astro",
        image_url: "assets/astro_icon.svg",
        skills: &[(Stack::Frontend, Proficiency::Learning, 60)],
        credit: "Astro および Astro のロゴは、各権利者の商標または著作物です。本サイトは Astro 公式とは無関係です。",
    },
    Skill {
        name: "Svelte",
        image_url: "assets/svelte_icon.svg",
        skills: &[(Stack::Frontend, Proficiency::Learning, 50)],
        credit: "Svelte および Svelte のロゴは、各権利者の商標または著作物です。本サイトは Svelte 公式とは無関係です。",
    },
    Skill {
        name: "Rust",
        image_url: "assets/rust_icon.svg",
        skills: &[
            (Stack::Application, Proficiency::Beginner, 30),
            (Stack::Frontend, Proficiency::Learning, 40),
        ],
        credit: "Rust のロゴは Rust Foundation による CC BY 4.0 のもとで提供されています。Rust および Rust のロゴは Rust Foundation の商標です。本サイトは Rust Foundation とは無関係です。",
    },
    Skill {
        name: "Python",
        image_url: "assets/python_icon.svg",
        skills: &[(Stack::Application, Proficiency::Intermediate, 60)],
        credit: "Python および Python のロゴは、Python Software Foundation の商標または登録商標です。本サイトは PSF とは無関係です。",
    },
    Skill {
        name: "Git",
        image_url: "assets/git_icon.svg",
        skills: &[(Stack::Infrastructure, Proficiency::Intermediate, 65)],
        credit: "Git のロゴは Jason Long による CC BY 3.0 のもとで提供されています。本サイトは Git プロジェクトとは無関係です。",
    },
    Skill {
        name: "GitHub",
        image_url: "assets/github_icon.svg",
        skills: &[(Stack::Infrastructure, Proficiency::Intermediate, 65)],
        credit: "GitHub および GitHub のロゴは、GitHub, Inc. の商標または登録商標です。本サイトは GitHub とは無関係です。",
    },
];
