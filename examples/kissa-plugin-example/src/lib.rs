use kissabot::{
    event::*,
    topic::kokoro::result,
    topic::{prelude::*, subscribe},
};

struct MyPlugin;
impl Plugin for MyPlugin {
    type Global = Kissa;
    type Pars = Event;
    fn load(ctx: Context<Self>) -> Result<()> {
        info!("来自插件的日志");
        subscribe!(ctx, SEvent, subscriber);
        ctx.observe(|| {
            info!("观察者被触发");
        });
        Ok(())
    }
}

fn subscriber(ctx: Context<MyPlugin>, event: &SEvent) {
    info!("事件被触发");
    if let SEventContent {
        ty: SEventType::MessageCreated,
        message: Some(message_),
        ..
    } = &event.content
    {
        let message = message_.content.clone();
        info!("插件收到消息: {}", &message);
        if let Err(err) = event.reply(&ctx, message) {
            error!("发送消息失败: {}", err);
        }
    }
}

export_plugin!(MyPlugin, result::Ok(MyPlugin));
