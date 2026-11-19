use crate::data::types::{Image, Lang, Link};

/// 作品一覧の 1 項目。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Work {
    /// ページ内アンカーの ID。`content/` 内の作品ディレクトリ名と同じにする。
    pub id: &'static str,
    pub header_image_url: &'static str,
    pub header_text: &'static str,
    /// モーダル内のスライドショーに表示する画像。
    pub images: &'static [Image],
    pub languages: &'static [Lang],
    /// `build.rs` が Markdown から生成した説明文の HTML。
    pub description_html: &'static str,
    pub is_public: bool,
    /// モーダルの題名の下に表示するリンク。
    pub links: &'static [Link],
}

pub const WORKS: &[Work] = &[
    Work {
        id: "programming-1_4",
        header_image_url: "content/programming-1_4/screenshot-2.png",
        header_text: "colab課題4",
        images: &[
            Image {
                url: "content/programming-1_4/screenshot-1.png",
                alt: "colab課題4 スクリーンショット1",
            },
            Image {
                url: "content/programming-1_4/screenshot-2.png",
                alt: "colab課題4 スクリーンショット2",
            },
            Image {
                url: "content/programming-1_4/screenshot-3.png",
                alt: "colab課題4 スクリーンショット3",
            },
            Image {
                url: "content/programming-1_4/screenshot-4.png",
                alt: "colab課題4 スクリーンショット4",
            },
        ],
        languages: &[Lang::Python],
        description_html: include_str!(concat!(env!("OUT_DIR"), "/works/programming-1_4.html")),
        is_public: true,
        links: &[Link {
            label: "colab",
            url: "https://drive.google.com/file/d/1nTQzwjVNw9vu-YG5YJoKgazwJEpGblWk",
        }],
    },
    Work {
        id: "fes-2026",
        header_image_url: "content/fes-2026/screenshot-1.jpg",
        header_text: "26年度高専祭展示",
        images: &[
            Image {
                url: "content/fes-2026/screenshot-1.jpg",
                alt: "26年度高専祭展示 スクリーンショット1",
            },
            Image {
                url: "content/fes-2026/screenshot-2.png",
                alt: "26年度高専祭展示 スクリーンショット2",
            },
            Image {
                url: "content/fes-2026/screenshot-3.png",
                alt: "26年度高専祭展示 スクリーンショット3",
            },
            Image {
                url: "content/fes-2026/screenshot-4.png",
                alt: "26年度高専祭展示 スクリーンショット4",
            },
        ],
        languages: &[Lang::Svelte],
        description_html: include_str!(concat!(env!("OUT_DIR"), "/works/fes-2026.html")),
        is_public: true,
        links: &[
            Link {
                label: "公開url",
                url: "https://omuct-navi.ms-fe4.workers.dev",
            },
            Link {
                label: "gitHub",
                url: "https://github.com/u0021u004Du0053/omuct-navi",
            },
        ],
    },
];
