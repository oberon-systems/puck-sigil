mod libs;

use libs::args;
use libs::config;
use libs::git;
use libs::version;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

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

    log::info!("Reading version from file: {}", _config.version_file);
    log::debug!("Looking for parameter: {}", _config.version_param);

    let _version = match version::read_version(&_config.version_file, &_config.version_param) {
        Ok(version) => {
            log::info!("Version found: {}", version);
            version
        }
        Err(e) => {
            log::error!("Error reading version: {}", e);
            std::process::exit(1);
        }
    };

    match git::create_tag(&_version) {
        Ok(()) => {
            log::info!("Version tagged: v{}", _version);
        }
        Err(e) => {
            log::error!("Error occurred while trying to tag version: {}", e);
            std::process::exit(1);
        }
    }
}
