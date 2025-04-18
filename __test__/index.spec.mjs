import test from 'ava'

import {getPrinterByName, print, printFile} from '../index.js'
import {TextEncoder} from "util";

test('getPrinterByName - returns null if the printer is unknown', (t) => {
  t.is(getPrinterByName("some unknown printer name"), null);
});

test('print - throws when the arguments do not match', (t) => {
  const error = t.throws(() => print("wrong num or args"))
  t.is(error.message, "Get TypedArray info failed")
});

test('print - throws when the printer name is unknown', (t) => {
  const text = "print content";
  const encoder = new TextEncoder();
  const uint8Array = encoder.encode(text);
  const error = t.throws(() => print("some unknown printer name", uint8Array))
  t.is(error.message, "Printer not found")
});

test('printFile - throws when the arguments do not match', (t) => {
  const error = t.throws(() => printFile("wrong num or args"))
  t.is(error.message, "Failed to convert JavaScript value `Undefined` into rust type `String`")
});

test('printFile - throws when the printer name is unknown', (t) => {
  const error = t.throws(() => printFile("some unknown printer name", "some file path"))
  t.is(error.message, "Printer not found")
});
