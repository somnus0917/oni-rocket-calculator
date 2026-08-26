use crate::calculator::{CalculatorError, CalculatorInput, CalculatorResult, calculate};
use crate::components::{
    CommandModuleSelector, EngineSelector, ErrorDisplay, NoseconeSelector, OxidizerSelector,
    OxidizerTankSelector, ResultDisplay,
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
                    <div class="config-section">
                        <div class="section-heading">
                            <h3>"动力系统"</h3>
                            <span>"选择火箭发动机"</span>
                        </div>
                        <EngineSelector engine=engine/>
                    </div>

                    <div class="config-section">
                        <div class="section-heading">
                            <h3>"载人结构"</h3>
                            <span>"配置指挥舱与前锥"</span>
                        </div>
                        <CommandModuleSelector spacefarer=spacefarer/>
                        <Show when=move || {
                            spacefarer.get() == SpacefarerKind::SpacefarerModule
                        }>
                            <NoseconeSelector nosecone=nosecone/>
                        </Show>
                    </div>

                    <div class="config-section">
                        <div class="section-heading">
                            <h3>"推进剂"</h3>
                            <span>"配置燃料与氧化剂"</span>
                        </div>
                        <label class="field">
                            <span>"燃料量"</span>
                            <input type="number" bind:value=fuel_amount/>
                        </label>
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
                    </div>
                    <button class="calculate-button" on:click=on_calculate>
                        "计算火箭"
                    </button>
                </section>
                <section class="panel result-panel">
                    <h2>"计算结果"</h2>
                    <ErrorDisplay
                        calculator_error=calculator_error.read_only()
                        form_error=form_error.read_only()
                    />
                    <ResultDisplay result=result.read_only()/>
                </section>
            </div>
        </main>
    }
}
