#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod bind;
mod context;

use bind::{bind_about_model, bind_detail_model, bind_home_model, bind_settings_model};
use context::UpdateContext;

use i_slint_backend_winit::WinitWindowAccessor;
use slint::{PhysicalPosition, Window};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tunet_helper::Result;
use tunet_model::{Action, Model};
use tunet_settings::FileSettingsReader;

slint::include_modules!();

#[tokio::main]
async fn main() -> Result<()> {
    let app = App::new()?;

    let context = UpdateContext::new(&app);
    let (tx, rx) = mpsc::channel(32);
    let model = context.create_model(tx).await?;
    start_model(&model).await?;

    {
        let home_model = app.global::<HomeModel>();
        bind_home_model(&home_model, &model);

        let detail_model = app.global::<DetailModel>();
        bind_detail_model(&detail_model, &model, &context);

        let settings_model = app.global::<SettingsModel>();
        bind_settings_model(&settings_model, &model, &context);

        let about_model = app.global::<AboutModel>();
        bind_about_model(&about_model, &context);
    }

    app.show()?;

    start_model_loop(model.clone(), rx);

    center_window(app.window());

    app.run()?;

    stop_model(&model, context.del_at_exit()).await?;
    Ok(())
}

async fn start_model(model: &Mutex<Model>) -> Result<()> {
    let model = model.lock().await;
    let settings_reader = FileSettingsReader::new()?;
    if let Ok(cred) = settings_reader.read_with_password() {
        model.queue(Action::Credential(Arc::new(cred)));
    }
    model.queue(Action::Status);
    model.queue(Action::WatchStatus);
    model.queue(Action::Timer);
    Ok(())
}

fn start_model_loop(model: Arc<Mutex<Model>>, mut rx: mpsc::Receiver<Action>) {
    tokio::spawn(async move {
        while let Some(a) = rx.recv().await {
            model.lock().await.handle(a);
        }
    });
}

async fn stop_model(model: &Mutex<Model>, del_at_exit: bool) -> Result<()> {
    let mut settings_reader = FileSettingsReader::new()?;
    if del_at_exit {
        settings_reader.delete()?;
    } else {
        let cred = model.lock().await.cred.clone();
        settings_reader.save(cred).await?;
    }
    Ok(())
}

fn center_window(window: &Window) {
    if let Some(new_pos) = window
        .with_winit_window(|window| {
            window.primary_monitor().map(|monitor| {
                let monitor_pos = monitor.position();
                let monitor_size = monitor.size();
                let window_size = window.outer_size();
                PhysicalPosition {
                    x: monitor_pos.x + ((monitor_size.width - window_size.width) / 2) as i32,
                    y: monitor_pos.y + ((monitor_size.height - window_size.height) / 2) as i32,
                }
            })
        })
        .flatten()
    {
        window.set_position(new_pos);
    }
}
