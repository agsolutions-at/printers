#![deny(clippy::all)]

use chrono::{DateTime, Utc};
use napi::bindgen_prelude::BigInt;
use napi::Result;
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
  OFFLINE,
}

impl From<NativePrinterState> for PrinterState {
  fn from(native: NativePrinterState) -> Self {
    match native {
      NativePrinterState::READY => PrinterState::READY,
      NativePrinterState::PAUSED => PrinterState::PAUSED,
      NativePrinterState::PRINTING => PrinterState::PRINTING,
      NativePrinterState::UNKNOWN => PrinterState::UNKNOWN,
      NativePrinterState::OFFLINE => PrinterState::OFFLINE,
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
  pub state_reasons: Vec<String>,
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
      state_reasons: p.state_reasons,
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
  pub id: BigInt,
  pub name: String,
  pub state: PrinterJobState,
  pub media_type: String,
  pub created_at: DateTime<Utc>,
  pub processed_at: Option<DateTime<Utc>>,
  pub completed_at: Option<DateTime<Utc>>,
  pub printer_name: String,
}

impl From<NativePrinterJob> for PrinterJob {
  fn from(j: NativePrinterJob) -> Self {
    PrinterJob {
      id: BigInt::from(j.id),
      name: j.name,
      state: PrinterJobState::from(j.state),
      media_type: j.media_type,
      created_at: safe_date(&j.created_at)
        .unwrap_or_else(|| DateTime::from_timestamp_millis(0).unwrap()),
      processed_at: j.processed_at.and_then(|t| safe_date(&t)),
      completed_at: j.completed_at.and_then(|t| safe_date(&t)),
      printer_name: j.printer_name,
    }
  }
}

#[napi(object)]
pub struct PrintOption {
  pub key: String,
  pub value: String,
}

#[napi]
pub fn get_printers() -> Vec<Printer> {
  printers::get_printers()
    .into_iter()
    .map(Printer::from)
    .collect()
}

#[napi]
pub fn get_printer_by_name(printer_name: String) -> Option<Printer> {
  printers::get_printer_by_name(printer_name.as_str()).map(Printer::from)
}

#[napi]
pub fn get_default_printer() -> Option<Printer> {
  printers::get_default_printer().map(Printer::from)
}

#[napi]
pub fn print(
  printer_name: String,
  buffer: &[u8],
  job_name: Option<String>,
  options: Vec<PrintOption>,
) -> Result<u64> {
  let printer = printers::get_printer_by_name(printer_name.as_str())
    .ok_or_else(|| napi::Error::from_reason("Printer not found".to_string()))?;

  let job_id = printer
    .print(buffer, job_name.as_deref(), &map_options(&options))
    .map_err(|e| napi::Error::from_reason(format!("Print failed: {}", e)))?;

  Ok(job_id)
}

#[napi]
pub fn print_file(
  printer_name: String,
  file_path: String,
  job_name: Option<String>,
  options: Vec<PrintOption>,
) -> Result<u64> {
  let printer = printers::get_printer_by_name(printer_name.as_str())
    .ok_or_else(|| napi::Error::from_reason("Printer not found".to_string()))?;

  let job_id = printer
    .print_file(
      file_path.as_str(),
      job_name.as_deref(),
      &map_options(&options),
    )
    .map_err(|e| napi::Error::from_reason(format!("Print failed: {}", e)))?;

  Ok(job_id)
}

#[napi]
pub fn get_active_jobs(printer_name: String) -> Vec<PrinterJob> {
  let Some(printer) = printers::get_printer_by_name(printer_name.as_str()) else {
    return Vec::new();
  };

  printer
    .get_active_jobs()
    .into_iter()
    .map(PrinterJob::from)
    .collect()
}

#[napi]
pub fn get_job_history(printer_name: String) -> Vec<PrinterJob> {
  let Some(printer) = printers::get_printer_by_name(printer_name.as_str()) else {
    return Vec::new();
  };

  printer
    .get_job_history()
    .into_iter()
    .map(PrinterJob::from)
    .collect()
}

fn safe_date(time: &SystemTime) -> Option<DateTime<Utc>> {
  let millis = time
    .duration_since(UNIX_EPOCH)
    .ok()? // handles SystemTime before UNIX_EPOCH
    .as_millis() as i64;

  DateTime::from_timestamp_millis(millis)
}

fn map_options(options: &[PrintOption]) -> Vec<(&str, &str)> {
  options
    .iter()
    .map(|opt| (opt.key.as_str(), opt.value.as_str()))
    .collect()
}
