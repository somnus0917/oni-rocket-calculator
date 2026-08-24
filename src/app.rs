use crate::calculator::{CalculatorInput, CalculatorResult, calculate};
use crate::models::{
    EngineKind, FuelStorage, FuelTankKind, OxidizerInput, OxidizerKind, OxidizerTankKind,
    RocketInput, RocketModule,
};
use leptos::{ev::MouseEvent, prelude::*};
#[component]
pub fn App() -> impl IntoView {
    let oxidizer_amount = RwSignal::new("".to_string());
    let fuel_amount = RwSignal::new("".to_string());
    let oxidizer = RwSignal::new(OxidizerKind::LiquidOxygen);
    let oxidizer_tank = RwSignal::new(OxidizerTankKind::Liquid);
    let oxidizer_tank_count_input = RwSignal::new("1".to_string());
    let oxidizer_name = move || oxidizer.get().spec().name;
    let engine = RwSignal::new(EngineKind::Steam);
    let engine_name = move || engine.get().spec().name;
    let requires_oxidizer = move || engine.get().spec().requires_oxidizer;
    let result = RwSignal::new(None::<CalculatorResult>);
    let fuel_tank_count_input = RwSignal::new("1".to_string());
    let requires_external_fuel_tank =
        move || matches!(engine.get().spec().fuel_storage, FuelStorage::ExternalTank);
    let on_calculate = move |_: MouseEvent| {
        result.set(None);
        match fuel_amount.get().parse::<f32>() {
            Ok(value) => {
                let current_engine = engine.get();
                let current_oxidizer = oxidizer.get();
                let mut modules = Vec::new();

                // 燃料舱
                match current_engine.spec().fuel_storage {
                    FuelStorage::Internal { .. } => {}
                    FuelStorage::ExternalTank => {
                        let fuel_tank_count = match fuel_tank_count_input.get().parse::<usize>() {
                            Ok(count) => count,
                            Err(_) => return,
                        };
                        for _ in 0..fuel_tank_count {
                            modules.push(RocketModule::FuelTank(FuelTankKind::LargeLiquid));
                        }
                    }
                }

                // 氧化剂舱
                let oxidizer_input = if current_engine.spec().requires_oxidizer {
                    match oxidizer_amount.get().parse::<f32>() {
                        Ok(amount) => {
                            let oxidizer_tank_count =
                                match oxidizer_tank_count_input.get().parse::<usize>() {
                                    Ok(count) => count,
                                    Err(_) => return,
                                };
                            for _ in 0..oxidizer_tank_count {
                                modules.push(RocketModule::OxidizerTank(oxidizer_tank.get()));
                            }
                            Some(OxidizerInput {
                                oxidizer: current_oxidizer,
                                oxidizer_amount: amount,
                            })
                        }
                        Err(_) => {
                            return;
                        }
                    }
                } else {
                    None
                };

                // 火箭本体
                let rocket = RocketInput {
                    engine: current_engine,
                    fuel_amount: value,
                    oxidizer_input,
                    modules,
                };
                leptos::logging::log!("燃料量: {}", value);
                let input = CalculatorInput { rocket };
                let calculated = calculate(input);

                match calculated {
                    Ok(calculated) => {
                        result.set(Some(calculated));
                    }

                    Err(error) => {
                        leptos::logging::log!("计算错误: {:?}", error);
                    }
                }
            }
            Err(_) => {
                return;
            }
        };
    };

    view! {
        <label>
        "燃料量:"
        <input type="number"
            bind:value=fuel_amount
        />
        </label>
        <EngineSelector engine=engine/>
        <Show when=requires_external_fuel_tank>
            <label>
                "燃料舱数量："
                <input
                    type="number"
                    min="0"
                    bind:value=fuel_tank_count_input
                >
                </input>
            </label>
        </Show>
        <Show when=requires_oxidizer>
            <OxidizerSelector oxidizer=oxidizer oxidizer_amount=oxidizer_amount/>
            <OxidizerTankSelector oxidizer_tank=oxidizer_tank oxidizer_tank_count_input=oxidizer_tank_count_input/>
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
        <ResultDisplay result=result.read_only()/>
    }
}

#[component]
fn EngineSelector(engine: RwSignal<EngineKind>) -> impl IntoView {
    view! {
        <fieldset>
            <legend>"火箭选择"</legend>
            {
                EngineKind::ALL.into_iter().map(|kind| {
                    let name = kind.spec().name;
                    view! {
                        <label>
                            {name}
                            <input
                                type="radio"
                                name="engine"
                                prop:checked=move||{
                                engine.get()==kind
                            }
                            on:change=move|_|{
                                engine.set(kind);
                            }
                            />
                        </label>
                    }
                }).collect_view()
            }
        </fieldset>
    }
}

#[component]
fn OxidizerSelector(
    oxidizer: RwSignal<OxidizerKind>,
    oxidizer_amount: RwSignal<String>,
) -> impl IntoView {
    view! {
        <fieldset>
            <legend>"氧化剂"</legend>
            {
                OxidizerKind::ALL.into_iter().map(|kind| {
                    let name = kind.spec().name;
                    view! {
                        <label>
                            {name}
                            <input
                            type="radio"
                            name="oxidizer"
                            prop:checked=move||{
                                oxidizer.get()==kind
                            }
                            on:change=move|_|{
                                oxidizer.set(kind);
                            }
                            />
                        </label>
                    }
                }).collect_view()
            }
            <label>
                "\n 氧化剂量："
                <input type="number"
                    bind:value=oxidizer_amount
                />
            </label>
        </fieldset>
    }
}

#[component]
fn OxidizerTankSelector(
    oxidizer_tank: RwSignal<OxidizerTankKind>,
    oxidizer_tank_count_input: RwSignal<String>,
) -> impl IntoView {
    view! {
        <fieldset>
            <legend>"氧化剂舱"</legend>
            {
                OxidizerTankKind::ALL.into_iter().map(|kind| {
                    let name = kind.spec().name;
                    view! {
                        <label>
                            {name}
                            <input
                                type="radio"
                                name="oxidizer_tank"
                                prop:checked=move ||{
                                    oxidizer_tank.get()==kind
                                }
                                on:change=move |_| {
                                    oxidizer_tank.set(kind);
                                }
                                />
                        </label>
                    }
                }).collect_view()
            }
            <label>
                "氧化剂舱数量："
                <input
                    type="number"
                    min="0"
                    bind:value=oxidizer_tank_count_input
                />
            </label>
        </fieldset>
    }
}
#[component]
fn ResultDisplay(result: ReadSignal<Option<CalculatorResult>>) -> impl IntoView {
    view! {
        {move || {
            result.get().map(|value| {
                format!(
                    "理论航程: {:.2}，完整航程: {}，速度：{:.2}格/周期，限制资源: {:?}",
                    value.exact_range,
                    value.complete_range,
                    value.speed,
                    value.restrict
                )
            })
        }}
    }
}
