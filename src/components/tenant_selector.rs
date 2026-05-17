use leptos::*;

#[derive(Debug, Clone, Copy)]
pub struct TenantContext {
    pub tenant_id: ReadSignal<String>,
    pub set_tenant_id: WriteSignal<String>,
}

pub fn get_initial_tenant() -> String {
    let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    storage.get_item("hsa_tenant_id").unwrap().unwrap_or_else(|| "default".to_string())
}

#[component]
pub fn TenantSelector() -> impl IntoView {
    let context = use_context::<TenantContext>().expect("TenantContext should be provided");
    let (tenant_id, set_tenant_id) = (context.tenant_id, context.set_tenant_id);

    let on_change = move |ev| {
        let val = event_target_value(&ev);
        set_tenant_id.set(val.clone());
        let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        storage.set_item("hsa_tenant_id", &val).unwrap();
    };

    view! {
        <div class="tenant-selector">
            <div class="selector-label">
                <i class="el-icon-office-building"></i> "当前租户"
            </div>
            <select 
                class="tenant-select" 
                on:change=on_change 
                prop:value=tenant_id
            >
                <option value="default">"默认租户 (公共)"</option>
                <option value="CITY_SH_001">"上海市中心医院 (CITY_SH_001)"</option>
                <option value="CITY_BJ_002">"北京市协和分院 (CITY_BJ_002)"</option>
                <option value="DIST_GZ_003">"广州天河医保局 (DIST_GZ_003)"</option>
            </select>
        </div>
    }
}
