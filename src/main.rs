mod libs;

use libs::args;
use libs::config;
use libs::version;

fn main() {
    env_logger::init();

    let _args = args::parse_args();
    let _path = _args.config_path;

    log::debug!("Loading configuration file: {_path}");
    let _config = match config::load_config(_path) {
        Ok(config) => {
            log::debug!("Config loaded successfully: {:?}", config);
            config
        }
        Err(e) => {
            log::error!("Error loading config: {}", e);
            std::process::exit(1);
        }
    };

    println!("\nReading version from file: {}", _config.version_file);
    log::debug!("Looking for parameter: {}", _config.version_param);

    match version::read_version(&_config.version_file, &_config.version_param) {
        Ok(version) => {
            println!("Version found: {}", version);
        }
        Err(e) => {
            log::error!("\nError reading version: {}", e);
            std::process::exit(1);
        }
    }
}
