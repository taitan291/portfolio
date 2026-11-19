use leptos::prelude::*;

use crate::components::element::footer::GITHUB_URL;
use crate::components::element::header::{ABOUT_ID, SKILLS_ID, WORKS_ID};
use crate::components::element::skills_card::SkillsCard;
use crate::components::element::works_card::WorkCard;
use crate::components::layout::section::Section;
use crate::components::reveal::reveal_delay;
use crate::data::skills::SKILLS;
use crate::data::types::Proficiency;
use crate::data::works::WORKS;
use crate::layouts::layout::Layout;

#[component]
pub fn IndexPage() -> impl IntoView {
    view! {
        <Layout>
            <Hero />
            <Section number="01" title="About Me" tag=ABOUT_ID>
                <About />
            </Section>
            <div class="border-y border-line bg-surface/50">
                <Section number="02" title="Skills" tag=SKILLS_ID lead="これまでに触れてきた言語・ツール。カードを選ぶと詳細と関連作品を表示します。">
                    <div class="skills-grid grid gap-4">
                        {SKILLS.iter().enumerate().map(|(index, skill)| view! {
                            // Stagger within a row (at most 4 columns) so later rows don't lag behind.
                            <div data-reveal="" style=reveal_delay(index % 4)>
                                <SkillsCard name=skill.name image_url=skill.image_url skills=skill.skills.to_vec() credit=skill.credit lang=skill.lang />
                            </div>
                        }).collect_view()}
                    </div>
                </Section>
            </div>
            <Section number="03" title="Works" tag=WORKS_ID lead="授業の課題や個人で取り組んだ制作物です。">
                <div class="grid gap-6 md:grid-cols-2">
                    {WORKS.iter().enumerate().map(|(index, work)| view! {
                        <div data-reveal="" style=reveal_delay(index % 2 * 2)>
                            <WorkCard work=*work index=index />
                        </div>
                    }).collect_view()}
                </div>
            </Section>
        </Layout>
    }
}

#[component]
fn Hero() -> impl IntoView {
    let learning = SKILLS
        .iter()
        .filter(|skill| {
            skill
                .skills
                .iter()
                .any(|(_, proficiency, _)| *proficiency == Proficiency::Learning)
        })
        .map(|skill| skill.name)
        .collect::<Vec<_>>()
        .join(" / ");
    let stats = [
        (format!("{:02}", SKILLS.len()), "Skills"),
        (format!("{:02}", WORKS.len()), "Works"),
        ("2nd".to_string(), "Year at KOSEN"),
    ];

    view! {
        <section class="relative isolate overflow-hidden">
            <div class="hero-grid absolute inset-0 -z-10 animate-fade-in" aria-hidden="true" />
            <div class="hero-glow absolute -right-24 -top-24 -z-10 h-[28rem] w-[28rem] animate-drift rounded-full" aria-hidden="true" />
            <div class="container-page grid items-center gap-12 pb-20 pt-16 sm:pb-28 sm:pt-24 lg:grid-cols-[minmax(0,1fr)_auto] lg:gap-16">
                <div>
                    <p class="inline-flex animate-fade-up items-center gap-2 rounded-full border border-line bg-surface/70 px-3 py-1 font-mono text-xs text-muted backdrop-blur">
                        <span class="relative flex h-2 w-2">
                            <span class="absolute inline-flex h-full w-full animate-ping rounded-full bg-accent opacity-75" />
                            <span class="relative inline-flex h-2 w-2 rounded-full bg-accent" />
                        </span>
                        "Currently learning: "{learning}
                    </p>
                    <p class="mt-8 animate-fade-up font-mono text-sm text-accent-text [animation-delay:80ms]">
                        "// Hello, I'm"
                    </p>
                    // Letters are split for the staggered entrance; the label keeps the name readable as one word.
                    <h1 aria-label="taitan" class="mt-2 text-7xl font-bold leading-none tracking-tighter sm:text-8xl lg:text-9xl">
                        {"taitan".chars().enumerate().map(|(index, letter)| view! {
                            <span
                                aria-hidden="true"
                                class="inline-block animate-letter-rise"
                                style=format!("animation-delay: {}ms", 140 + index * 45)
                            >
                                {letter.to_string()}
                            </span>
                        }).collect_view()}
                        <span aria-hidden="true" class="animate-blink text-accent-text">"_"</span>
                    </h1>
                    <p class="mt-8 max-w-2xl animate-fade-up text-lg text-muted [animation-delay:220ms] sm:text-xl">
                        "高専2年・情報系の学生です。"<br/>
                        <span class="text-fg">"Webフロントエンド"</span>"や"<span class="text-fg">"Rust"</span>"を中心に、作りながら学んでいます。"
                    </p>
                    <div class="mt-10 flex animate-fade-up flex-wrap gap-3 [animation-delay:300ms]">
                        <a href=format!("#{WORKS_ID}") class="group btn-primary">
                            "作品を見る"
                            <span aria-hidden="true" class="transition-transform duration-300 group-hover:translate-x-1">"→"</span>
                        </a>
                        <a href=GITHUB_URL target="_blank" rel="noopener noreferrer" class="group btn-ghost">
                            "GitHub"
                            <span aria-hidden="true" class="transition-transform duration-300 group-hover:-translate-y-0.5 group-hover:translate-x-0.5">"↗"</span>
                        </a>
                    </div>
                    <dl class="mt-14 grid max-w-lg animate-fade-up grid-cols-3 divide-x divide-line border-y border-line [animation-delay:380ms]">
                        {stats.into_iter().map(|(value, label)| view! {
                            <div class="px-4 py-4 first:pl-0">
                                <dt class="font-mono text-[0.7rem] uppercase tracking-wider text-muted">{label}</dt>
                                <dd class="mt-1 text-3xl font-bold tracking-tight">{value}</dd>
                            </div>
                        }).collect_view()}
                    </dl>
                </div>
                <div class="mx-auto hidden w-64 animate-fade-up [animation-delay:200ms] sm:block lg:w-80">
                    <div class="relative animate-float">
                        <div class="absolute -inset-3 rotate-6 animate-sway rounded-[2rem] border border-dashed border-accent-text/50" aria-hidden="true" />
                        <img
                            src="assets/my_icon.avif"
                            alt="taitanのアイコン"
                            width="320"
                            height="320"
                            decoding="async"
                            class="relative aspect-square w-full rounded-[2rem] border border-line object-cover shadow-2xl"
                        />
                        <span class="absolute -bottom-4 -left-4 rounded-full bg-accent px-4 py-2 font-mono text-xs font-medium text-accent-on shadow-lg">
                            "@taitan291"
                        </span>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn About() -> impl IntoView {
    let profile = [
        ("Name", "taitan"),
        ("School", "高専 2年 / 情報系"),
        ("Interests", "Web Frontend, Rust"),
        ("GitHub", "@taitan291"),
    ];

    view! {
        <div class="grid gap-10 lg:grid-cols-[minmax(0,1.4fr)_minmax(0,1fr)] lg:gap-16">
            <div data-reveal="" class="space-y-6 text-lg leading-loose text-muted">
                <p>
                    "高専2年、情報系所属の"<span class="font-bold text-fg">"taitan"</span>"です。"
                </p>
                <p>
                    "現在は"<span class="text-fg">"Webフロントエンド"</span>"や"<span class="text-fg">"Rust"</span>
                    "などを勉強中です。このサイトも Rust（Leptos）と WebAssembly で作っています。"
                </p>
            </div>
            // Wrapped so the reveal transition doesn't fight the card's own transition.
            <div data-reveal="" style=reveal_delay(2)>
                <dl class="card divide-y divide-line">
                    {profile.into_iter().map(|(label, value)| view! {
                        <div class="flex flex-wrap items-baseline justify-between gap-x-6 gap-y-1 px-6 py-4">
                            <dt class="font-mono text-xs uppercase tracking-wider text-muted">{label}</dt>
                            <dd class="font-medium">{value}</dd>
                        </div>
                    }).collect_view()}
                </dl>
            </div>
        </div>
    }
}
