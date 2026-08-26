use crate::calculator::{
    CalculatorError, CalculatorInput, CalculatorResult, LimitingResource, calculate,
};
use crate::models::{
    EngineKind, FuelStorage, FuelTankKind, NoseconeKind, OxidizerInput, OxidizerKind,
    OxidizerTankKind, RocketInput, RocketModule, SpacefarerKind,
};
use leptos::{ev::MouseEvent, prelude::*};
#[component]
pub fn App() -> impl IntoView {
    let oxidizer_amount = RwSignal::new("".to_string());
    let fuel_amount = RwSignal::new("".to_string());
    let oxidizer = RwSignal::new(OxidizerKind::LiquidOxygen);
    let oxidizer_tank = RwSignal::new(OxidizerTankKind::Liquid);
    let oxidizer_tank_count_input = RwSignal::new("1".to_string());
    let engine = RwSignal::new(EngineKind::Steam);
    let spacefarer = RwSignal::new(SpacefarerKind::SoloSpacefarerNosecone);
    let nosecone = RwSignal::new(NoseconeKind::BasicNosecone);
    let requires_oxidizer = move || engine.get().spec().requires_oxidizer;
    let result = RwSignal::new(None::<CalculatorResult>);
    let fuel_tank_count_input = RwSignal::new("1".to_string());
    let requires_external_fuel_tank =
        move || matches!(engine.get().spec().fuel_storage, FuelStorage::ExternalTank);
    let calculator_error = RwSignal::new(None::<CalculatorError>);
    let form_error = RwSignal::new(None::<&'static str>);
    let on_calculate = move |_: MouseEvent| {
        result.set(None);
        calculator_error.set(None);
        form_error.set(None);
        match fuel_amount.get().parse::<f32>() {
            Ok(value) => {
                let current_engine = engine.get();
                let current_oxidizer = oxidizer.get();
                let current_spacefarer = spacefarer.get();
                let current_nosecone = nosecone.get();
                let mut modules = Vec::new();
                // 指挥舱
                modules.push(RocketModule::Spacefarer(current_spacefarer));
                if current_spacefarer == SpacefarerKind::SpacefarerModule {
                    modules.push(RocketModule::Nosecone(current_nosecone));
                }
                // 燃料舱
                match current_engine.spec().fuel_storage {
                    FuelStorage::Internal { .. } => {}
                    FuelStorage::ExternalTank => {
                        let fuel_tank_count = match fuel_tank_count_input.get().parse::<usize>() {
                            Ok(count) => count,
                            Err(_) => {
                                form_error.set(Some("燃料舱数量格式错误"));
                                return;
                            }
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
                                    Err(_) => {
                                        form_error.set(Some("氧化剂舱数量格式错误"));
                                        return;
                                    }
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
                            form_error.set(Some("请输入正确的氧化剂量"));
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
                        calculator_error.set(None);
                        result.set(Some(calculated));
                    }

                    Err(error) => {
                        leptos::logging::log!("计算错误: {:?}", error);
                        calculator_error.set(Some(error));
                    }
                }
            }
            Err(_) => {
                form_error.set(Some("请输入正确的燃料量"));
                return;
            }
        };
    };

    view! {
        <main class="app-shell">
            <header class="hero">
                <p class="eyebrow">"OXYGEN NOT INCLUDED"</p>
                <h1>"Rocket Calculator"</h1>
                <p class="subtitle">"缺氧：眼冒金星 火箭配置与航程计算"</p>
            </header>

            <div class="calculator-layout">
                <section class="panel config-panel">
                    <h2>"火箭配置"</h2>
                    <label class="field">
                        <span>"燃料量"</span>
                        <input type="number" bind:value=fuel_amount/>
                    </label>
                    <EngineSelector engine=engine/>
                    <CommandModuleSelector spacefarer=spacefarer/>
                    <Show when=move || spacefarer.get() == SpacefarerKind::SpacefarerModule>
                        <NoseconeSelector nosecone=nosecone/>
                    </Show>
                    <Show when=requires_external_fuel_tank>
                        <label class="field">
                            <span>"燃料舱数量"</span>
                            <input type="number" min="0" bind:value=fuel_tank_count_input/>
                        </label>
                    </Show>
                    <Show when=requires_oxidizer>
                        <OxidizerSelector oxidizer=oxidizer oxidizer_amount=oxidizer_amount/>
                        <OxidizerTankSelector oxidizer_tank=oxidizer_tank oxidizer_tank_count_input=oxidizer_tank_count_input/>
                    </Show>
                    <button class="calculate-button" on:click=on_calculate>
                        "计算火箭"
                    </button>
                </section>
                <section class="panel result-panel">
                    <h2>"计算结果"</h2>
                    {move || {
                        calculator_error.get().map(|error| {
                            view! {
                                <p class="error">{error_message_text(&error)}</p>
                            }
                        })
                    }}
                    {move || {
                        form_error.get().map(|message| {
                            view! {
                                <p class="error">{message}</p>
                            }
                        })
                    }}
                    <ResultDisplay result=result.read_only()/>
                </section>
            </div>
        </main>
    }
}

#[component]
fn EngineSelector(engine: RwSignal<EngineKind>) -> impl IntoView {
    view! {
        <fieldset class="selector-group">
            <legend>"引擎"</legend>
            <div class="option-grid">
            {
                EngineKind::ALL.into_iter().map(|kind| {
                    let spec=kind.spec();
                    view! {
                        <label class="option-card">
                            <input
                                type="radio"
                                name="engine"
                                prop:checked=move || engine.get()==kind
                                on:change=move |_| engine.set(kind)
                            />
                            <div>
                                <strong>
                                    {spec.name}
                                </strong>
                                <span class="option-meta">
                                    {format!("动力 {}",spec.engine_power)}
                                </span>
                            </div>
                        </label>

                    }
                }).collect_view()
            }
            </div>
        </fieldset>
    }
}

#[component]
fn OxidizerSelector(
    oxidizer: RwSignal<OxidizerKind>,
    oxidizer_amount: RwSignal<String>,
) -> impl IntoView {
    view! {
        <fieldset class="selector-group">
            <legend>"氧化剂"</legend>
            <div class="option-grid">
                {
                    OxidizerKind::ALL.into_iter().map(|kind| {
                        let spec = kind.spec();
                        view! {
                            <label class="option-card">
                                <input
                                    type="radio"
                                    name="oxidizer"
                                    prop:checked=move || oxidizer.get() == kind
                                    on:change=move |_| oxidizer.set(kind)
                                />
                                <div>
                                    <strong>{spec.name}</strong>
                                    <span class="option-meta">
                                        {format!("效率 {:.1}", spec.efficiency)}
                                    </span>
                                </div>
                            </label>
                        }
                    }).collect_view()
                }
            </div>
            <label class="field">
                <span>"氧化剂量"</span>
                <input type="number" bind:value=oxidizer_amount/>
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
        <fieldset class="selector-group">
            <legend>"氧化剂舱"</legend>
            <div class="option-grid">
                {
                    OxidizerTankKind::ALL.into_iter().map(|kind| {
                        let spec = kind.spec();
                        view! {
                            <label class="option-card">
                                <input
                                    type="radio"
                                    name="oxidizer_tank"
                                    prop:checked=move || oxidizer_tank.get() == kind
                                    on:change=move |_| oxidizer_tank.set(kind)
                                />
                                <div>
                                    <strong>{spec.name}</strong>
                                    <span class="option-meta">
                                        {format!("容量 {}", spec.capacity)}
                                    </span>
                                </div>
                            </label>
                        }
                    }).collect_view()
                }
            </div>
            <label class="field">
                <span>"氧化剂舱数量"</span>
                <input type="number" min="0" bind:value=oxidizer_tank_count_input/>
            </label>
        </fieldset>
    }
}
#[component]
fn ResultDisplay(result: ReadSignal<Option<CalculatorResult>>) -> impl IntoView {
    view! {
        {move || match result.get() {
            Some(value) => view! {
                <div class="result-grid">
                    <div class="result-card result-primary">
                        <span>"理论航程"</span>
                        <strong>{format!("{:.2}", value.exact_range)}</strong>
                        <small>"格"</small>
                    </div>
                    <div class="result-card">
                        <span>"完整航程"</span>
                        <strong>{value.complete_range}</strong>
                        <small>"格"</small>
                    </div>
                    <div class="result-card">
                        <span>"火箭速度"</span>
                        <strong>{format!("{:.2}", value.speed)}</strong>
                        <small>"格 / 周期"</small>
                    </div>
                    <div class="result-card">
                        <span>"总高度"</span>
                        <strong>{value.total_height}</strong>
                    </div>
                    <div class="result-card">
                        <span>"总负担"</span>
                        <strong>{value.total_burden}</strong>
                    </div>
                    <div class="result-card">
                        <span>"限制资源"</span>
                        <strong>{limiting_resource_message(&value.restrict)}</strong>
                    </div>
                </div>
            }
            .into_any(),
            None => view! {
                <div class="result-empty">
                    <div class="result-empty-icon">"🚀"</div>
                    <strong>"等待计算"</strong>
                    <p>"完成左侧火箭配置后，点击计算火箭查看结果。"</p>
                </div>
            }
            .into_any(),
        }}
    }
}
#[component]
fn CommandModuleSelector(spacefarer: RwSignal<SpacefarerKind>) -> impl IntoView {
    view! {
        <fieldset class="selector-group">
            <legend>"指挥舱选择"</legend>
            <div class="option-grid">
                {
                    SpacefarerKind::ALL.into_iter().map(|kind| {
                        let spec = kind.spec();
                        view! {
                            <label class="option-card">
                                <input
                                    type="radio"
                                    name="spacefarer"
                                    prop:checked=move || spacefarer.get() == kind
                                    on:change=move |_| spacefarer.set(kind)
                                />
                                <div>
                                    <strong>{spec.name}</strong>
                                    <span class="option-meta">
                                        {format!("负担 {} · 高度 {}", spec.burden, spec.height)}
                                    </span>
                                </div>
                            </label>
                        }
                    }).collect_view()
                }
            </div>
        </fieldset>
    }
}
#[component]
fn NoseconeSelector(nosecone: RwSignal<NoseconeKind>) -> impl IntoView {
    view! {
        <fieldset class="selector-group">
            <legend>"前锥选择"</legend>
            <div class="option-grid">
                {
                    NoseconeKind::ALL.into_iter().map(|kind| {
                        let spec = kind.spec();
                        view! {
                            <label class="option-card">
                                <input
                                    type="radio"
                                    name="nosecone"
                                    prop:checked=move || nosecone.get() == kind
                                    on:change=move |_| nosecone.set(kind)
                                />
                                <div>
                                    <strong>{spec.name}</strong>
                                    <span class="option-meta">
                                        {format!("负担 {} · 高度 {}", spec.burden, spec.height)}
                                    </span>
                                </div>
                            </label>
                        }
                    }).collect_view()
                }
            </div>
        </fieldset>
    }
}
pub fn error_message_text(error: &CalculatorError) -> &'static str {
    match error {
        CalculatorError::NegativeFuel => "燃料量不能为负数",
        CalculatorError::MissingOxidizer => "缺少氧化剂",
        CalculatorError::NegativeOxidizer => "氧化剂量不能为负数",
        CalculatorError::FuelExceedsCapacity => "燃料超过容量",
        CalculatorError::OxidizerExceedsCapacity => "氧化剂超过容量",
        CalculatorError::IncompatibleOxidizerTank => "氧化剂舱类型不兼容",
        CalculatorError::RocketExceedsMaxHeight => "火箭超过最大高度",
        CalculatorError::CommandModuleTooLess => "指挥舱数量不能为0",
        CalculatorError::CommandModuleTooMuch => "指挥舱数量不能多于1个",
        CalculatorError::MissingNosecone => "前锥数量不能为0",
        CalculatorError::MultipleNosecones => "前锥数量不能多于1个",
    }
}

pub fn limiting_resource_message(limit: &LimitingResource) -> &'static str {
    match limit {
        LimitingResource::Balance => "刚好平衡",
        LimitingResource::Fuel => "燃料",
        LimitingResource::Oxidizer => "氧化剂",
    }
}
