use crate::models::{EngineKind, OxidizerInput, OxidizerKind, RocketInput};
use leptos::prelude::*;
#[component]
pub fn App() -> impl IntoView {
    let oxidizer_amount = RwSignal::new("".to_string());
    let fuel_amount = RwSignal::new("".to_string());
    let oxidizer = RwSignal::new(OxidizerKind::LiquidOxygen);
    let oxidizer_name = move || oxidizer.get().spec().name;
    let engine = RwSignal::new(EngineKind::Steam);
    let engine_name = move || engine.get().spec().name;
    let requires_oxidizer = move || engine.get().spec().requires_oxidizer;
    let on_calculate = move |_| match fuel_amount.get().parse::<f32>() {
        Ok(value) => {
            let current_engine = engine.get();

            let oxidizer_input = if current_engine.spec().requires_oxidizer {
                match oxidizer_amount.get().parse::<f32>() {
                    Ok(amount) => Some(OxidizerInput {
                        oxidizer: oxidizer.get(),
                        oxidizer_amount: amount,
                    }),
                    Err(_) => {
                        leptos::logging::log!("氧化剂量输入错误");
                        return;
                    }
                }
            } else {
                None
            };
            let rocket = RocketInput {
                engine: engine.get(),
                fuel_amount: value,
                oxidizer_input: oxidizer_input,
            };
            leptos::logging::log!("燃料量: {}", value);
        }
        Err(_) => leptos::logging::log!("燃料量输入错误"),
    };

    view! {
        <label>
        "燃料量:"
        <input type="number"
            bind:value=fuel_amount
        />
        </label>
        <fieldset>
            <legend>"火箭选择"</legend>
            <label>
                "液氢引擎"
                <input
                    type="radio"
                    name="engine"
                    prop:checked=move || engine.get() == EngineKind::Hydrogen
                    on:change=move |_|{
                        engine.set(EngineKind::Hydrogen);
                    }
                />
            </label>
            <label>
                "蒸汽引擎"
                <input
                    type="radio"
                    name="engine"
                    prop:checked=move || engine.get() == EngineKind::Steam
                    on:change=move |_|{
                        engine.set(EngineKind::Steam);
                    }
                />
            </label>
        </fieldset>
        <Show when=requires_oxidizer>
        <fieldset>
            <legend>"氧化剂"</legend>
            <label>
                "液氧"
                <input
                    type="radio"
                    name="oxidizer"
                    prop:checked=move || {
                        oxidizer.get()==OxidizerKind::LiquidOxygen
                    }
                    on:change=move |_|{
                        oxidizer.set(OxidizerKind::LiquidOxygen);
                    }
                />
            </label>
            <label>
                "氧石"
                <input
                    type="radio"
                    name="oxidizer"
                    prop:checked=move || {
                        oxidizer.get()==OxidizerKind::OxyRock
                    }
                    on:change=move |_|{
                        oxidizer.set(OxidizerKind::OxyRock);
                    }
                />
            </label>
            <label>
                "肥料"
                <input
                    type="radio"
                    name="oxidizer"
                    prop:checked=move || {
                        oxidizer.get()==OxidizerKind::Fertilizer
                    }
                    on:change=move |_|{
                        oxidizer.set(OxidizerKind::Fertilizer);
                    }
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
        <p>"你选择的引擎是" {engine_name} "."</p>
        <p>"燃料量是: " {fuel_amount}</p>
        <p>"是否需要氧化剂: " {requires_oxidizer}</p>

        <Show when=requires_oxidizer>
        <p>"你选择的氧化剂是" {oxidizer_name} "."</p>
        <p>"氧化剂量是: " {oxidizer_amount}</p>
        </Show>
        <button on:click=on_calculate>
            "计算"
        </button>
    }
}
