mod chat_view;
mod graph_view;
mod state;

use chat_view::ChatView;
use gpui::prelude::*;
use gpui::*;
use gpui_platform::application;
use graph_view::GraphView;
use state::AppState;

struct RootView {
    chat: Entity<ChatView>,
    graph: Entity<GraphView>,
}

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .child(self.chat.clone())
            .child(self.graph.clone())
    }
}

fn main() {
    application().run(|cx: &mut App| {
        let state = cx.new(|_| AppState::new());

        cx.open_window(WindowOptions::default(), |_window, cx| {
            let chat = cx.new(|_| ChatView { state: state.clone() });
            let graph = cx.new(|_| GraphView { state: state.clone() });
            cx.new(|_| RootView { chat, graph })
        })
        .unwrap();

        cx.activate(true);
    });
}
