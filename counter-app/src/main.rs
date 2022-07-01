/// Import all goodies from Yew.
use yew::prelude::*;

//Define Input Struct that has one state and is a use_state handle of type unsigned 64bit int
#[derive(Properties, PartialEq)]
struct TodoProps {
    state: UseStateHandle<u64>,
}
//take the TodoProps reference as the Parameter
#[function_component(Counter)]
fn counter(props: &TodoProps) -> Html {
    //Define Callback for increment
    let increment = {
        let _state = props.state.clone();
        Callback::from(move |_| _state.set(*_state + 1))
    };
    let decrement = {
        let state = props.state.clone();
        Callback::from(move |_| state.set(*state - 1))
    };

    let reset = {
        let state = props.state.clone();
        Callback::from(move |_| state.set(0))
    };
    html! {
        <div class="uk-position-center uk-text-center">
        <button
            onclick={increment}
            class="uk-button uk-button-primary uk-button-large"
        >
            { "+1" }
        </button>
        <button
            onclick={reset}
            class="uk-button uk-button-primary uk-button-large"
        >
            { "0" }
        </button>
        <button
            onclick={decrement}
            class="uk-button uk-button-primary uk-button-large"
        >
            { "-1" }
    </button>
        <p>{ *props.state }</p>
    </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    //define intial state as zero.
    let state = use_state(|| 0 as u64);
    html! {
        <Counter {state}/>
    }
}

/// Start the Yew app
fn main() {
    yew::start_app::<App>();
}
