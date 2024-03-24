use leptos::*;
use leptos_lucide::Camera;

fn main() {
    mount_to_body(|| {
        view! {
            <Camera/>
            <Camera fill="red" color="white" size=20/>
        }
    })
}
