use leptos::html::Div;
use leptos::prelude::*;

use crate::components::ui::zoom_image::ZoomImage;
use crate::data::types::Image;

/// 前後のボタンにのせるクラス（位置の `left-2` / `right-2` は個別に付ける）。
const NAV_BUTTON: &str = "absolute top-1/2 z-10 flex min-h-11 min-w-11 -translate-y-1/2 items-center justify-center rounded-full bg-black/50 text-xl text-white transition-colors hover:bg-black/70";

/// スワイプできるスライドショー（CSS scroll-snap）。前後ボタン・ドット・矢印キーで切り替える。
/// スライドをクリックすると [`ZoomImage`] で拡大表示する。
#[component]
pub fn Slideshow(images: &'static [Image]) -> impl IntoView {
    let track = NodeRef::<Div>::new();
    let current = RwSignal::new(0usize);
    let count = images.len();

    let go_to = move |index: usize| {
        if let Some(track) = track.get() {
            track.set_scroll_left(index as i32 * track.client_width());
        }
        current.set(index);
    };
    let prev = move || go_to((current.get_untracked() + count - 1) % count);
    let next = move || go_to((current.get_untracked() + 1) % count);

    view! {
        <section
            aria-roledescription="carousel"
            aria-label="画像スライドショー"
            class="mt-4"
        >
            <div class="relative">
                <div
                    node_ref=track
                    tabindex="0"
                    class="slideshow-track flex aspect-video snap-x snap-mandatory overflow-x-auto scroll-smooth rounded-lg bg-site-subtle"
                    on:scroll=move |_| {
                        if let Some(track) = track.get() {
                            let width = track.client_width();
                            if width > 0 {
                                let index = (f64::from(track.scroll_left()) / f64::from(width)).round() as usize;
                                current.set(index.min(count - 1));
                            }
                        }
                    }
                    on:keydown=move |event| match event.key().as_str() {
                        "ArrowLeft" => { event.prevent_default(); prev(); }
                        "ArrowRight" => { event.prevent_default(); next(); }
                        _ => {}
                    }
                >
                    {images.iter().enumerate().map(|(i, image)| view! {
                        <div
                            role="group"
                            aria-roledescription="slide"
                            aria-label=format!("{} / {}", i + 1, count)
                            class="h-full w-full shrink-0 snap-center"
                        >
                            <ZoomImage src=image.url alt=image.alt class="h-full" />
                        </div>
                    }).collect_view()}
                </div>
                {(count > 1).then(|| view! {
                    <button
                        type="button"
                        aria-label="前の画像"
                        class=format!("{NAV_BUTTON} left-2")
                        on:click=move |_| prev()
                    >"‹"</button>
                    <button
                        type="button"
                        aria-label="次の画像"
                        class=format!("{NAV_BUTTON} right-2")
                        on:click=move |_| next()
                    >"›"</button>
                })}
            </div>
            {(count > 1).then(|| view! {
                <div class="mt-2 flex items-center justify-center gap-1">
                    {(0..count).map(|i| view! {
                        <button
                            type="button"
                            aria-label=format!("{}枚目を表示", i + 1)
                            aria-current=move || (current.get() == i).then_some("true")
                            class="group flex min-h-6 min-w-6 items-center justify-center"
                            on:click=move |_| go_to(i)
                        >
                            <span
                                class="block h-2 w-2 rounded-full transition-colors"
                                class=("bg-site-accent", move || current.get() == i)
                                class=("bg-site-border-strong/50", move || current.get() != i)
                            ></span>
                        </button>
                    }).collect_view()}
                </div>
            })}
        </section>
    }
}
