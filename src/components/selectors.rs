use crate::models::{EngineKind, NoseconeKind, OxidizerKind, OxidizerTankKind, SpacefarerKind};
use leptos::prelude::*;
#[component]
pub fn EngineSelector(engine: RwSignal<EngineKind>) -> impl IntoView {
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
pub fn OxidizerSelector(
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
pub fn OxidizerTankSelector(
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
pub fn CommandModuleSelector(spacefarer: RwSignal<SpacefarerKind>) -> impl IntoView {
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
pub fn NoseconeSelector(nosecone: RwSignal<NoseconeKind>) -> impl IntoView {
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
