use crate::data::types::Stack;

/// `Stack` の見た目 (Tailwind クラスとバー色)。配色は `tailwind.config.js` の `stack.*` と対応する。
pub trait StackStyle {
    /// 背景色クラス
    fn bg_class(&self) -> &'static str;
    /// 文字色クラス
    fn text_class(&self) -> &'static str;
    /// 枠線色クラス
    fn border_class(&self) -> &'static str;
    /// 熟練度(0-100)に応じたバー色 (OKLCH)。L = 0.92 - 0.24t, C = 0.03 + 0.095t
    fn bar_color(&self, level: u8) -> String;
}

impl StackStyle for Stack {
    fn bg_class(&self) -> &'static str {
        match self {
            Stack::Frontend => "bg-stack-frontend-surface",
            Stack::Backend => "bg-stack-backend-surface",
            Stack::Application => "bg-stack-application-surface",
            Stack::Infrastructure => "bg-stack-infrastructure-surface",
        }
    }

    fn text_class(&self) -> &'static str {
        match self {
            Stack::Frontend => "text-stack-frontend-fg",
            Stack::Backend => "text-stack-backend-fg",
            Stack::Application => "text-stack-application-fg",
            Stack::Infrastructure => "text-stack-infrastructure-fg",
        }
    }

    fn border_class(&self) -> &'static str {
        match self {
            Stack::Frontend => "border-stack-frontend-border",
            Stack::Backend => "border-stack-backend-border",
            Stack::Application => "border-stack-application-border",
            Stack::Infrastructure => "border-stack-infrastructure-border",
        }
    }

    fn bar_color(&self, level: u8) -> String {
        let t = f32::from(level.min(100)) / 100.0;
        format!(
            "oklch({:.4} {:.4} {})",
            0.92 - 0.24 * t,
            0.03 + 0.095 * t,
            hue(*self)
        )
    }
}

fn hue(stack: Stack) -> u16 {
    match stack {
        Stack::Frontend => 250,
        Stack::Backend => 195,
        Stack::Infrastructure => 55,
        Stack::Application => 5,
    }
}
