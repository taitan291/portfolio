//! スキル・作品データで共有する型。

/// 画面に表示する名前を持つ型。
pub trait Label {
    fn label(&self) -> &'static str;
}

/// 習熟度。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Proficiency {
    /// 上級者
    Advanced,
    /// 中級者
    Intermediate,
    /// 学習中
    Learning,
    /// 初心者
    Beginner,
}

impl Label for Proficiency {
    fn label(&self) -> &'static str {
        match self {
            Proficiency::Advanced => "Advanced",
            Proficiency::Intermediate => "Intermediate",
            Proficiency::Learning => "Learning",
            Proficiency::Beginner => "Beginner",
        }
    }
}

/// 技術スタック。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Stack {
    Frontend,
    Backend,
    Application,
    Infrastructure,
}

impl Stack {
    /// 全スタック（表示順: FE → BE → APP → Infra）。
    pub const ALL: [Stack; 4] = [
        Stack::Frontend,
        Stack::Backend,
        Stack::Application,
        Stack::Infrastructure,
    ];
}

impl Label for Stack {
    fn label(&self) -> &'static str {
        match self {
            Stack::Frontend => "Frontend",
            Stack::Backend => "Backend",
            Stack::Application => "Application",
            Stack::Infrastructure => "Infrastructure",
        }
    }
}

/// 作品で使った言語・フレームワーク。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Lang {
    Rust,
    Python,
    Astro,
    Svelte,
}

impl Label for Lang {
    fn label(&self) -> &'static str {
        match self {
            Lang::Rust => "Rust",
            Lang::Python => "Python",
            Lang::Astro => "Astro",
            Lang::Svelte => "Svelte",
        }
    }
}

/// 画像（URL と代替テキスト）。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Image {
    pub url: &'static str,
    pub alt: &'static str,
}

/// 題名付きのリンク。`題名:URL` の形で表示する。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Link {
    pub label: &'static str,
    pub url: &'static str,
}
