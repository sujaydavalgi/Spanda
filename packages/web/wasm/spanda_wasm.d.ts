/**
 * spanda_wasm.d module (spanda_wasm.d.ts).
 * @module
 */

export default function init(): Promise<void>;
export function wasm_check(source: string): unknown;
export function wasm_run(source: string, max: number): unknown;
