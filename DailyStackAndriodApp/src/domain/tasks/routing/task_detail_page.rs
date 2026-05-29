use leptos::prelude::*;

use crate::{
    domain::tasks::mock_tasks,
    utils::param::{ParamsUtils, PARAM},
};

#[component]
pub fn TaskDetailPage() -> impl IntoView {
    let id = ParamsUtils::extract(PARAM::ID.to_string());

    let task = Memo::new(move |_| {
        let current_id = id.get().unwrap_or_default();
        mock_tasks().into_iter().find(|t| t.id == current_id)
    });

    view! {
        <div>
            {move || match task.get() {
                None => {
                    view! { <p class="text-muted-foreground">"Task not found."</p> }.into_any()
                }
                Some(t) => {
                    view! {
                        <div class="space-y-4">
                            <div>
                                <h2 class="text-xl font-bold">{t.title}</h2>
                                <div class="flex gap-2 mt-2">
                                    <PriorityBadge priority=t.priority />
                                    <StatusBadge status=t.status />
                                </div>
                            </div>
                            <p class="text-sm text-muted-foreground leading-relaxed">{t.desc}</p>
                            <p class="text-xs font-mono text-muted-foreground/50">"id: " {t.id}</p>
                        </div>
                    }
                        .into_any()
                }
            }}
        </div>
    }
}

#[component]
fn PriorityBadge(priority: &'static str) -> impl IntoView {
    let cls = match priority {
        "High" => "bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-400",
        "Medium" => "bg-yellow-100 text-yellow-700 dark:bg-yellow-900/30 dark:text-yellow-400",
        _ => "bg-muted text-muted-foreground",
    };
    view! {
        <span class=format!("text-xs px-2 py-0.5 rounded-full font-medium {cls}")>{priority}</span>
    }
}

#[component]
fn StatusBadge(status: &'static str) -> impl IntoView {
    let cls = match status {
        "Done" => "bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400",
        "In Progress" => "bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400",
        _ => "bg-muted text-muted-foreground",
    };
    view! {
        <span class=format!("text-xs px-2 py-0.5 rounded-full font-medium {cls}")>{status}</span>
    }
}
