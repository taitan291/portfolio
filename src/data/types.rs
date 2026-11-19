pub trait ChangeLabel {
    fn label(&self) -> &'static str;
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Proficiency {
    Advanced,     // 上級者
    Intermediate, // 中級者
    Learning,     // 学習中
    Beginner,     // 初心者
}

impl ChangeLabel for Proficiency {
    fn label(&self) -> &'static str {
        match self {
            Proficiency::Advanced => "Advanced",
            Proficiency::Intermediate => "Intermediate",
            Proficiency::Learning => "Learning",
            Proficiency::Beginner => "Beginner",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Stack {
    Frontend,
    Backend,
    Application,
    Infrastructure,
}

impl Stack {
    pub fn bg_class(&self) -> &'static str {
        match self {
            Stack::Frontend => "bg-stack-frontend/10",
            Stack::Backend => "bg-stack-backend/10",
            Stack::Application => "bg-stack-app/10",
            Stack::Infrastructure => "bg-stack-infra/10",
        }
    }

    pub fn text_class(&self) -> &'static str {
        match self {
            Stack::Frontend => "text-stack-frontend",
            Stack::Backend => "text-stack-backend",
            Stack::Application => "text-stack-app",
            Stack::Infrastructure => "text-stack-infra",
        }
    }

    pub fn bar_class(&self) -> &'static str {
        match self {
            Stack::Frontend => "bg-stack-frontend",
            Stack::Backend => "bg-stack-backend",
            Stack::Application => "bg-stack-app",
            Stack::Infrastructure => "bg-stack-infra",
        }
    }

    pub fn border_class(&self) -> &'static str {
        match self {
            Stack::Frontend => "border-stack-frontend/30",
            Stack::Backend => "border-stack-backend/30",
            Stack::Application => "border-stack-app/30",
            Stack::Infrastructure => "border-stack-infra/30",
        }
    }

    /// カード代表色: スタックの種類が複数ある場合、最初のものを採用
    pub fn dominant(skills: &[(Stack, Proficiency, u8)]) -> Stack {
        skills.first().map(|(s, _, _)| *s).unwrap_or(Stack::Backend)
    }
}

impl ChangeLabel for Stack {
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
