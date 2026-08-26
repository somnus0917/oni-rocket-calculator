use crate::calculator::{CalculatorResult, LimitingResource};
use leptos::prelude::*;
pub fn limiting_resource_message(limit: &LimitingResource) -> &'static str {
    match limit {
        LimitingResource::Balance => "刚好平衡",
        LimitingResource::Fuel => "燃料",
        LimitingResource::Oxidizer => "氧化剂",
    }
}
#[component]
pub fn ResultDisplay(result: ReadSignal<Option<CalculatorResult>>) -> impl IntoView {
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
