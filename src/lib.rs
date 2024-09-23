use wasm_bindgen::prelude::*;
use yew::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Clone, PartialEq)]
struct TodoItem {
    text: String,
    completed: bool,
}

enum Msg {
    Add,
    UpdateInput(String),
    Toggle(usize),
    Delete(usize),
}

struct Model {
    todos: Vec<TodoItem>,
    input: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            todos: Vec::new(),
            input: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Add => {
                if !self.input.is_empty() {
                    self.todos.push(TodoItem {
                        text: self.input.clone(),
                        completed: false,
                    });
                    self.input.clear();
                    true
                } else {
                    false
                }
            }
            Msg::UpdateInput(text) => {
                self.input = text;
                true
            }
            Msg::Toggle(index) => {
                if let Some(todo) = self.todos.get_mut(index) {
                    todo.completed = !todo.completed;
                    true
                } else {
                    false
                }
            }
            Msg::Delete(index) => {
                if index < self.todos.len() {
                    self.todos.remove(index);
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput = ctx.link().callback(|e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Msg::UpdateInput(input.value())
        });

        let onclick_add = ctx.link().callback(|_| Msg::Add);

        html! {
            <div class="container">
                <h1>{ "To-Do List" }</h1>
                <input
                    type="text"
                    placeholder="Add a new task"
                    value={self.input.clone()}
                    oninput={oninput}
                />
                <button onclick={onclick_add}>{ "Add" }</button>
                <ul>
                    { for self.todos.iter().enumerate().map(|(index, todo)| self.view_todo(ctx, index, todo)) }
                </ul>
            </div>
        }
    }
}

impl Model {
    fn view_todo(&self, ctx: &Context<Self>, index: usize, todo: &TodoItem) -> Html {
        let toggle = ctx.link().callback(move |_| Msg::Toggle(index));
        let delete = ctx.link().callback(move |_| Msg::Delete(index));
        html! {
            <li>
                <input type="checkbox" checked={todo.completed} onclick={toggle} />
                <span style={if todo.completed { "text-decoration: line-through;" } else { "" }}>
                    { &todo.text }
                </span>
                <button onclick={delete}>{ "Delete" }</button>
            </li>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<Model>::new().render();
}
