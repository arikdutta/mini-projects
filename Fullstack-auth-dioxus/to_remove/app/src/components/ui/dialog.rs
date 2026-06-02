use icons::X;
use leptos::context::Provider;
use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::*;

use crate::components::hooks::use_random::use_random_id_for;

mod components {
    use super::*;
    clx! {DialogBody, div, "flex flex-col gap-4"}
    clx! {DialogHeader, div, "flex flex-col gap-2 text-center sm:text-left"}
    clx! {DialogTitle, h3, "text-lg leading-none font-semibold"}
    clx! {DialogDescription, p, "text-muted-foreground text-sm"}
    clx! {DialogFooter, footer, "flex flex-col-reverse gap-2 sm:flex-row sm:justify-end"}
}

pub use components::*;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[derive(Clone)]
struct DialogContext {
    target_id: String,
}

#[component]
pub fn Dialog(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    let dialog_target_id = use_random_id_for("dialog");

    let ctx = DialogContext {
        target_id: dialog_target_id.clone(),
    };

    let merged_class = tw_merge!("w-fit", class);

    view! {
        <Provider value=ctx>
            <div class=merged_class data-name="__Dialog">
                {children()}
            </div>
        </Provider>
    }
}

#[component]
pub fn DialogTrigger(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    let ctx = use_context::<DialogContext>().expect("DialogTrigger must be inside a Dialog");
    let button_class = tw_merge!(CLASS_BUTTON_OUTLINE, class);
    let trigger_id = format!("trigger_{}", ctx.target_id);

    view! {
        <button class=button_class id=trigger_id tabindex="0" data-dialog-trigger=ctx.target_id>
            {children()}
        </button>
    }
}

#[component]
pub fn DialogContent(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(into, optional)] hide_close_button: Option<bool>,
    #[prop(default = true)] close_on_backdrop_click: bool,
) -> impl IntoView {
    let ctx = use_context::<DialogContext>().expect("DialogTrigger must be inside a Dialog");
    let merged_class = tw_merge!(
        // "flex flex-col gap-4", // TODO 🐛 Bug when I try to have this.. Using DialogBody instead.
        "relative bg-background border rounded-2xl shadow-lg p-6 w-full max-w-[calc(100%-2rem)] max-h-[85vh] fixed top-[50%] left-[50%] translate-x-[-50%] translate-y-[-50%] z-100 transition-all duration-200 data-[state=closed]:opacity-0 data-[state=closed]:scale-95 data-[state=open]:opacity-100 data-[state=open]:scale-100",
        class
    );

    let target_id_clone = ctx.target_id.clone();
    let backdrop_id = format!("{}_backdrop", ctx.target_id);
    let target_id_for_script = ctx.target_id.clone();
    let backdrop_id_for_script = backdrop_id.clone();
    let backdrop_behavior = if close_on_backdrop_click {
        "auto"
    } else {
        "manual"
    };

    view! {
        <script src="/hooks/lock_scroll.js"></script>

        <div
            data-name="DialogBackdrop"
            id=backdrop_id
            class="fixed inset-0 transition-opacity duration-200 pointer-events-none z-60 bg-black/50 data-[state=closed]:opacity-0 data-[state=open]:opacity-100"
            data-state="closed"
        />

        <div
            data-name="DialogContent"
            class=merged_class
            id=ctx.target_id
            data-target="target__dialog"
            data-state="closed"
            data-backdrop=backdrop_behavior
            style="pointer-events: none;"
        >
            <button
                class=format!(
                    "absolute top-4 right-4 p-1 rounded-sm focus:ring-2 focus:ring-offset-2 focus:outline-none [&_svg:not([class*='size-'])]:size-4 focus:ring-ring{}",
                    if hide_close_button.unwrap_or(false) { " hidden" } else { "" },
                )
                data-dialog-close=target_id_clone.clone()
                aria-label="Close dialog"
            >
                <span class="sr-only">"Close Dialog"</span>
                <X />
            </button>

            {children()}
        </div>

        <script>
            {format!(
                r#"
                (function() {{
                    const setupDialog = () => {{
                        const dialog = document.querySelector('#{}');
                        const backdrop = document.querySelector('#{}');
                        const trigger = document.querySelector('[data-dialog-trigger="{}"]');

                        if (!dialog || !backdrop || !trigger) {{
                            setTimeout(setupDialog, 50);
                            return;
                        }}

                        if (dialog.hasAttribute('data-initialized')) {{
                            return;
                        }}
                        dialog.setAttribute('data-initialized', 'true');

                        const openDialog = () => {{
                            // Lock scrolling
                            window.ScrollLock.lock();

                            dialog.setAttribute('data-state', 'open');
                            backdrop.setAttribute('data-state', 'open');
                            dialog.style.pointerEvents = 'auto';
                            backdrop.style.pointerEvents = 'auto';
                        }};

                        const closeDialog = () => {{
                            dialog.setAttribute('data-state', 'closed');
                            backdrop.setAttribute('data-state', 'closed');
                            dialog.style.pointerEvents = 'none';
                            backdrop.style.pointerEvents = 'none';

                            // Unlock scrolling after animation
                            window.ScrollLock.unlock(200);
                        }};

                        // Open dialog when trigger is clicked
                        trigger.addEventListener('click', openDialog);

                        // Close buttons
                        const closeButtons = dialog.querySelectorAll('[data-dialog-close]');
                        closeButtons.forEach(btn => {{
                            btn.addEventListener('click', closeDialog);
                        }});

                        // Close on backdrop click (if data-backdrop="auto")
                        backdrop.addEventListener('click', () => {{
                            if (dialog.getAttribute('data-backdrop') === 'auto') {{
                                closeDialog();
                            }}
                        }});

                        // Handle ESC key to close
                        document.addEventListener('keydown', (e) => {{
                            if (e.key === 'Escape' && dialog.getAttribute('data-state') === 'open') {{
                                e.preventDefault();
                                closeDialog();
                            }}
                        }});
                    }};

                    if (document.readyState === 'loading') {{
                        document.addEventListener('DOMContentLoaded', setupDialog);
                    }} else {{
                        setupDialog();
                    }}
                }})();
                "#,
                target_id_for_script,
                backdrop_id_for_script,
                target_id_for_script,
            )}
        </script>
    }
}

#[component]
pub fn DialogClose(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    let ctx = use_context::<DialogContext>().expect("DialogClose must be inside a Dialog");
    let button_class = tw_merge!(CLASS_BUTTON_OUTLINE, class);

    view! {
        <button class=button_class data-dialog-close=ctx.target_id aria-label="Close dialog">
            {children()}
        </button>
    }
}

#[component]
pub fn DialogAction(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    let ctx = use_context::<DialogContext>().expect("DialogClose must be inside a Dialog");
    let button_class = tw_merge!(CLASS_BUTTON_OUTLINE, class);

    view! {
        <button class=button_class data-dialog-close=ctx.target_id aria-label="Close dialog">
            {children()}
        </button>
    }
}

/* ========================================================== */
/*                     ✨ CONSTANTS ✨                        */
/* ========================================================== */

const CLASS_BUTTON_OUTLINE: &str = "px-4 py-2 h-9 inline-flex justify-center items-center text-sm font-medium whitespace-nowrap rounded-md transition-colors w-fit focus-visible:outline-hidden focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 [&_svg:not(:last-child)]:mr-2 [&_svg:not(:first-child)]:ml-2 [&_svg:not([class*='size-'])]:size-4 border bg-background border-input hover:bg-accent hover:text-accent-foreground";
