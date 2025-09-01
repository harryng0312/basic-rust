use druid::widget::{Align, Button, Flex, Label, TextBox};
use druid::{
    AppLauncher, Data, Env, Event, EventCtx, Lens, LifeCycle, LifeCycleCtx, PlatformError,
    UpdateCtx, Widget, WidgetExt, WindowDesc,
};
use std::process::exit;
use utils::log::configuration::init_logger;

#[derive(Clone, Data, Lens)]
struct AppState {
    name: String,
    message: String,
}

/// Controller để bắt các sự kiện như mở cửa sổ, resize, v.v.
struct MyController;

impl<W: Widget<AppState>> druid::widget::Controller<AppState, W> for MyController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        match event {
            Event::WindowConnected => {
                println!("📌 Cửa sổ đã mở");
            }
            Event::WindowDisconnected => {
                println!("❌ Cửa sổ đã đóng");
                exit(0)
            }
            Event::WindowSize(size) => {
                println!("📐 Kích thước mới: {:?}", size);
            }
            _ => {}
        }
        child.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        child: &mut W,
        ctx: &mut LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &AppState,
        env: &Env,
    ) {
        if let LifeCycle::WidgetAdded = event {
            println!("🎯 Widget đã được thêm vào UI");
        }
        child.lifecycle(ctx, event, data, env);
    }

    fn update(
        &mut self,
        child: &mut W,
        ctx: &mut UpdateCtx,
        old_data: &AppState,
        data: &AppState,
        env: &Env,
    ) {
        child.update(ctx, old_data, data, env);
    }
}

fn build_ui() -> impl Widget<AppState> {
    let label = Label::new(|data: &AppState, _env: &Env| data.message.clone()).with_text_size(18.0);
    let textbox = TextBox::new()
        .with_placeholder("Nhập tên")
        .lens(AppState::name);
    let button = Button::new("Gửi").on_click(|_ctx, data: &mut AppState, _env| {
        data.message = format!("Xin chào, {}!", data.name);
    });

    Align::centered(
        Flex::column()
            .with_spacer(10.0)
            .with_child(textbox.fix_width(200.0))
            .with_spacer(10.0)
            .with_child(button)
            .with_spacer(10.0)
            .with_child(label)
            .padding(20.0),
    )
        .controller(MyController) // Gắn controller ở đây
}

pub fn init_app() -> Result<(), PlatformError> {
    init_logger();
    let window = WindowDesc::new(build_ui())
        .window_size((400.0, 200.0))
        .title("Druid + Event Controller");

    let initial_state = AppState {
        name: "".into(),
        message: "Chào mừng!".into(),
    };

    AppLauncher::with_window(window)
        .log_to_console()
        .launch(initial_state)
}
