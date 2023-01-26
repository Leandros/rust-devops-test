// Copyright (c) 2023 Arvid Gerstmann. All rights reserved.
use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};
use tokio::time::sleep;

const SPINNER: &[&str] = &[
    "[    ]", "[=   ]", "[==  ]", "[=== ]", "[ ===]", "[  ==]", "[   =]", "[    ]", "[   =]",
    "[  ==]", "[ ===]", "[====]", "[=== ]", "[==  ]", "[=   ]", "✔",
];

pub async fn deploy(
    env: &str,
    project: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(80));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            .tick_strings(SPINNER),
    );
    pb.set_message(format!("deploying {project}"));
    sleep(Duration::from_secs(2)).await;
    pb.finish_with_message(format!("successfully deployed {project} to {env}!"));

    Ok(())
}
