use std::env;

pub struct Args {
    pub config_path: String,
}

pub fn parse_args() -> Args {
    let mut config_path = None;
    let args: Vec<String> = env::args().collect();

    for i in 0..args.len() {
        if args[i] == "--config" && i + 1 < args.len() {
            config_path = Some(args[i + 1].clone());
            break;
        }
    }

    let config_path = config_path
        .unwrap_or_else(|| env::var("PSIGIL_CONFIG").unwrap_or_else(|_| "config.json".to_string()));

    Args { config_path }
}

#[cfg(test)]
mod tests;
