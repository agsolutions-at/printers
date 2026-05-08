#!/usr/bin/env node
import { confirm, input, select } from '@inquirer/prompts';
import fs from 'node:fs';
import { TextEncoder } from 'node:util';

import {
  cancelJob,
  getActiveJobs,
  getDefaultPrinter,
  getJobHistory,
  getPrinterByName,
  getPrinters,
  pauseJob,
  print,
  printFile,
  restartJob,
  resumeJob,
} from './index.js';

const STATE_GLYPH = {
  READY: '🟢',
  PRINTING: '🖨️ ',
  PAUSED: '⏸️ ',
  OFFLINE: '⚫',
  UNKNOWN: '❔',
};

const RAW_OPTION = { key: 'document-format', value: 'application/vnd.cups-raw' };

function pretty(obj) {
  return JSON.stringify(obj, (_, v) => (typeof v === 'bigint' ? v.toString() : v), 2);
}

function jobsTable(jobs) {
  if (jobs.length === 0) {
    console.log('  (none)');
    return;
  }
  console.table(
    jobs.map((j) => ({
      id: j.id.toString(),
      name: j.name,
      state: j.state,
      mediaType: j.mediaType,
      createdAt: j.createdAt?.toISOString(),
      processedAt: j.processedAt?.toISOString() ?? '',
      completedAt: j.completedAt?.toISOString() ?? '',
    })),
  );
}

async function pickPrinter() {
  const printers = getPrinters();
  if (printers.length === 0) {
    console.log('No printers found on this system.');
    return null;
  }

  const def = getDefaultPrinter();

  return select({
    message: 'Select printer',
    default: def?.name,
    pageSize: 15,
    choices: printers.map((p) => ({
      name: `${STATE_GLYPH[p.state] ?? '·'} ${p.name}${def?.name === p.name ? '  (default)' : ''}`,
      value: p.name,
      description: [p.driverName, p.location].filter(Boolean).join(' — '),
    })),
  });
}

async function actionPrintText(printerName) {
  const text = await input({ message: 'Text to print', validate: (v) => v.length > 0 || 'cannot be empty' });
  const raw = await confirm({ message: 'Send as RAW (CUPS-raw)?', default: false });
  const buffer = new TextEncoder().encode(text);
  const jobId = print(printerName, buffer, 'cli-text', raw ? [RAW_OPTION] : []);
  console.log(`✓ Submitted text job — ID ${jobId.toString()}`);
}

async function actionPrintFile(printerName) {
  const filePath = await input({
    message: 'Path to file',
    validate: (v) => fs.existsSync(v) || 'file does not exist',
  });
  const raw = await confirm({ message: 'Send as RAW (CUPS-raw)?', default: false });
  const jobId = printFile(printerName, filePath, 'cli-file', raw ? [RAW_OPTION] : []);
  console.log(`✓ Submitted file job — ID ${jobId.toString()}`);
}

function actionActiveJobs(printerName) {
  console.log(`\nActive jobs on ${printerName}:`);
  jobsTable(getActiveJobs(printerName));
}

function actionJobHistory(printerName) {
  console.log(`\nJob history for ${printerName}:`);
  jobsTable(getJobHistory(printerName));
}

function actionPrinterDetails(printerName) {
  const p = getPrinterByName(printerName);
  if (!p) {
    console.log('Printer not found.');
    return;
  }
  console.log('\n' + pretty(p));
}

async function actionManageJob(printerName) {
  const jobs = getActiveJobs(printerName);
  if (jobs.length === 0) {
    console.log('  (no active jobs)');
    return;
  }
  const jobId = await select({
    message: 'Pick job',
    choices: jobs.map((j) => ({
      name: `${j.id.toString()} — ${j.name} [${j.state}]`,
      value: j.id,
    })),
  });
  const action = await select({
    message: 'Action',
    choices: [
      { name: 'Pause', value: 'pause' },
      { name: 'Resume', value: 'resume' },
      { name: 'Restart', value: 'restart' },
      { name: 'Cancel', value: 'cancel' },
    ],
  });
  switch (action) {
    case 'pause': pauseJob(printerName, jobId); break;
    case 'resume': resumeJob(printerName, jobId); break;
    case 'restart': restartJob(printerName, jobId); break;
    case 'cancel': cancelJob(printerName, jobId); break;
  }
  console.log(`✓ ${action} on job ${jobId.toString()}`);
}

async function actionMenu(printerName) {
  while (true) {
    const action = await select({
      message: `[${printerName}] What now?`,
      pageSize: 10,
      choices: [
        { name: 'Print text', value: 'text' },
        { name: 'Print file', value: 'file' },
        { name: 'View active jobs', value: 'active' },
        { name: 'View job history', value: 'history' },
        { name: 'Manage active job (pause/resume/restart/cancel)', value: 'manage' },
        { name: 'Show printer details', value: 'details' },
        { name: 'Switch printer', value: 'switch' },
        { name: 'Quit', value: 'quit' },
      ],
    });

    if (action === 'switch') return 'switch';
    if (action === 'quit') return 'quit';

    try {
      switch (action) {
        case 'text': await actionPrintText(printerName); break;
        case 'file': await actionPrintFile(printerName); break;
        case 'active': actionActiveJobs(printerName); break;
        case 'history': actionJobHistory(printerName); break;
        case 'manage': await actionManageJob(printerName); break;
        case 'details': actionPrinterDetails(printerName); break;
      }
    } catch (e) {
      console.error(`✗ ${e?.message ?? e}`);
    }
    console.log('');
  }
}

async function main() {
  console.log('🖨️  Printer CLI\n');
  while (true) {
    const printerName = await pickPrinter();
    if (!printerName) return;
    const result = await actionMenu(printerName);
    if (result === 'quit') {
      console.log('Bye.');
      return;
    }
  }
}

main().catch((e) => {
  // Ctrl-C / Esc on a prompt throws ExitPromptError — treat as clean exit.
  if (e?.name === 'ExitPromptError') return;
  console.error(e);
  process.exit(1);
});
