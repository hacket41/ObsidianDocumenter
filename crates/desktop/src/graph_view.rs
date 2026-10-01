use crate::state::AppState;
use gpui::*;

pub struct GraphView {
    pub state: Model<AppState>,
}

impl Render for GraphView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let state = self.state.clone();

        div()
            .flex_1()
            .h_full()
            .bg(rgb(0x11111b))
            .child(
                canvas(
                    move |_bounds, cx| {
                        // prepaint: advance the force-directed layout one tick
                        state.update(cx, |state, _| state.step_layout());
                    },
                    move |bounds, _prepaint, cx| {
                        let state = state.read(cx);

                        for edge in &state.graph.edges {
                            let a = &state.graph.nodes[edge.source];
                            let b = &state.graph.nodes[edge.target];
                            cx.paint_quad(gpui::fill(
                                Bounds::from_corners(
                                    point(bounds.left() + px(a.x), bounds.top() + px(a.y)),
                                    point(bounds.left() + px(b.x), bounds.top() + px(b.y)),
                                ),
                                rgb(0x45475a),
                            ));
                        }

                        for node in &state.graph.nodes {
                            let center = point(bounds.left() + px(node.x), bounds.top() + px(node.y));
                            cx.paint_quad(gpui::fill(
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
