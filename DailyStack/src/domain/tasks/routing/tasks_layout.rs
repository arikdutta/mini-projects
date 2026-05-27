use leptos::prelude::*;

#[derive(Clone)]
struct TodoItem {
    id: usize,
    title: String,
    done: RwSignal<bool>,
}

#[derive(Clone, Copy, PartialEq)]
enum Filter {
    All,
    Active,
    Done,
}

#[component]
pub fn TasksLayout() -> impl IntoView {
    let todos = RwSignal::new(vec![
        TodoItem { id: 1, title: "Set up your Rust environment".into(), done: RwSignal::new(true) },
        TodoItem { id: 2, title: "Master ownership & borrowing".into(), done: RwSignal::new(true) },
        TodoItem { id: 3, title: "Practice pattern matching".into(), done: RwSignal::new(false) },
        TodoItem { id: 4, title: "Build comfort with Result & Option".into(), done: RwSignal::new(false) },
        TodoItem { id: 5, title: "Learn cargo workflows".into(), done: RwSignal::new(false) },
        TodoItem { id: 6, title: "Write your first async program".into(), done: RwSignal::new(false) },
    ]);

    let filter = RwSignal::new(Filter::All);
    let input = RwSignal::new(String::new());
    let next_id = RwSignal::new(7usize);

    let add_todo = move || {
        let title = input.get_untracked();
        if !title.trim().is_empty() {
            let id = next_id.get_untracked();
            todos.update(|v| {
                v.push(TodoItem {
                    id,
                    title: title.trim().to_string(),
                    done: RwSignal::new(false),
                })
            });
            next_id.update(|n| *n += 1);
            input.set(String::new());
        }
    };

    let active_count =
        move || todos.with(|v| v.iter().filter(|t| !t.done.get()).count());

    let clear_completed = move || {
        todos.update(|v| v.retain(|t| !t.done.get_untracked()));
    };

    view! {
        <div class="w-full flex flex-col items-center pt-10">
            <div class="w-full max-w-md space-y-4 px-4">

                // ── Header ──────────────────────────────────────────────
                <div class="text-center pb-2">
                    <h1 class="text-4xl font-bold text-white">"Ariks Daily To Do List"</h1>
                    <p class="text-[#9ca3af] text-sm mt-1">"Stay on top of things"</p>
                </div>

                // ── Input row ───────────────────────────────────────────
                <div class="flex gap-3">
                    <input
                        type="text"
                        placeholder="What needs to be done?"
                        class="flex-1 bg-[#1a1a2e] border border-violet-500 text-white placeholder-[#6b7280] rounded-lg px-4 py-3 text-sm focus:outline-none focus:border-violet-400"
                        prop:value=move || input.get()
                        on:input=move |e| input.set(event_target_value(&e))
                        on:keydown=move |e: web_sys::KeyboardEvent| {
                            if e.key() == "Enter" {
                                add_todo();
                            }
                        }
                    />
                    <button
                        class="bg-violet-600 hover:bg-violet-700 text-white px-5 py-3 rounded-lg text-sm font-semibold transition-colors whitespace-nowrap"
                        on:click=move |_| add_todo()
                    >
                        "+ Add"
                    </button>
                </div>

                // ── Filter tabs ─────────────────────────────────────────
                <div class="bg-[#1a1a2e] rounded-lg p-1 flex gap-1">
                    {[("All", Filter::All), ("Active", Filter::Active), ("Done", Filter::Done)]
                        .into_iter()
                        .map(|(label, f)| {
                            view! {
                                <button
                                    class=move || {
                                        if filter.get() == f {
                                            "flex-1 py-2 rounded-md text-sm font-medium bg-white text-[#111128] transition-colors"
                                        } else {
                                            "flex-1 py-2 rounded-md text-sm font-medium text-[#9ca3af] hover:text-white transition-colors"
                                        }
                                    }
                                    on:click=move |_| filter.set(f)
                                >
                                    {label}
                                </button>
                            }
                        })
                        .collect_view()}
                </div>

                // ── Todo list ───────────────────────────────────────────
                <div class="space-y-2">
                    {move || {
                        todos
                            .get()
                            .into_iter()
                            .filter(|t| match filter.get() {
                                Filter::All => true,
                                Filter::Active => !t.done.get(),
                                Filter::Done => t.done.get(),
                            })
                            .map(|todo| {
                                let done = todo.done;
                                view! {
                                    <div class="flex items-center gap-3 bg-[#1a1a2e] rounded-lg px-4 py-3 border border-transparent hover:border-violet-500/60 hover:bg-violet-500/10 transition-colors cursor-pointer">
                                        <button
                                            class=move || {
                                                if done.get() {
                                                    "w-5 h-5 rounded-full flex items-center justify-center flex-shrink-0 bg-violet-600 border-2 border-violet-600 transition-colors"
                                                } else {
                                                    "w-5 h-5 rounded-full flex items-center justify-center flex-shrink-0 border-2 border-[#3a3a5a] transition-colors"
                                                }
                                            }
                                            on:click=move |_| done.update(|d| *d = !*d)
                                        >
                                            {move || {
                                                if done.get() {
                                                    view! {
                                                        <svg
                                                            xmlns="http://www.w3.org/2000/svg"
                                                            class="w-3 h-3 text-white"
                                                            viewBox="0 0 20 20"
                                                            fill="currentColor"
                                                        >
                                                            <path
                                                                fill-rule="evenodd"
                                                                d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
                                                                clip-rule="evenodd"
                                                            />
                                                        </svg>
                                                    }
                                                    .into_any()
                                                } else {
                                                    view! { <span /> }.into_any()
                                                }
                                            }}
                                        </button>
                                        <span class=move || {
                                            if done.get() {
                                                "text-sm flex-1 line-through text-[#4a4a6a]"
                                            } else {
                                                "text-sm flex-1 text-[#d1d5db]"
                                            }
                                        }>
                                            {todo.title.clone()}
                                        </span>
                                    </div>
                                }
                            })
                            .collect_view()
                    }}
                </div>

                // ── Footer ──────────────────────────────────────────────
                <div class="flex justify-between text-sm text-[#9ca3af] pt-1">
                    <span>{move || format!("{} items left", active_count())}</span>
                    <button
                        class="hover:text-white transition-colors"
                        on:click=move |_| clear_completed()
                    >
                        "Clear completed"
                    </button>
                </div>

            </div>
        </div>
    }
}
