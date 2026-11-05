use leptos::prelude::*;

use crate::components::cards::works_card::WorkCard;
use crate::components::layout::page_layout::PageLayout;
use crate::components::layout::section::Section;
use crate::components::layout::toc::Toc;
use crate::components::skills_view::SkillsView;
use crate::data::site::{ABOUT_ID, PAGE_LINKS, WORKS_ID};
use crate::data::works::WORKS;

#[component]
pub fn IndexPage() -> impl IntoView {
    view! {
        <PageLayout>
            <div class="py-12 sm:py-16">
                <div class="flex items-stretch gap-4">
                    // <img src="assets/my_icon.avif" alt="taitanのアイコン" class="aspect-square self-stretch rounded-xl object-cover" />
                    <div>
                        <h1 class="text-4xl font-bold tracking-tight text-site-accent sm:text-5xl">"taitan"</h1>
                        <p class="mt-4 text-site-muted">"高専2年 情報系所属"</p>
                    </div>
                </div>
            </div>
            <div class="lg:grid lg:grid-cols-[16rem_minmax(0,1fr)] lg:gap-10 xl:gap-16">
                <div class="lg:sticky lg:top-6 lg:self-start">
                    <Section title="Table of contents" tag="contents">
                        <nav aria-label="目次" class="grid gap-3 sm:grid-cols-2 lg:grid-cols-1">
                            {PAGE_LINKS.into_iter().map(|(label, id)| view! {
                                <Toc title=label tag=format!("#{id}") />
                            }).collect_view()}
                        </nav>
                    </Section>
                </div>
                <div class="min-w-0">
                    <Section title="About Me" tag=ABOUT_ID>
                        <p>"高専2年 情報系所属のtaitanです。"<br/>"現在はwebフロントエンドやRustなどの勉強中です。"</p>
                    </Section>
                    <SkillsView />
                    <Section title="Works" tag=WORKS_ID>
                        <div class="grid gap-3 sm:grid-cols-2">
                            {WORKS.iter().map(|work| view! { <WorkCard work=*work /> }).collect_view()}
                        </div>
                    </Section>
                </div>
            </div>
        </PageLayout>
    }
}
