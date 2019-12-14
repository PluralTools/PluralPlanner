use orbtk::prelude::*;

fn main() {
    Application::new()
        .window(move |ctx| {
            Window::create()
                .title("Do it")
                .position((100.0, 100.0))
                .size(372.0, 768.0)
                .build(ctx)
        })
        .run();
}
