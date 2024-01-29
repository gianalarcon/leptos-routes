use leptos::*;
use leptos_router::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <h1>"Contact App"</h1>
            <nav>
                <a href="/">"Home"</a>
                <a href="/contacts">"Contacts"</a>
            </nav>
            <main>
                <Routes>
                    <Route path="/" view = || view!{
                        <h3>"Home"</h3>
                    }/>

                    <Route path="/contacts" view = ContactList>
                        <Route path=":id" view=ContactInfo>
                            <Route path="" view=|| view!{
                                <div class="tab">
                                    "(Contact info)"
                                </div>
                            }/>
                            <Route path="conversations" view=|| view!{
                                <div class="tab">
                                    "(Conversations)"
                                </div>
                            }/>
                        </Route>
                        <Route path="" view=|| view!{
                            <div class="select-user">
                                "Select a user to view contact info."
                            </div>
                        }/>
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn ContactInfo() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());
    let name = move || match id().as_str() {
        "alice" => "Alice",
        "bob" => "Bob",
        "carol" => "Carol",
        _ => "Unknown",
    };
    view! {
        <h4>{name}</h4>
        <div class="contact-info">
            <div class="tabs">
                <A href="" exact=true>"Contact Info"</A>
                <A href="conversations">"Conversations"</A>
            </div>
            <Outlet/>
        </div>
    }
}

#[component]
fn ContactList() -> impl IntoView {
    view! {
        <div class="contact-list">
            <h3>"Contacts"</h3>
            <div class="contact-list-contacts">
                <A href="alice">"Alice"</A>
                <A href="bob">"Bob"</A>
                <A href="carol">"Carol"</A>
            </div>
            <Outlet/>
        </div>
    }
}

fn main() {
    leptos::mount_to_body(App)
}
