#![deny(clippy::all)]

use napi::{Env, JsDate, JsNumber, Result};
use napi_derive::napi;
use printers::common::base::job::{
  PrinterJob as NativePrinterJob, PrinterJobState as NativePrinterJobState,
};
use printers::common::base::printer::{
  Printer as NativePrinter, PrinterState as NativePrinterState,
};
use std::time::{SystemTime, UNIX_EPOCH};

#[napi(string_enum)]
pub enum PrinterState {
  READY,
  PAUSED,
  PRINTING,
  UNKNOWN,
}

impl From<NativePrinterState> for PrinterState {
  fn from(native: NativePrinterState) -> Self {
    match native {
      NativePrinterState::READY => PrinterState::READY,
      NativePrinterState::PAUSED => PrinterState::PAUSED,
      NativePrinterState::PRINTING => PrinterState::PRINTING,
      NativePrinterState::UNKNOWN => PrinterState::UNKNOWN,
    }
  }
}

#[napi(object)]
pub struct Printer {
  pub name: String,
  pub system_name: String,
  pub driver_name: String,
  pub uri: String,
  pub port_name: String,
  pub processor: String,
  pub data_type: String,
  pub description: String,
  pub location: String,
  pub is_default: bool,
  pub is_shared: bool,
  pub state: PrinterState,
}

impl From<NativePrinter> for Printer {
  fn from(p: NativePrinter) -> Self {
    Printer {
      name: p.name,
      system_name: p.system_name,
      driver_name: p.driver_name,
      uri: p.uri,
      port_name: p.port_name,
      processor: p.processor,
      data_type: p.data_type,
      description: p.description,
      location: p.location,
      is_default: p.is_default,
      is_shared: p.is_shared,
      state: PrinterState::from(p.state),
    }
  }
}

#[napi(string_enum)]
pub enum PrinterJobState {
  PENDING,
  PAUSED,
  PROCESSING,
  CANCELLED,
  COMPLETED,
  UNKNOWN,
}

impl From<NativePrinterJobState> for PrinterJobState {
  fn from(native: NativePrinterJobState) -> Self {
    match native {
      NativePrinterJobState::PENDING => PrinterJobState::PENDING,
      NativePrinterJobState::PAUSED => PrinterJobState::PAUSED,
      NativePrinterJobState::PROCESSING => PrinterJobState::PROCESSING,
      NativePrinterJobState::CANCELLED => PrinterJobState::CANCELLED,
      NativePrinterJobState::COMPLETED => PrinterJobState::COMPLETED,
      NativePrinterJobState::UNKNOWN => PrinterJobState::UNKNOWN,
    }
  }
}

#[napi(object)]
pub struct PrinterJob {
  pub id: JsNumber,
  pub name: String,
  pub state: PrinterJobState,
  pub media_type: String,
  pub created_at: JsDate,
  pub processed_at: Option<JsDate>,
  pub completed_at: Option<JsDate>,
  pub printer_name: String,
}

pub trait FromWithEnv<T> {
  fn from_with_env(env: &Env, value: T) -> Self;
}

impl FromWithEnv<NativePrinterJob> for PrinterJob {
  fn from_with_env(env: &Env, j: NativePrinterJob) -> Self {
    let id = env
      .create_int64(j.id as i64)
      .unwrap_or_else(|_| env.create_int32(-1).unwrap());

    let created_at = safe_date(env, j.created_at).unwrap_or_else(|| env.create_date(0.0).unwrap());

    let processed_at = j.processed_at.and_then(|t| safe_date(env, t));
    let completed_at = j.completed_at.and_then(|t| safe_date(env, t));

    PrinterJob {
      id,
      name: j.name,
      state: PrinterJobState::from(j.state),
      media_type: j.media_type,
      created_at,
      processed_at,
      completed_at,
      printer_name: j.printer_name,
    }
  }
}

#[napi]
pub fn get_printers() -> Vec<Printer> {
  printers::get_printers()
    .into_iter()
    .map(|p| Printer::from(p))
    .collect()
}

#[napi]
pub fn get_printer_by_name(printer_name: String) -> Option<Printer> {
  printers::get_printer_by_name(printer_name.as_str()).map(|p| Printer::from(p))
}

#[napi]
pub fn get_default_printer() -> Option<Printer> {
  printers::get_default_printer().map(|p| Printer::from(p))
}

#[napi]
pub fn print(printer_name: String, buffer: &[u8], job_name: Option<&str>) -> Result<()> {
  let printer = printers::get_printer_by_name(printer_name.as_str())
    .ok_or_else(|| napi::Error::from_reason("Printer not found".to_string()))?;

  printer
    .print(buffer, job_name)
    .map_err(|e| napi::Error::from_reason(format!("Print failed: {}", e)))?;

  Ok(())
}

#[napi]
pub fn print_file(printer_name: String, file_path: String, job_name: Option<&str>) -> Result<()> {
  let printer = printers::get_printer_by_name(printer_name.as_str())
    .ok_or_else(|| napi::Error::from_reason("Printer not found".to_string()))?;

  printer
    .print_file(file_path.as_str(), job_name)
    .map_err(|e| napi::Error::from_reason(format!("Print failed: {}", e)))?;

  Ok(())
}

#[napi]
pub fn get_active_jobs(env: Env, printer_name: String) -> Vec<PrinterJob> {
  let Some(printer) = printers::get_printer_by_name(printer_name.as_str()) else {
    return Vec::new();
  };

  printer
    .get_active_jobs()
    .into_iter()
    .map(|j| PrinterJob::from_with_env(&env, j))
    .collect()
}

#[napi]
pub fn get_job_history(env: Env, printer_name: String) -> Vec<PrinterJob> {
  let Some(printer) = printers::get_printer_by_name(printer_name.as_str()) else {
    return Vec::new();
  };

  printer
    .get_job_history()
    .into_iter()
    .map(|j| PrinterJob::from_with_env(&env, j))
    .collect()
}

fn safe_date(env: &Env, time: SystemTime) -> Option<JsDate> {
  let millis = time
    .duration_since(UNIX_EPOCH)
    .ok()? // handles SystemTime before UNIX_EPOCH
    .as_millis() as f64;

  env.create_date(millis).ok()
}
