use leptos::*;

use crate::db::*;

///
/// Try request data from db, and render it as unordered list.
/// Render of each object is provided by user.
///
// #[component]
#[allow(non_snake_case)]
fn __DisplayList<T, FV, V>(cx: Scope, children: FV) -> impl IntoView
where
    FV: Fn(Scope, &T) -> V + 'static,
    V: IntoView,
    T: SelectFromDb + 'static + Clone,
{
    let data: Vec<(u32, T)> =
        <T as SelectFromDb>::mock_select_all(/* some limits, and other params */);
    let (data, _) = create_signal(cx, data);

    view! {
        cx,
        <ul>
        <For each=data key=|i|i.0 view=move |cx, (_,v)| {
            view!{
                cx,
                <li>
                    {children(cx, &v)}
                </li>
            }
        } />
        </ul>
    }
}

//
// Modify component generation,
// to fix `parameter `T` is never used` error
//
use std::marker::PhantomData;

#[derive(::leptos::typed_builder::TypedBuilder)]
pub struct DisplayListProps<T, FV, V>
where
    FV: Fn(Scope, &T) -> V + 'static,
    V: IntoView,
    T: SelectFromDb + 'static + Clone,
{
    #[builder(setter(doc = "**children**: [`FV`]"))]
    #[builder()]
    pub children: FV,
    #[builder(default)]
    pub _pd_t: PhantomData<T>,
    #[builder(default)]
    pub _pd_v: PhantomData<V>,
}
impl<T, FV, V> ::leptos::Props for DisplayListProps<T, FV, V>
where
    FV: Fn(Scope, &T) -> V + 'static,
    V: IntoView,
    T: SelectFromDb + 'static + Clone,
{
    type Builder = DisplayListPropsBuilder<T, FV, V>;
    fn builder() -> Self::Builder {
        DisplayListProps::builder()
    }
}
impl<T, FV, V> ::leptos::IntoView for DisplayListProps<T, FV, V>
where
    FV: Fn(Scope, &T) -> V + 'static,
    V: IntoView,
    T: SelectFromDb + 'static + Clone,
{
    fn into_view(self, cx: ::leptos::Scope) -> ::leptos::View {
        DisplayList(cx, self).into_view(cx)
    }
}

#[allow(non_snake_case)]
pub fn DisplayList<T, FV, V>(
    #[allow(unused_variables)] cx: ::leptos::Scope,
    props: DisplayListProps<T, FV, V>,
) -> impl IntoView
where
    FV: Fn(Scope, &T) -> V + 'static,
    V: IntoView,
    T: SelectFromDb + 'static + Clone,
{
    let DisplayListProps { children, .. } = props;
    ::leptos::leptos_dom::Component::new(stringify!(DisplayList), move |cx| {
        __DisplayList(cx, children)
    })
}
