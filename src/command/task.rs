use crate::connector::ConnectorBuilder;

#[derive(Clap, Debug)]
pub struct TaskFlags {
    /// Select which task to run
    #[clap(arg_enum)]
    task_type: TaskType,
}

#[derive(Clap, Debug)]
enum TaskType {
    /// Refresh all gauges based on checkins
    #[clap(name = "refresh-all-gauges")]
    RefreshAllGauges,
}

pub async fn run(flags: TaskFlags, builder: ConnectorBuilder) {
    let connector = builder.create();

    let task_result = match flags.task_type {
        TaskType::RefreshAllGauges => {
            crate::model::place::refresh_all_gauges(&connector).map(|_| ())
        }
    };

    match task_result {
        Ok(_) => println!("Task {:?} completed", flags.task_type),
        Err(error) => error.exit(),
    }
}
