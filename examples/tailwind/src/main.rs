use leptos::*;
use leptos_lucide::Camera;

fn main() {
    mount_to_body(|| {
        view! {
            <Camera class="stroke-2 fill-none stroke-primary"/>
            <Camera class="stroke-1 fill-secondary stroke-destructive"/>
            <Camera class="stroke-1 fill-destructive stroke-primary-foreground"/>
            <Camera class="stroke-1 fill-primary stroke-primary-foreground"/>
        }
    })
}
