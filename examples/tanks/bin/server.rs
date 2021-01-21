use bevy::app::ScheduleRunnerSettings;
use bevy::prelude::*;
use std::time::Duration;

fn main() {
    App::build()
        .add_plugins(MinimalPlugins)
        .add_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .run()
}
