use leptos::*;

pub mod db;
pub mod display_list;
use display_list::DisplayList;

#[derive(Clone)]
struct MyObject {
    name: String,
    age: u32,
}
impl db::SelectFromDb for MyObject {
    fn mock_select_all() -> Vec<(u32, Self)>
    where
        Self: Sized,
    {
        vec![
            (
                0,
                MyObject {
                    name: "Ivan".to_string(),
                    age: 32,
                },
            ),
            (
                1,
                MyObject {
                    name: "Andrew".to_string(),
                    age: 11,
                },
            ),
            (
                2,
                MyObject {
                    name: "Tanya".to_string(),
                    age: 22,
                },
            ),
        ]
    }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // 1. This could solve the issue, but DisplayList is a fn, not a Type.
    // type MyDisplayList<U,V> = DisplayList<MyObject, U, V>;
    // use DisplayList::<T,_,_> as _;
    view! { cx,
        "I have a table and want to render it:"
        <DisplayList
            bind:my_object
            //
            // error[E0282]: type annotations needed for `&_`
            //   --> src/lib.rs:44:13
            //   |
            // 44 |             bind:my_object
            //   |             ^^^^^^^^^^^^^^
            // ...
            // 48 |             {my_object.name} = {my_object.age}
            //   |              -------------- type must be known at this point
            //   |
            // help: consider giving this closure parameter an explicit type, where the placeholders `_` are specified
            //   |
            // 44 |             bind:my_object: &_
            //   |
            // 2. second solution, the one that i propose
            // bind(my_object: MyObject)
        >
            {&my_object.name}" => "{my_object.age}

        </DisplayList>

        // 3. Type in component probably can solve the issue too
        // <DisplayList<T, _,_>
        //     bind:my_object
        // >
        //     {my_object.name} = {my_object.age}
        // </DisplayList>
    }
}
