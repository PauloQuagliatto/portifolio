use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| 0);
    let onclick = {
        let state = state.clone();
        move |_| {
            let value = *state + 1;
            state.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{"+1"}</button>
            <p>{ *state }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
