use leptos::prelude::*;
use w4_d18_routing_full::domain::user::routing::user_profile_page::UserProfilePage;
use w4_d18_routing_full::routing::page_home::PageHome;
use w4_d18_routing_full::utils::param::PARAM;
use w4_d18_routing_full::utils::query::QUERY;
use wasm_bindgen_test::*;

// ============================================================
// Helper Functions
// ============================================================

fn assert_contains_text<F, IV>(component: F, expected_text: &str)
where
    F: Fn() -> IV,
    IV: IntoView,
{
    let owner = Owner::new();
    owner.with(|| {
        let view = component().into_view();
        let html = view.to_html();

        assert!(
            html.contains(expected_text),
            "Component should contain '{}' but rendered: {}",
            expected_text,
            html
        );
    });
}

// ============================================================
// Page Component Content Tests
// ============================================================

#[wasm_bindgen_test]
fn test_page_home_contains_correct_content() {
    assert_contains_text(PageHome, "Home Page");
}

#[wasm_bindgen_test]
fn test_user_profile_page_contains_correct_content() {
    assert_contains_text(UserProfilePage, "User Profile Page");
}

// ============================================================
// Parameter Constants Tests (from 03-param)
// ============================================================

#[wasm_bindgen_test]
fn test_param_name_constant_has_correct_value() {
    assert_eq!(PARAM::NAME, "name", "PARAM::NAME should be 'name'");
}

#[wasm_bindgen_test]
fn test_param_id_constant_has_correct_value() {
    assert_eq!(PARAM::ID, "id", "PARAM::ID should be 'id'");
}

// ============================================================
// Query Constants Tests (from 04-query)
// ============================================================

#[wasm_bindgen_test]
fn test_query_demo_constant_has_correct_value() {
    assert_eq!(QUERY::DEMO, "demo", "QUERY::DEMO should be 'demo'");
}
