pub mod routing;

#[derive(PartialEq, Clone)]
pub struct Task {
    pub id: &'static str,
    pub title: &'static str,
    pub desc: &'static str,
    pub status: &'static str,
    pub priority: &'static str,
}

pub fn mock_tasks() -> Vec<Task> {
    vec![
        Task {
            id: "1",
            title: "Read Leptos docs",
            desc: "Go through the Leptos 0.8 routing guide and typed routes chapter. Focus on ParentRoute, Outlet, and the new Segment API.",
            status: "In Progress",
            priority: "High",
        },
        Task {
            id: "2",
            title: "Build DailyStack",
            desc: "Create a LinkedIn-worthy Leptos routing demo showcasing query params, route params, and nested routes with an outlet.",
            status: "Done",
            priority: "High",
        },
        Task {
            id: "3",
            title: "Write blog post",
            desc: "Document the routing patterns used in DailyStack. Cover use_params, use_location query, and the Outlet pattern.",
            status: "Todo",
            priority: "Medium",
        },
        Task {
            id: "4",
            title: "Review PRs",
            desc: "Review open pull requests on the team repository. Check for correctness, performance, and project conventions.",
            status: "Todo",
            priority: "Low",
        },
        Task {
            id: "5",
            title: "Refactor utils",
            desc: "Clean up the param and query utility modules. Consider extracting more generic helpers for reuse.",
            status: "In Progress",
            priority: "Medium",
        },
    ]
}
