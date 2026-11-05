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
            Stack::Frontend => "bg-stack-frontend-bg",
            Stack::Backend => "bg-stack-backend-bg",
            Stack::Application => "bg-stack-app-bg",
            Stack::Infrastructure => "bg-stack-infra-bg",
        }
    }

    pub fn text_class(&self) -> &'static str {
        match self {
            Stack::Frontend => "text-stack-frontend-text",
            Stack::Backend => "text-stack-backend-text",
            Stack::Application => "text-stack-app-text",
            Stack::Infrastructure => "text-stack-infra-text",
        }
    }

    pub fn border_class(&self) -> &'static str {
        match self {
            Stack::Frontend => "border-stack-frontend-border",
            Stack::Backend => "border-stack-backend-border",
            Stack::Application => "border-stack-app-border",
            Stack::Infrastructure => "border-stack-infra-border",
        }
    }

    fn hue(&self) -> u16 {
        match self {
            Stack::Frontend => 250,
            Stack::Backend => 195,
            Stack::Infrastructure => 55,
            Stack::Application => 5,
        }
    }

    /// 熟練度(0-100)に応じたバー色 (OKLCH)。L = 0.92 - 0.24t, C = 0.03 + 0.095t
    pub fn bar_color(&self, level: u8) -> String {
        let t = f32::from(level.min(100)) / 100.0;
        format!(
            "oklch({:.4} {:.4} {})",
            0.92 - 0.24 * t,
            0.03 + 0.095 * t,
            self.hue()
        )
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
