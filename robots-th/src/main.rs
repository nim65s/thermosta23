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

    let mut relay = true;
    let mut cmd = Cmd::Ping;
    println!("sending {cmd:?}...");
    tx.send(&cmd).await?;

    for hue in 0..=255 {
        hz.tick().await;
        println!("sending {cmd:?}...");
        tx.send(&cmd).await?;
        tx.send(&Cmd::Relay(relay)).await?;
        relay = !relay;
        cmd = Cmd::Hue(hue);
    }

    Ok(())
}
