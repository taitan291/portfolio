use std::cmp::Ordering;

use crate::data::skills::Skill;
use crate::data::types::{Label, Stack};

/// 並べ替えキー
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SortKey {
    Proficiency,
    Name,
    /// スタック順(FE→BE→APP→Infra)でまとめ、その中で習熟度順(Defaultモード用)
    StackProficiency,
}

/// 昇順 / 降順
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SortOrder {
    Asc,
    Desc,
}

/// 表示モード
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    Default,
    Single(Stack),
    Custom,
}

/// 全モード共通の状態: 段(スタックのまとまり)のリストと、ソート設定。
#[derive(Clone, PartialEq, Debug)]
pub struct SortConfig {
    pub groups: Vec<Vec<Stack>>,
    pub key: SortKey,
    pub order: SortOrder,
}

/// 並び順が固定の全スタック
pub const ALL_STACKS: [Stack; 4] = [
    Stack::Frontend,
    Stack::Backend,
    Stack::Application,
    Stack::Infrastructure,
];

impl SortConfig {
    /// モードとカスタム設定から `SortConfig` を作る。
    pub fn from_mode(mode: Mode, custom_stacks: &[Stack], key: SortKey, order: SortOrder) -> Self {
        match mode {
            Mode::Default => Self {
                groups: vec![
                    vec![Stack::Frontend, Stack::Backend, Stack::Application],
                    vec![Stack::Infrastructure],
                ],
                key: SortKey::StackProficiency,
                order: SortOrder::Desc,
            },
            Mode::Single(stack) => Self {
                groups: vec![vec![stack]],
                key: SortKey::Proficiency,
                order: SortOrder::Desc,
            },
            Mode::Custom => Self {
                // チェック順に関わらず fe/be/ap/if の順で並べる
                groups: ALL_STACKS
                    .iter()
                    .filter(|s| custom_stacks.contains(s))
                    .map(|s| vec![*s])
                    .collect(),
                key,
                order,
            },
        }
    }
}

/// 段の中の1カード
#[derive(Clone, Copy)]
pub struct Entry {
    pub skill: &'static Skill,
    /// カードの色に使うスタック(その段のスタックのうち習熟度が最大のもの)
    pub stack: Stack,
    /// その段での習熟度
    pub level: u8,
}

/// 1段分の表示内容
pub struct Row {
    /// 見出し(スタック名を「 / 」で連結)
    pub heading: String,
    /// 見出しに色を付けるためのスタック一覧
    pub stacks: Vec<Stack>,
    pub entries: Vec<Entry>,
}

fn heading(stacks: &[Stack]) -> String {
    stacks
        .iter()
        .map(|s| s.label())
        .collect::<Vec<_>>()
        .join(" / ")
}

/// スキルをその段のスタックで評価する。該当スタックがなければ `None`。
fn entry_for(skill: &'static Skill, stacks: &[Stack]) -> Option<Entry> {
    // 同値なら先に出てくるスタックを採用するため max_by_key ではなく手動で比較
    let mut best: Option<(Stack, u8)> = None;
    for (stack, _, level) in skill.skills {
        if !stacks.contains(stack) {
            continue;
        }
        if best.is_none_or(|(_, l)| *level > l) {
            best = Some((*stack, *level));
        }
    }
    best.map(|(stack, level)| Entry {
        skill,
        stack,
        level,
    })
}

fn stack_rank(stack: Stack) -> usize {
    ALL_STACKS
        .iter()
        .position(|s| *s == stack)
        .unwrap_or(usize::MAX)
}

fn compare(a: &Entry, b: &Entry, key: SortKey, order: SortOrder) -> Ordering {
    let primary = match key {
        SortKey::Proficiency => a.level.cmp(&b.level),
        SortKey::Name => a.skill.name.cmp(b.skill.name),
        SortKey::StackProficiency => {
            // スタック順は常に FE→BE→APP、習熟度は order に従う
            let level_ord = match order {
                SortOrder::Asc => a.level.cmp(&b.level),
                SortOrder::Desc => b.level.cmp(&a.level),
            };
            return stack_rank(a.stack)
                .cmp(&stack_rank(b.stack))
                .then(level_ord)
                .then_with(|| a.skill.name.cmp(b.skill.name));
        }
    };
    let primary = match order {
        SortOrder::Asc => primary,
        SortOrder::Desc => primary.reverse(),
    };
    // 同値のときは常に name 昇順
    primary.then_with(|| a.skill.name.cmp(b.skill.name))
}

/// 設定に従って段ごとにスキルを並べ替える(純粋関数)。
pub fn build_rows(skills: &'static [Skill], config: &SortConfig) -> Vec<Row> {
    config
        .groups
        .iter()
        .map(|stacks| {
            let mut entries: Vec<Entry> = skills
                .iter()
                .filter_map(|skill| entry_for(skill, stacks))
                .collect();
            entries.sort_by(|a, b| compare(a, b, config.key, config.order));
            Row {
                heading: heading(stacks),
                stacks: stacks.clone(),
                entries,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::skills::SKILLS;

    fn names(row: &Row) -> Vec<&'static str> {
        row.entries.iter().map(|e| e.skill.name).collect()
    }

    #[test]
    fn default_has_two_rows() {
        let cfg = SortConfig::from_mode(Mode::Default, &[], SortKey::Name, SortOrder::Asc);
        let rows = build_rows(SKILLS, &cfg);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].heading, "Frontend / Backend / Application");
        assert_eq!(rows[1].heading, "Infrastructure");
        // FE→BE→APP の順にまとめ、各スタック内は習熟度の降順。Rust は fe 40 / ap 30 で FE 扱い
        assert_eq!(
            names(&rows[0]),
            ["Astro", "Svelte", "Rust", "Python", "Arduino"]
        );
        assert_eq!(names(&rows[1]), ["Git", "GitHub"]);
    }

    #[test]
    fn multi_stack_skill_appears_in_each_row() {
        let cfg = SortConfig::from_mode(
            Mode::Custom,
            &[Stack::Application, Stack::Frontend],
            SortKey::Proficiency,
            SortOrder::Desc,
        );
        let rows = build_rows(SKILLS, &cfg);
        assert_eq!(rows[0].heading, "Frontend");
        assert_eq!(rows[1].heading, "Application");
        assert!(names(&rows[0]).contains(&"Rust"));
        assert!(names(&rows[1]).contains(&"Rust"));
        let rust_fe = rows[0].entries.iter().find(|e| e.skill.name == "Rust");
        assert_eq!(rust_fe.map(|e| e.level), Some(40));
        let rust_ap = rows[1].entries.iter().find(|e| e.skill.name == "Rust");
        assert_eq!(rust_ap.map(|e| e.level), Some(30));
    }

    #[test]
    fn custom_name_order() {
        let cfg = SortConfig::from_mode(
            Mode::Custom,
            &[Stack::Application],
            SortKey::Name,
            SortOrder::Desc,
        );
        let rows = build_rows(SKILLS, &cfg);
        assert_eq!(names(&rows[0]), ["Rust", "Python", "Arduino"]);
    }

    #[test]
    fn custom_proficiency_asc_ties_by_name() {
        let cfg = SortConfig::from_mode(
            Mode::Custom,
            &[Stack::Application],
            SortKey::Proficiency,
            SortOrder::Asc,
        );
        let rows = build_rows(SKILLS, &cfg);
        assert_eq!(names(&rows[0]), ["Arduino", "Rust", "Python"]);
    }

    #[test]
    fn custom_empty_has_no_rows() {
        let cfg = SortConfig::from_mode(Mode::Custom, &[], SortKey::Name, SortOrder::Asc);
        assert!(build_rows(SKILLS, &cfg).is_empty());
    }

    #[test]
    fn single_mode_one_row() {
        let cfg = SortConfig::from_mode(
            Mode::Single(Stack::Infrastructure),
            &[],
            SortKey::Name,
            SortOrder::Asc,
        );
        let rows = build_rows(SKILLS, &cfg);
        assert_eq!(rows.len(), 1);
        assert_eq!(names(&rows[0]), ["Git", "GitHub"]);
    }
}
