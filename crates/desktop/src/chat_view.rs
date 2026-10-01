use crate::state::AppState;
use gpui::prelude::*;
use gpui::*;

pub struct ChatView {
    pub state: Entity<AppState>,
}

impl Render for ChatView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.state.read(cx);

        let messages: Vec<AnyElement> = state
            .messages
            .iter()
            .map(|m| {
                let color = if m.is_error { rgb(0xf38ba8) } else { rgb(0xcdd6f4) };
                div()
                    .p_2()
                    .mb_1()
                    .rounded_md()
                    .bg(rgb(0x313244))
                    .text_color(color)
                    .text_sm()
                    .child(m.text.clone())
                    .into_any_element()
            })
            .collect();

        div()
            .flex()
            .flex_col()
            .w(px(380.))
            .h_full()
            .p_4()
            .bg(rgb(0x1e1e2e))
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .mb_3()
                    .child(div().text_lg().text_color(rgb(0xcdd6f4)).child("Obsidian Agent"))
                    .child(div().text_xs().text_color(rgb(0x89b4fa)).child(state.selected_model.clone())),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .overflow_y_scroll()
                    .gap_1()
                    .children(messages),
            )
            .child(
                div()
                    .id("send-btn")
                    .px_3()
                    .py_2()
                    .bg(rgb(0x89b4fa))
                    .rounded_md()
                    .text_color(rgb(0x1e1e2e))
                    .text_sm()
                    .cursor_pointer()
                    .child(if state.busy { "..." } else { "Generate" })
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.submit(cx);
                    })),
            )
    }
}

impl ChatView {
    pub fn submit(&mut self, cx: &mut Context<Self>) {
        self.state.update(cx, |state, _| {
            state.push_message("submit wiring still a stub — see earlier note", false);
        });
        cx.notify();
    }
}
