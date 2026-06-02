use leptos::prelude::*;

use crate::components::ui::button::Button;
use crate::components::ui::card::{Card, CardContent, CardHeader, CardTitle};
use crate::components::ui::input::Input;
use crate::domain::auth::_users::service::login::Login;

#[component]
#[allow(non_snake_case)]
pub fn LoginPage() -> impl IntoView {
    let login_action = ServerAction::<Login>::new();

    view! {
        <div class="flex justify-center items-center min-h-screen">
            <Card class="mx-auto w-full max-w-md">
                <CardHeader>
                    <CardTitle class="text-2xl text-center">"Login"</CardTitle>
                </CardHeader>

                <CardContent>
                    <ActionForm action=login_action>
                        <div class="flex flex-col gap-4">
                            <Input
                                attr:name="user_login"
                                attr:placeholder="Your email"
                                attr:autocomplete="email"
                                attr:required=true
                            />

                            <Input
                                attr:name="password"
                                attr:r#type="password"
                                attr:placeholder="Your password"
                                attr:autocomplete="current-password"
                                attr:required=true
                            />

                            <Button attr:r#type="submit" class="w-full">
                                "Sign in"
                            </Button>
                        </div>
                    </ActionForm>
                </CardContent>
            </Card>
        </div>
    }
}
