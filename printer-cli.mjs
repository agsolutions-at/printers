import {
  getActiveJobs,
  getDefaultPrinter,
  getJobHistory,
  getPrinterByName,
  getPrinters,
  print,
  printFile
} from './index.js'

import readline from 'readline';
import {TextEncoder} from 'util';
import fs from 'fs';

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

function ask(question) {
  return new Promise(resolve => rl.question(question, resolve));
}

async function main() {
  console.log("Welcome to the printer CLI!\n");
  const printers = getPrinters();

  if (!printers || printers.length === 0) {
    console.log("No printers found.");
    rl.close();
    return;
  }

  console.log(`Default printer: ${getDefaultPrinter().name}\n`);

  console.log("Available Printers:");
  printers.forEach((printer, index) => {
    console.log(`=> ${index + 1}:`)
    console.log(printer);
  });

  let choice = await ask("\nSelect a printer by number: ");
  let index = parseInt(choice) - 1;

  if (isNaN(index) || index < 0 || index >= printers.length) {
    console.log("Invalid selection.");
    rl.close();
    return;
  }

  const selectedPrinter = printers[index];
  console.log(
      `Selected printer: ${selectedPrinter.name} (should match result of getPrinterByName: ${getPrinterByName(
          selectedPrinter.name).name})`);

  console.log(`
Choose a mode:
1. Print text
2. Print file
3. View active jobs
4. View job history
  `);

  const mode = await ask("Enter mode number: ");

  switch (mode) {
    case '1': {
      const text = await ask("Enter text to print: ");
      const encoder = new TextEncoder();
      const uint8Array = encoder.encode(text);
      try {
        print(selectedPrinter.name, uint8Array, "cli-job-text");
        console.log("Text print succeeded.");
      } catch (e) {
        console.log("Error printing text:", e);
      }
      break;
    }

    case '2': {
      const filePath = await ask("Enter full path to file: ");
      if (!fs.existsSync(filePath)) {
        console.log("Hint: File does not exist.");
      }
      try {
        printFile(selectedPrinter.name, filePath, "cli-job-file");
        console.log("File print succeeded.");
      } catch (e) {
        console.log("Error printing file:", e);
      }
      break;
    }

    case '3': {
      const jobs = getActiveJobs(selectedPrinter.name);
      console.log("Active Jobs:\n", jobs);
      break;
    }

    case '4': {
      const history = getJobHistory(selectedPrinter.name);
      console.log("Job History:\n", history);
      break;
    }

    default:
      console.log("Invalid mode selected.");
  }

  rl.close();
}

main();
