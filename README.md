# Rust_Webassebly Starting point

Build a Decentralized App on IPFS using WebAssembly .

### Verify Prerequisites

**Assure you have the following installed .. Otherwise , Install Rust and follow the Yew's Project Setup [here](https://yew.rs/docs/getting-started/introduction.)**

```
> cargo --version
   > cargo 1.62.0 (a748cf5a3 2022-06-08)
> rustup target list | grep 'wasm32-unknown-unknown'
   > wasm32-unknown-unknown (installed)
> trunk --version
   > trunk 0.16.0
```

## From this repo

**Clone Repo**
**trunk serve**

## From Scratch

# **Create App :**

```
$cargo new --bin counter-app
```

```
cd counter-app
```

```
cargo run
```

This should put out hello world

**Add Dependency to Cargo.toml file :**

yew = "0.19"

```
cargo update
```

Create an index.html file and paste the following :

```
<!DOCTYPE html>
<html>
<head>
  <!-- UIkit CSS -->
  <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/uikit@3.10.1/dist/css/uikit.min.css" />
  <!-- UIkit JS -->
  <script src="https://cdn.jsdelivr.net/npm/uikit@3.10.1/dist/js/uikit.min.js"></script>
  <script src="https://cdn.jsdelivr.net/npm/uikit@3.10.1/dist/js/uikit-icons.min.js"></script>
  <meta charset="utf-8" />
  <title>Counter App</title>
</head>
</html>
```

Write some rust in main.rs.

```
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

```

**Run the APP :**

`$trunk serve`

**See it in action :**
Goto : http://localhost:8080

## Fun Facts

From the Dist directory one can host this server with Python or node with the following .

```
python3 -m http.server 8080
```

```
python -m SimpleHTTPServer 8080
```

## Regarding IPFS

Check out the future of the internet in a ipfs compatable browser such as brave.

```
ipfs://bafybeicsyilnu4rxrjlerad5kzstvgio3n62qlxektwqudj4x53vaexxiu/python3 -m http.server 8080
```

Basically , Install Ipfs node and use their nice UI to upload the dist folder. Then share link and view the app live.

## Acknowledgments

_Original Article .🍕
_[Source](https://dev.to/pancy/build-a-decentralized-app-on-ipfs-using-webassembly-46a4)
