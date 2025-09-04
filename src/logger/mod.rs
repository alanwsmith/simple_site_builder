use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::fmt::Write;
use std::path::Path;
use std::path::PathBuf;
use tracing::Event;
use tracing::Subscriber;
use tracing::metadata::LevelFilter;
use tracing::span;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::FmtContext;
use tracing_subscriber::fmt::FormatEvent;
use tracing_subscriber::fmt::FormatFields;
use tracing_subscriber::fmt::FormattedFields;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::fmt::time::SystemTime;
use tracing_subscriber::prelude::*;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::registry::Scope;

pub struct Logger {
  pub guards: Vec<WorkerGuard>,
  stdout: Option<LevelFilter>,
  stderr: Option<LevelFilter>,
  json_dir: Option<PathBuf>,
  json_level: Option<LevelFilter>,
  txt_dir: Option<PathBuf>,
  txt_level: Option<LevelFilter>,
}

impl Logger {
  pub fn new() -> Self {
    Self {
      guards: vec![],
      stdout: None,
      stderr: None,
      json_dir: None,
      json_level: None,
      txt_dir: None,
      txt_level: None,
    }
  }

  pub fn setup() -> Self {
    Self {
      guards: vec![],
      stdout: None,
      stderr: None,
      json_dir: None,
      json_level: None,
      txt_dir: None,
      txt_level: None,
    }
  }

  pub fn with_stdout(
    self,
    level: LevelFilter,
  ) -> Self {
    Self {
      stdout: Some(level),
      ..self
    }
  }

  pub fn with_stderr(
    self,
    level: LevelFilter,
  ) -> Self {
    Self {
      stderr: Some(level),
      ..self
    }
  }

  pub fn to_json_dir(
    self,
    dir: &Path,
    level: LevelFilter,
  ) -> Self {
    Self {
      json_dir: Some(dir.to_path_buf()),
      json_level: Some(level),
      ..self
    }
  }

  pub fn to_txt_dir(
    self,
    dir: &Path,
    level: LevelFilter,
  ) -> Self {
    Self {
      txt_dir: Some(dir.to_path_buf()),
      txt_level: Some(level),
      ..self
    }
  }

  pub fn init(mut self) -> Vec<WorkerGuard> {
    let json_dir_layer = match (&self.json_dir, &self.json_level) {
      (Some(dir), Some(level)) => {
        let file_appender = RollingFileAppender::builder()
          .rotation(Rotation::DAILY)
          .filename_suffix("json.log")
          .max_log_files(2)
          .build(dir)
          .expect("could not make file appender");
        let (file_writer, log_guard) =
          tracing_appender::non_blocking(file_appender);
        self.guards.push(log_guard);
        let file_layer_format =
          tracing_subscriber::fmt::format().json();
        let layer = fmt::Layer::default()
          .event_format(file_layer_format)
          .with_writer(file_writer)
          .json() // migth be able to remove .json since other format is ready in palce
          .with_filter(*level);
        Some(layer)
      }
      _ => None,
    };

    let stderr_layer = match self.stderr {
      Some(level) => {
        let format = MiniFormat;
        let layer = fmt::Layer::default()
          .event_format(format)
          .with_writer(std::io::stderr)
          .with_filter(level);
        Some(layer)
      }
      None => None,
    };

    let stdout_layer = match self.stdout {
      Some(level) => {
        let layer = fmt::Layer::default()
          .event_format(MiniFormat)
          .with_writer(std::io::stdout)
          .with_filter(level);
        Some(layer)
      }
      None => None,
    };

    let txt_dir_layer = match (&self.txt_dir, &self.txt_level) {
      (Some(dir), Some(level)) => {
        let file_appender = RollingFileAppender::builder()
          .rotation(Rotation::DAILY)
          .filename_suffix("log")
          .max_log_files(2)
          .build(dir)
          .expect("could not make file appender");
        let (file_writer, log_guard) =
          tracing_appender::non_blocking(file_appender);
        self.guards.push(log_guard);
        let file_layer_format = MiniFormat;

        let layer = fmt::Layer::default()
          .event_format(file_layer_format)
          .with_writer(file_writer)
          .with_filter(*level);
        Some(layer)
      }
      _ => None,
    };

    let subscriber = tracing_subscriber::Registry::default()
      .with(json_dir_layer)
      .with(stderr_layer)
      .with(stdout_layer)
      .with(txt_dir_layer);

    tracing::subscriber::set_global_default(subscriber)
      .expect("unable to set global subscriber");

    self.guards
  }
}

impl Default for Logger {
  fn default() -> Self {
    Self::new()
  }
}

// NOTE: This is largely copy paste from tracing
// TODO: is to clean it up some and remove
// unnecessary parts

pub struct MiniFormat;
impl<S, N> FormatEvent<S, N> for MiniFormat
where
  S: Subscriber + for<'a> LookupSpan<'a>,
  N: for<'a> FormatFields<'a> + 'static,
{
  fn format_event(
    &self,
    ctx: &FmtContext<'_, S, N>,
    mut writer: Writer<'_>,
    event: &Event<'_>,
  ) -> Result {
    let meta = event.metadata();

    let _ = SystemTime.format_time(&mut writer);
    write!(
      writer,
      "|{}",
      meta.level().as_str().chars().take(1).collect::<Vec<char>>()
        [0]
    )?;
    if let Some(filename) = meta.file() {
      write!(writer, "|{}", filename)?;
    }

    if let Some(line_number) = meta.line() {
      write!(writer, "|{}", line_number,)?;
    }

    let fmt_ctx = { FmtCtx::new(ctx, event.parent()) };
    write!(writer, "{}", fmt_ctx)?;
    writeln!(writer)?;

    ctx.format_fields(writer.by_ref(), event)?;
    for span in
      ctx.event_scope().into_iter().flat_map(Scope::from_root)
    {
      let exts = span.extensions();
      if let Some(fields) = exts.get::<FormattedFields<N>>() {
        if !fields.is_empty() {
          write!(writer, " {}", &fields.fields)?;
        }
      }
    }
    writeln!(writer)?;

    Ok(())
  }
}

struct FmtCtx<'a, S, N> {
  ctx: &'a FmtContext<'a, S, N>,
  span: Option<&'a span::Id>,
}

impl<'a, S, N: 'a> FmtCtx<'a, S, N>
where
  S: Subscriber + for<'lookup> LookupSpan<'lookup>,
  N: for<'writer> FormatFields<'writer> + 'static,
{
  pub(crate) fn new(
    ctx: &'a FmtContext<'_, S, N>,
    span: Option<&'a span::Id>,
  ) -> Self {
    Self { ctx, span }
  }

  fn bold(&self) -> Style {
    Style::new()
  }
}

impl<'a, S, N: 'a> Display for FmtCtx<'a, S, N>
where
  S: Subscriber + for<'lookup> LookupSpan<'lookup>,
  N: for<'writer> FormatFields<'writer> + 'static,
{
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> Result {
    let bold = self.bold();

    let mut seen = false;

    let span = self
      .span
      .and_then(|id| self.ctx.span(id))
      .or_else(|| self.ctx.lookup_current());

    let scope =
      span.into_iter().flat_map(|span| span.scope().from_root());

    for span in scope {
      seen = true;

      write!(f, "|{}", bold.paint(span.metadata().name()))?;
    }

    if seen {
      f.write_char(' ')?;
    }

    Ok(())
  }
}

struct Style;
impl Style {
  fn new() -> Self {
    Style
  }

  fn _bold(self) -> Self {
    self
  }

  fn paint(
    &self,
    d: impl Display,
  ) -> impl Display {
    d
  }

  fn _prefix(&self) -> impl Display {
    ""
  }

  fn _suffix(&self) -> impl Display {
    ""
  }
}
