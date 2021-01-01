use yew::prelude::*;

/// A function to make the header over a badge field with a line going out to either side
pub fn badge_field_header(title: &str) -> Html {
    html!{
        <div class="badge-field-header">
            <span class="buffer"></span>
            <span class="title">{title}</span>
            <span class="buffer"></span>
        </div>
    }
}