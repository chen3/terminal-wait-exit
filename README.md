## terminal-wait-exit
wait user exit terminal, like Ctrl+C.

# Usage
```rust
#[tokio::main]
async fn main() {
    // Spawn the server into a runtime
    tokio::spawn(async move {
        // work
        server.serve(service).await;
    });

    if let Err(e) = wait_exit::wait_exit().await {
        error!("Listening exit failed.{:?}", e);
    }

    stop_graceful();
}
```
