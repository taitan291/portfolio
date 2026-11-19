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

    /// 淡い背景色のクラス。
    pub fn bg_class(&self) -> &'static str {
        match self {
            Stack::Frontend => "bg-stack-frontend/10",
            Stack::Backend => "bg-stack-backend/10",
            Stack::Application => "bg-stack-app/10",
            Stack::Infrastructure => "bg-stack-infra/10",
        }
    }

    /// 文字色のクラス。
    pub fn text_class(&self) -> &'static str {
        match self {
            Stack::Frontend => "text-stack-frontend",
            Stack::Backend => "text-stack-backend",
            Stack::Application => "text-stack-app",
            Stack::Infrastructure => "text-stack-infra",
        }
    }

    /// 習熟度バーの色のクラス。
    pub fn bar_class(&self) -> &'static str {
        match self {
            Stack::Frontend => "bg-stack-frontend",
            Stack::Backend => "bg-stack-backend",
            Stack::Application => "bg-stack-app",
            Stack::Infrastructure => "bg-stack-infra",
        }
    }

    /// 枠線の色のクラス。
    pub fn border_class(&self) -> &'static str {
        match self {
            Stack::Frontend => "border-stack-frontend/30",
            Stack::Backend => "border-stack-backend/30",
            Stack::Application => "border-stack-app/30",
            Stack::Infrastructure => "border-stack-infra/30",
        }
    }
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
