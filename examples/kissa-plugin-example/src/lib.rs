use kissabot::{event::*, topic::prelude::*};

struct MyPlugin;
impl Plugin for MyPlugin {
    type Global = Kissa;
    type Pars = Event;
    fn load(ctx: Context<Self>) -> Result<()> {
        info!("来自插件的日志");
        ctx.observe(repetition);
        Ok(())
    }
}

fn repetition(ctx: Context<MyPlugin>, event: Event) {
    let se;
    if let Some(se_) = event.downcast_ref::<SEvent>() {
        se = se_;
    } else {
        return;
    }
    if let SEventContent {
        ty: SEventType::MessageCreated,
        message: Some(message_),
        ..
    } = &se.content
    {
        let message = message_.content.clone();
        if let Err(err) = se.reply(&ctx, message) {
            println!("Error: {}", err);
        }
    }
}

export_plugin!(MyPlugin, MyPlugin);
