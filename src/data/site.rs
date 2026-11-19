/// ページ内セクションの ID。ヘッダーと各 `Section` で共有する。
pub const ABOUT_ID: &str = "about";
pub const SKILLS_ID: &str = "skills";
pub const WORKS_ID: &str = "works";
pub const CONTACT_ID: &str = "contact";

/// ヘッダーに並べるリンク `(表示名, セクションID)`。
pub const PAGE_LINKS: [(&str, &str); 4] = [
    ("About", ABOUT_ID),
    ("Skills", SKILLS_ID),
    ("Works", WORKS_ID),
    ("Contact", CONTACT_ID),
];

/// 本編ページのルーター内パス（`Router base` からの相対）。
pub const INDEX_PATH: &str = "/index.php";

/// 本編ページの絶対 URL。`/` と 404 から戻るリンクはルーター内パスだとうまく遷移しないため、こちらを使う。
pub const INDEX_URL: &str = "https://taitan291.github.io/portfolio/index.php";

pub const GITHUB_URL: &str = "https://github.com/taitan291";
