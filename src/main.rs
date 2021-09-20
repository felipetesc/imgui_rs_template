use imgui::{ Window , Condition};

mod support;

fn main() {
    let system = support::lib::sys_init(file!());
    //let event_loop = winit::event_loop::EventLoop::new();

    let mut value = 0;
    let choices = ["test test this is 1", "test test this is 2"];
    system.main_loop(move |_, ui| {
        Window::new("Hello world")
            .size([300.0, 110.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.text_wrapped("Hello world!");
                ui.text_wrapped("こんにちは世界！");
                if ui.button(choices[value]) {
                    value += 1;
                    value %= 2;
                }

                ui.button("This...is...imgui-rs!");
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));
            });
    });
}
