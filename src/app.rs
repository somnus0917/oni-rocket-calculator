use leptos::prelude::*;
#[component]
pub fn App() -> impl IntoView {
    let is_oxidizer = RwSignal::new(true);
    let oxidizer_amount = RwSignal::new("".to_string());
    let fuel_amount = RwSignal::new("".to_string());
    let selected_oxidizer = RwSignal::new("".to_string());
    let selected_fuel = RwSignal::new("".to_string());

    view! {
        <label>
        "燃料量:"
        <input type="number"
            bind:value=fuel_amount
        />
        </label>
        <label>
            "是否需要氧化剂"
            <input type="checkbox"
                bind:checked=is_oxidizer
            />
        </label>
        <fieldset>
            <legend>"火箭选择"</legend>
            <label>
                "液氢引擎"
                <input
                    type="radio"
                    name="color"
                    value="liquid_hydrogen"
                    bind:group=selected_fuel
                />
            </label>
            <label>
                "蒸汽引擎"
                <input
                    type="radio"
                    name="color"
                    value="stream"
                    bind:group=selected_fuel
                />
            </label>
            <label>
                "石油引擎"
                <input
                    type="radio"
                    name="color"
                    value="petroleum"
                    bind:group=selected_fuel
                />
            </label>
        </fieldset>
        <Show when=move || is_oxidizer.get()>
        <fieldset>
            <legend>"氧化剂"</legend>
            <label>
                "液氧"
                <input
                    type="radio"
                    name="color"
                    value="liquid_oxygen"
                    bind:group=selected_oxidizer
                />
            </label>
            <label>
                "氧石"
                <input
                    type="radio"
                    name="color"
                    value="oxidizer_stone"
                    bind:group=selected_oxidizer
                />
            </label>
            <label>
                "肥料"
                <input
                    type="radio"
                    name="color"
                    value="fertilizer"
                    bind:group=selected_oxidizer
                />
            </label>
            <label>
                "\n 氧化剂量："
                <input type="number"
                    bind:value=oxidizer_amount
                />
            </label>
        </fieldset>
        </Show>
        <p>"你选择的引擎是" {selected_fuel} "."</p>
        <p>"燃料量是: " {fuel_amount}</p>
        <p>"是否需要氧化剂: " {is_oxidizer}</p>

        <Show when=move || is_oxidizer.get()>
        <p>"你选择的氧化剂是" {selected_oxidizer} "."</p>
        <p>"氧化剂量是: " {oxidizer_amount}</p>
        </Show>

    }
}
