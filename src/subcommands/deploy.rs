// Copyright (c) 2023 Arvid Gerstmann. All rights reserved.
use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};
use log::info;
use tokio::time::sleep;

pub async fn deploy() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("running deploy ...");

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(80));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            .tick_strings(&[
                "[    ]", "[=   ]", "[==  ]", "[=== ]", "[ ===]", "[  ==]", "[   =]", "[    ]",
                "[   =]", "[  ==]", "[ ===]", "[====]", "[=== ]", "[==  ]", "[=   ]", "[====]",
            ]),
    );
    pb.set_message("Deploying...");
    sleep(Duration::from_secs(2)).await;
    pb.finish_with_message("Done");

    Ok(())
}
