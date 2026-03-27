use leptos::*;

#[component]
pub fn DataTable<T>(
    data: Vec<T>,
    columns: Vec<TableColumn<T>>,
) -> impl IntoView 
where 
    T: Clone + 'static,
{
    view! {
        <div class="data-table-wrapper">
            <table class="data-table">
                <thead>
                    <tr>
                        {columns.iter().map(|col| view! { 
                            <th>{col.title.clone()}</th> 
                        }).collect_view()}
                    </tr>
                </thead>
                <tbody>
                    {data.into_iter().map(|item| {
                        let item_clone = item.clone();
                        view! {
                            <tr>
                                {columns.iter().map(|col| {
                                    let render = &col.render;
                                    view! { 
                                        <td>{render(item_clone.clone())}</td>
                                    }
                                }).collect_view()}
                            </tr>
                        }
                    }).collect_view()}
                </tbody>
            </table>
        </div>
    }
}

pub struct TableColumn<T> {
    pub title: String,
    pub render: Box<dyn Fn(T) -> View>,
}

impl<T> TableColumn<T> {
    pub fn new<F, IV>(title: String, render: F) -> Self 
    where 
        F: Fn(T) -> IV + 'static,
        IV: IntoView,
    {
        Self {
            title,
            render: Box::new(move |item| render(item).into_view()),
        }
    }
}
