use crate::state::AppState;
use gpui::prelude::*;
use gpui::*;

pub struct GraphView {
    pub state: Entity<AppState>,
}

impl Render for GraphView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.state.clone();

        div()
            .flex_1()
            .h_full()
            .bg(rgb(0x11111b))
            .child(
                canvas(
                    move |_bounds, _window, cx| {
                        state.update(cx, |state, _| state.step_layout());
                    },
                    move |bounds, _prepaint, window, cx| {
                        let state = state.read(cx);

                        for node in &state.graph.nodes {
                            let center = point(bounds.left() + px(node.x), bounds.top() + px(node.y));
                            window.paint_quad(gpui::fill(
                                Bounds::centered_at(center, size(px(14.), px(14.))),
                                rgb(0x89b4fa),
                            ));
                        }
                    },
                )
                .size_full(),
            )
    }
}
