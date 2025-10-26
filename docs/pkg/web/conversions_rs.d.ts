/* tslint:disable */
/* eslint-disable */
/**
 * Convert length units
 */
export function convert_length_wasm(value: number, from: string, to: string): ConversionResult;
/**
 * Convert weight/mass units
 */
export function convert_weight_wasm(value: number, from: string, to: string): ConversionResult;
/**
 * Convert temperature units
 */
export function convert_temperature_wasm(value: number, from: string, to: string): ConversionResult;
/**
 * Convert volume units
 */
export function convert_volume_wasm(value: number, from: string, to: string): ConversionResult;
/**
 * Convert time units
 */
export function convert_time_wasm(value: number, from: string, to: string): ConversionResult;
/**
 * Convert electric current units
 */
export function convert_current_wasm(value: number, from: string, to: string): ConversionResult;
/**
 * Convert amount of substance units
 */
export function convert_substance_wasm(value: number, from: string, to: string): ConversionResult;
/**
 * Convert luminous intensity units
 */
export function convert_luminous_intensity_wasm(value: number, from: string, to: string): ConversionResult;
/**
 * Convert area units
 */
export function convert_area_wasm(value: number, from: string, to: string): ConversionResult;
/**
 * Initialize the WASM module
 */
export function main(): void;
/**
 * Get supported units for a given conversion type
 */
export function get_supported_units(conversion_type: string): string[];
/**
 * Result type for WASM conversion operations
 */
export class ConversionResult {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  readonly success: boolean;
  readonly value: number;
  readonly error: string | undefined;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_conversionresult_free: (a: number, b: number) => void;
  readonly conversionresult_success: (a: number) => number;
  readonly conversionresult_value: (a: number) => number;
  readonly conversionresult_error: (a: number) => [number, number];
  readonly convert_length_wasm: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly convert_weight_wasm: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly convert_temperature_wasm: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly convert_volume_wasm: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly convert_time_wasm: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly convert_current_wasm: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly convert_substance_wasm: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly convert_luminous_intensity_wasm: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly convert_area_wasm: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly get_supported_units: (a: number, b: number) => [number, number];
  readonly main: () => void;
  readonly __wbindgen_export_0: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_drop_slice: (a: number, b: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
