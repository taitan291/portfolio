pub trait Label {
    fn label(&self) -> &'static str;
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Proficiency {
    Advanced,     // 上級者
    Intermediate, // 中級者
    Learning,     // 学習中
    Beginner,     // 初心者
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Stack {
    Frontend,
    Backend,
    Application,
    Infrastructure,
}

impl Stack {
    /// カード代表色: スタックの種類が複数ある場合、最初のものを採用
    pub fn dominant(skills: &[(Stack, Proficiency, u8)]) -> Stack {
        skills.first().map(|(s, _, _)| *s).unwrap_or(Stack::Backend)
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

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Lang {
    Rust,
    Python,
    Astro,
    Svelte,
}

impl Lang {
    pub fn label(&self) -> &'static str {
        match self {
            Lang::Rust => "Rust",
            Lang::Python => "Python",
            Lang::Astro => "Astro",
            Lang::Svelte => "Svelte",
        }
    }
}
