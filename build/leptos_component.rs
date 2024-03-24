#[cfg(feature = "__component_name")]
#[component]
pub fn __component_fn(
    #[prop(optional, into)] class: Option<&'static str>,
    #[prop(default = 24, into)] size: i32,
    #[prop(default = "none", into)] fill: &'static str,
    #[prop(default = "currentColor", into)] color: &'static str,
    #[prop(default = 2, into)] stroke_width: u32,
    #[prop(default = "round", into)] stroke_linecap: &'static str,
    #[prop(default = "round", into)] stroke_linejoin: &'static str,
) -> impl IntoView {
    view! {
        <svg
            class={class}
            fill={fill}
            width={size}
            height={size}
            stroke={color}
            stroke-width={stroke_width}
            stroke-linecap={stroke_linecap}
            stroke-linejoin={stroke_linejoin}
            viewBox="0 0 24 24"
        >
            __icon_paths
        </svg>
    }
}
