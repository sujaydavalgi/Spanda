import type { RuntimeValue } from "../runtime/interpreter.js";

export type CompletionRequest = {
  prompt: string;
  input?: RuntimeValue;
  model: string;
  provider: string;
  temperature: number;
  maxTokens: number;
};

export type DetectionRequest = {
  model: string;
  provider: string;
  frame: RuntimeValue;
};

export type EmbedRequest = {
  model: string;
  provider: string;
  text: string;
};

export interface AIProvider {
  complete(request: CompletionRequest): Promise<RuntimeValue>;
  detect?(request: DetectionRequest): Promise<RuntimeValue>;
  embed?(request: EmbedRequest): Promise<RuntimeValue>;
}
