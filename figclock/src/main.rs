use std::{
    io::{stdout, Write},
    time::Duration,
};

use anyhow::{anyhow, Context, Result};
use chrono::Local;
use config::get_config;
use crossterm::{
    cursor::MoveTo,
    style::Print,
    terminal::{Clear, ClearType},
};
use figlet_rs::FIGfont;
use tokio::time::delay_for;

mod config;

#[tokio::main]
async fn main() -> Result<()> {
    let config = get_config()?;

    let font = match &config.font {
        Some(p) => FIGfont::from_file(&p),
        None => FIGfont::standand(),
    }
    .map_err(|e| anyhow!(e))?;

    loop {
        tick(&font)?;
        delay_for(Duration::from_millis(config.update_interval)).await;
    }
}

fn tick(font: &FIGfont) -> Result<()> {
    let fig = font
        .convert(&Local::now().format(&get_config()?.format).to_string())
        .context("error formatting fig")?
        .to_string();

    crossterm::execute!(
        stdout(),
        // Clear terminal
        Clear(ClearType::All),
        // Reset cursor
        MoveTo(0, 0),
        // Print fig
        Print(fig),
    )?;

    Ok(())
}
