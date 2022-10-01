use anyhow::Result;

use pueue_lib::network::message::{Message, TaskSelection};
use pueue_lib::task::*;

use crate::fixtures::*;
use crate::helper::*;

/// Test if adding a normal task works as intended.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_normal_add() -> Result<()> {
    let daemon = daemon().await?;
    let shared = &daemon.settings.shared;

    // Add a task that instantly finishes
    assert_success(add_task(shared, "sleep 0.01", false).await?);

    // Wait until the task finished and get state
    wait_for_task_condition(shared, 0, |task| task.is_done()).await?;

    // The task finished successfully
    assert_eq!(
        get_task_status(shared, 0).await?,
        TaskStatus::Done(TaskResult::Success)
    );

    Ok(())
}


/// Test if adding a task in stashed state work.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_stashed_add() -> Result<()> {
    let daemon = daemon().await?;
    let shared = &daemon.settings.shared;

    // Tell the daemon to add a task in stashed state.
    let mut inner_message = create_add_message(shared, "sleep 60");
    inner_message.stashed = true;
    let message = Message::Add(inner_message);
    assert_success(send_message(shared, message).await?);

    // Make sure the task is actually stashed.
    wait_for_task_condition(shared, 0, |task| {
        matches!(task.status, TaskStatus::Stashed { .. })
    })
    .await?;

    Ok(())
}


/// Test if the `created_at` field persists after submission.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_time_metadata_persistence() -> Result<()> {
    let daemon = daemon().await?;
    let shared = &daemon.settings.shared;

    // Add a task that instantly finishes
    let message = add_task(shared, "sleep 2.0", false).await?;

    assert!(false);
    unimplemented!{ r#"
    - Update `tests/helper/task.rs#L25-L38` to use the `created_at` field
    - Update `lib/src/network/message.rs#L77-L159` for backward-compatibility on AddMessage fields
    - Inspect message from this scope && verify created_at + enqueue_at
    "# }
    // Wait until the task is done to get its state
    wait_for_task_condition(shared, 0, |task| task.is_done()).await?;
    Ok(())
}
/// Pause the default group and make sure that immediately spawning a task still works.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_add_with_immediate_start() -> Result<()> {
    let daemon = daemon().await?;
    let shared = &daemon.settings.shared;

    // Pause the daemon and prevent tasks to be automatically spawned.
    pause_tasks(shared, TaskSelection::All).await?;

    // Tell the daemon to add a task that must be immediately started.
    assert_success(add_task(shared, "sleep 60", true).await?);

    // Make sure the task is actually being started.
    wait_for_task_condition(shared, 0, |task| task.is_running()).await?;

    Ok(())
}
