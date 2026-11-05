use crate::data::types::Lang;

/// Attached image shown inside the modal.
#[derive(Clone, PartialEq, Debug)]
pub struct Seting {
    pub url: &'static str,
    pub alt: &'static str,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Work {
    pub header_image_url: &'static str,
    pub header_text: &'static str,
    pub images: &'static [Seting],
    pub languages: &'static [Lang],
    pub description_html: &'static str,
    pub is_public: bool,
    /// `(題名, url)` pairs shown under the modal title.
    pub urls: &'static [(&'static str, &'static str)],
}

pub const WORKS: &[Work] = &[
    Work {
        header_image_url: "content/programming-1_4/screenshot-2.png",
        header_text: "colab課題4",
        images: &[
            Seting {
                url: "content/programming-1_4/screenshot-1.png",
                alt: "colab課題4 スクリーンショット1",
            },
            Seting {
                url: "content/programming-1_4/screenshot-2.png",
                alt: "colab課題4 スクリーンショット2",
            },
            Seting {
                url: "content/programming-1_4/screenshot-3.png",
                alt: "colab課題4 スクリーンショット3",
            },
            Seting {
                url: "content/programming-1_4/screenshot-4.png",
                alt: "colab課題4 スクリーンショット4",
            },
        ],
        languages: &[Lang::Python],
        description_html: include_str!(concat!(env!("OUT_DIR"), "/works/programming-1_4.html")),
        is_public: true,
        urls: &[(
            "colab",
            "https://drive.google.com/file/d/1nTQzwjVNw9vu-YG5YJoKgazwJEpGblWk",
        )],
    },
    Work {
        header_image_url: "content/fes-2026/screenshot-1.jpg",
        header_text: "26年度高専祭展示",
        images: &[Seting {
            url: "content/fes-2026/screenshot-1.jpg",
            alt: "colab課題4 スクリーンショット2",
        }],
        languages: &[Lang::Svelte],
        description_html: include_str!(concat!(env!("OUT_DIR"), "/works/fes-2026.html")),
        is_public: true,
        urls: &[
            ("公開url", "https://omuct-navi.ms-fe4.workers.dev"),
            ("gitHub", "https://github.com/u0021u004Du0053/omuct-navi"),
        ],
    },
];
