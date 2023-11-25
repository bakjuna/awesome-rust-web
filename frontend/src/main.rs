use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html!(
        <div>
            {"Hello world!"}
        </div>
    )
}
fn main() {
    yew::start_app::<App>();
}
