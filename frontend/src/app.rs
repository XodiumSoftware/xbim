#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

use crate::User;
use gloo::net::http::Request;
use yew::platform::spawn_local;
use yew::{function_component, use_state, Callback, Html};

#[function_component(App)]
fn app() -> Html {
    let user_state = use_state(|| ("".to_string(), "".to_string(), None as Option<i32>));
    let message = use_state(|| "".to_string());
    let users = use_state(Vec::new);

    let get_users = {
        let users = users.clone();
        let message = message.clone();
        Callback::from(move |_| {
            let users = users.clone();
            let message = message.clone();
            spawn_local(async move {
                match Request::get("http://127.0.0.1:8000/api/users").send().await {
                    Ok(resp) if resp.ok() => {
                        let fetched_users: Vec<User> = resp.json().await.unwrap_or_default();
                        users.set(fetched_users);
                    }

                    _ => message.set("Failed to fetch users".into()),
                }
            });
        })
    };

    let create_user = {
        let user_state = user_state.clone();
        let message = message.clone();
        let get_users = get_users.clone();
        Callback::from(move |_| {
            let (name, email, _) = (*user_state).clone();
            let user_state = user_state.clone();
            let message = message.clone();
            let get_users = get_users.clone();

            spawn_local(async move {
                let user_data = serde_json::json!({ "name": name, "email": email });

                let response = Request::post("http://127.0.0.1:8000/api/users")
                    .header("Content-Type", "application/json")
                    .body(user_data.to_string())
                    .expect("REASON")
                    .send()
                    .await;

                match response {
                    Ok(resp) if resp.ok() => {
                        message.set("User created successfully".into());
                        get_users.emit(());
                    }

                    _ => message.set("Failed to create user".into()),
                }

                user_state.set(("".to_string(), "".to_string(), None));
            });
        })
    };

    let update_user = {
        let user_state = user_state.clone();
        let message = message.clone();
        let get_users = get_users.clone();

        Callback::from(move |_| {
            let (name, email, editing_user_id) = (*user_state).clone();
            let user_state = user_state.clone();
            let message = message.clone();
            let get_users = get_users.clone();

            if let Some(id) = editing_user_id {
                spawn_local(async move {
                    let response = Request::put(&format!("http://127.0.0.1:8000/api/users/{}", id))
                        .header("Content-Type", "application/json")
                        .body(serde_json::to_string(&(id, name.as_str(), email.as_str())).unwrap())
                        .send()
                        .await;

                    match response {
                        Ok(resp) if resp.ok() => {
                            message.set("User updated successfully".into());
                            get_users.emit(());
                        }

                        _ => message.set("Failed to update user".into()),
                    }

                    user_state.set(("".to_string(), "".to_string(), None));
                });
            }
        })
    };

    let delete_user = {
        let message = message.clone();
        let get_users = get_users.clone();

        Callback::from(move |id: i32| {
            let message = message.clone();
            let get_users = get_users.clone();

            spawn_local(async move {
                let response = Request::delete(&format!("http://127.0.0.1:8000/api/users/{}", id))
                    .send()
                    .await;

                match response {
                    Ok(resp) if resp.ok() => {
                        message.set("User deleted successfully".into());
                        get_users.emit(());
                    }

                    _ => message.set("Failed to delete user".into()),
                }
            });
        })
    };

    let edit_user = {
        let user_state = user_state.clone();
        let users = users.clone();

        Callback::from(move |id: i32| {
            if let Some(user) = users.iter().find(|u| u.id == id) {
                user_state.set((user.name.clone(), user.email.clone(), Some(id)));
            }
        })
    };
}
