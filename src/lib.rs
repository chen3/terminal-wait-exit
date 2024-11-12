use std::io;

#[cfg(windows)]
pub async fn wait_exit() -> Result<(), io::Error> {
    use tokio::signal::windows::{ctrl_break, ctrl_c, ctrl_close, ctrl_logoff, ctrl_shutdown};
    use tokio::select;

    let mut ctrl_c = match ctrl_c() {
        Ok(c) => c,
        Err(e) => return Err(e),
    };
    let mut ctrl_break = match ctrl_break() {
        Ok(c) => c,
        Err(e) => return Err(e),
    };
    let mut ctrl_close = match ctrl_close() {
        Ok(c) => c,
        Err(e) => return Err(e),
    };
    let mut ctrl_logoff = match ctrl_logoff() {
        Ok(c) => c,
        Err(e) => return Err(e),
    };
    let mut ctrl_shutdown = match ctrl_shutdown() {
        Ok(c) => c,
        Err(e) => return Err(e),
    };
    select! {
        _ = ctrl_c.recv() => (),
        _ = ctrl_break.recv() => (),
        _ = ctrl_close.recv() => (),
        _ = ctrl_logoff.recv() => (),
        _ = ctrl_shutdown.recv() => (),
    };
    return Ok(());
}

#[cfg(unix)]
pub async fn wait_exit() -> Result<(), io::Error> {
    use tokio::signal::unix::{signal, SignalKind};
    use tokio::select;

    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigquit = signal(SignalKind::quit())?;
    let mut sigterm = signal(SignalKind::terminate())?;
    select! {
        _ = sigint.recv() => (),
        _ = sigquit.recv() => (),
        _ = sigterm.recv() => (),
    };
    return Ok(());
}
