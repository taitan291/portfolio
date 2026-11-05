/// ページ内セクションの ID。ヘッダー・目次・各 `Section` で共有する。
pub const ABOUT_ID: &str = "about";
pub const SKILLS_ID: &str = "skills";
pub const WORKS_ID: &str = "works";

/// ヘッダーと目次に並べるリンク `(表示名, セクションID)`。
pub const PAGE_LINKS: [(&str, &str); 3] = [
    ("About", ABOUT_ID),
    ("Skills", SKILLS_ID),
    ("Works", WORKS_ID),
];

pub const GITHUB_URL: &str = "https://github.com/taitan291";
