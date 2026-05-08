/// Attached image shown inside the modal.
#[derive(Clone, PartialEq, Debug)]
pub struct Seting {
    pub url: &'static str,
    pub alt: &'static str,
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

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Work {
    pub header_image_url: &'static str,
    pub header_text: &'static str,
    pub images: &'static [Seting],
    pub languages: &'static [Lang],
    pub description_html: &'static str,
    pub is_public: bool,
    pub url: Option<&'static str>,
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
        url: Some("https://drive.google.com/file/d/1nTQzwjVNw9vu-YG5YJoKgazwJEpGblWk"),
    },
    Work {
        header_image_url: "assets/my_icon.avif",
        header_text: "26年度高専祭展示",
        images: &[],
        languages: &[Lang::Svelte],
        description_html: include_str!(concat!(env!("OUT_DIR"), "/works/fes-2026.html")),
        is_public: false,
        url: None,
    },
];
