pub mod serial;
pub mod commands;

use commands::DigitalServoCommandSet;
use serial::canadapter::CANAdapter;

use std::{thread, time};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut canadapter: CANAdapter = CANAdapter::open()?;

    let channel: u8 = 2;

    let commands: Vec<String> = DigitalServoCommandSet::parameters_for_mn501(channel);
    canadapter.send_commands(&commands)?;

    /* Read */
    let mut rdbuf: [u8; 8192] = [0; 8192];
    let recv_size: usize = canadapter.read(&mut rdbuf);
    let recv_data: &[u8] = &rdbuf[0..recv_size];
    let rx_delimiter: &[u8; 2] = b"\r\n";
    let rx_delimiter_len: usize = rx_delimiter.len();

    if recv_size > 0 {
        let rx_delimiter_position: Vec<usize> = recv_data.windows(rx_delimiter_len)
          .enumerate()
          .filter(|(_, w)| *w == rx_delimiter)
          .map(|(i, _)| i)
          .collect::<Vec<usize>>();

        let mut splitter: Vec<usize> = rx_delimiter_position.iter().map(|x| x + rx_delimiter_len).collect();
        splitter.insert(0, 0);
        let words: Vec<&str> = splitter.windows(2)
          .into_iter()
          .map(|x| std::str::from_utf8(&recv_data[x[0]..(x[1] - rx_delimiter_len)]).unwrap_or_default())
          .collect();

        for w in words {
            println!("{w:?}");
        }
    }

    thread::sleep(time::Duration::from_millis(500));

    canadapter.close()?;

    Ok(())
}
