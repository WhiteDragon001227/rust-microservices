use failure::Fallible;

/// Initializes the logging system.
pub fn init() -> Fallible<()> {
  fern::Dispatch::new()
    .format(|out, message, record| {
      out.finish(format_args!(
        "{}[{}][{}] {}",
        chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
        record.target(),
        record.level(),
        message
      ))
    }).level(log::LevelFilter::Debug)
    .level_for("tokio_reactor", log::LevelFilter::Warn)
    .level_for("actix_web::server::server", log::LevelFilter::Warn)
    .chain(std::io::stdout())
    .chain(fern::log_file("output.log")?)
    .apply()?;

  Ok(())
}
