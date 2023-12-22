use tokio::time::{interval, Duration};

use robots_drv::{driver, Cmd, Result, RX, TX};

#[tokio::main]
async fn main() -> Result<()> {
    let uart_port = serialport::new("/dev/ttyUSB0", 115_200);

    driver(uart_port)?;
    let tx = TX.clone();
    let mut rx = RX.clone();

    tokio::spawn(async move {
        while let Some(cmd) = rx.recv().await {
            println!("received {cmd:?}");
        }
    });

    let mut hz = interval(Duration::from_secs(1));

    let relay = false;
    let cmd = Cmd::Relay(!relay);

    hz.tick().await;
    hz.tick().await;
    println!("sending {cmd:?}...");
    tx.send(&cmd).await?;
    hz.tick().await;

    Ok(())
}
