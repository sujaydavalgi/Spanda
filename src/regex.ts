/**
 * First-class regex compilation, validation, and runtime matching for Spanda.
 * @module
 */

import type { Span } from "./ast/nodes.js";

export type RegexPattern = {
  source: string;
  flags: string;
  span: Span;
};

export type CaptureResult = {
  full: string;
  groups: Record<string, string>;
};

export class RegexError extends Error {
  constructor(
    message: string,
    public line: number,
    public column: number,
  ) {
    super(message);
    this.name = "RegexError";
  }
}

export function compileRegex(pattern: RegexPattern): RegExp {
  // Description:
  //     CompileRegex.
  //
  // Inputs:
  //     pattern: RegexPattern
  //         Caller-supplied pattern.
  //
  // Outputs:
  //     result: RegExp
  //         Return value from `compileRegex`.
  //
  // Example:
  //     const result = compileRegex(pattern);
  // Description:
  //     CompileRegex.
  //
  // Inputs:
  //     pattern: RegexPattern
  //         Caller-supplied pattern.
  //
  // Outputs:
  //     result: RegExp
  //         Return value from `compileRegex`.
  //
  // Example:

  //     const result = compileRegex(pattern);

  let jsFlags = "";
  for (const flag of pattern.flags) {
    if (!"ims".includes(flag)) {
      throw new RegexError(
        `Invalid regex flag '${flag}'; supported flags are i, m, s. Suggestion: remove unsupported flags.`,
        pattern.span.start.line,
        pattern.span.start.column,
      );
    }
    if (!jsFlags.includes(flag)) {
      jsFlags += flag;
    }
  }

  try {
    return new RegExp(pattern.source, jsFlags);
  } catch (err) {
    const message = err instanceof Error ? err.message : String(err);
    throw new RegexError(
      `Invalid regex syntax: ${message}. Suggestion: verify delimiters and escape sequences.`,
      pattern.span.start.line,
      pattern.span.start.column,
    );
  }
}

export function regexMatches(pattern: RegexPattern, text: string): boolean {
  // Description:
  //     RegexMatches.
  //
  // Inputs:
  //     pattern: RegexPattern
  //         Caller-supplied pattern.
  //     text: string
  //         Caller-supplied text.
  //
  // Outputs:
  //     result: boolean
  //         Return value from `regexMatches`.
  //
  // Example:
  //     const result = regexMatches(pattern, text);
  // Description:
  //     RegexMatches.
  //
  // Inputs:
  //     pattern: RegexPattern
  //         Caller-supplied pattern.
  //     text: string
  //         Caller-supplied text.
  //
  // Outputs:
  //     result: boolean
  //         Return value from `regexMatches`.
  //
  // Example:

  //     const result = regexMatches(pattern, text);

  const re = compileRegex(pattern);
  return re.test(text);
}

export function regexFind(pattern: RegexPattern, text: string): string | null {
  // Description:
  //     RegexFind.
  //
  // Inputs:
  //     pattern: RegexPattern
  //         Caller-supplied pattern.
  //     text: string
  //         Caller-supplied text.
  //
  // Outputs:
  //     result: string | null
  //         Return value from `regexFind`.
  //
  // Example:
  //     const result = regexFind(pattern, text);
  // Description:
  //     RegexFind.
  //
  // Inputs:
  //     pattern: RegexPattern
  //         Caller-supplied pattern.
  //     text: string
  //         Caller-supplied text.
  //
  // Outputs:
  //     result: string | null
  //         Return value from `regexFind`.
  //
  // Example:

  //     const result = regexFind(pattern, text);

  const re = compileRegex(pattern);
  const match = re.exec(text);
  return match ? match[0] : null;
}

export function regexReplace(pattern: RegexPattern, text: string, replacement: string): string {
  // Description:
  //     RegexReplace.
  //
  // Inputs:
  //     pattern: RegexPattern
  //         Caller-supplied pattern.
  //     text: string
  //         Caller-supplied text.
  //     replacement: string
  //         Caller-supplied replacement.
  //
  // Outputs:
  //     result: string
  //         Return value from `regexReplace`.
  //
  // Example:
  //     const result = regexReplace(pattern, text, replacement);
  // Description:
  //     RegexReplace.
  //
  // Inputs:
  //     pattern: RegexPattern
  //         Caller-supplied pattern.
  //     text: string
  //         Caller-supplied text.
  //     replacement: string
  //         Caller-supplied replacement.
  //
  // Outputs:
  //     result: string
  //         Return value from `regexReplace`.
  //
  // Example:

  //     const result = regexReplace(pattern, text, replacement);

  const re = compileRegex(pattern);
  return text.replace(re, replacement);
}

export function regexSplit(pattern: RegexPattern, text: string): string[] {
  // Description:
  //     RegexSplit.
  //
  // Inputs:
  //     pattern: RegexPattern
  //         Caller-supplied pattern.
  //     text: string
  //         Caller-supplied text.
  //
  // Outputs:
  //     result: string[]
  //         Return value from `regexSplit`.
  //
  // Example:
  //     const result = regexSplit(pattern, text);
  // Description:
  //     RegexSplit.
  //
  // Inputs:
  //     pattern: RegexPattern
  //         Caller-supplied pattern.
  //     text: string
  //         Caller-supplied text.
  //
  // Outputs:
  //     result: string[]
  //         Return value from `regexSplit`.
  //
  // Example:

  //     const result = regexSplit(pattern, text);

  const re = compileRegex(pattern);
  return text.split(re);
}

export function regexCapture(pattern: RegexPattern, text: string): CaptureResult | null {
  // Description:
  //     RegexCapture.
  //
  // Inputs:
  //     pattern: RegexPattern
  //         Caller-supplied pattern.
  //     text: string
  //         Caller-supplied text.
  //
  // Outputs:
  //     result: CaptureResult | null
  //         Return value from `regexCapture`.
  //
  // Example:
  //     const result = regexCapture(pattern, text);
  // Description:
  //     RegexCapture.
  //
  // Inputs:
  //     pattern: RegexPattern
  //         Caller-supplied pattern.
  //     text: string
  //         Caller-supplied text.
  //
  // Outputs:
  //     result: CaptureResult | null
  //         Return value from `regexCapture`.
  //
  // Example:

  //     const result = regexCapture(pattern, text);

  const re = compileRegex(pattern);
  const match = re.exec(text);
  if (!match) {
    return null;
  }
  const groups: Record<string, string> = {};
  for (const name of Object.keys(match.groups ?? {})) {
    const value = match.groups?.[name];
    if (value !== undefined) {
      groups[name] = value;
    }
  }
  return { full: match[0], groups };
}

export function validateRegexLiteral(source: string, flags: string, span: Span): void {
  // Description:
  //     ValidateRegexLiteral.
  //
  // Inputs:
  //     source: string
  //         Caller-supplied source.
  //     flags: string
  //         Caller-supplied flags.
  //     span: Span
  //         Caller-supplied span.
  //
  // Outputs:
  //     None.
  //
  // Example:
  //     const result = validateRegexLiteral(source, flags, span);
  // Description:
  //     ValidateRegexLiteral.
  //
  // Inputs:
  //     source: string
  //         Caller-supplied source.
  //     flags: string
  //         Caller-supplied flags.
  //     span: Span
  //         Caller-supplied span.
  //
  // Outputs:
  //     None.
  //
  // Example:

  //     const result = validateRegexLiteral(source, flags, span);

  compileRegex({ source, flags, span });
}

export function regexFromLexeme(lexeme: string, span: Span): RegexPattern {
  // Description:
  //     RegexFromLexeme.
  //
  // Inputs:
  //     lexeme: string
  //         Caller-supplied lexeme.
  //     span: Span
  //         Caller-supplied span.
  //
  // Outputs:
  //     result: RegexPattern
  //         Return value from `regexFromLexeme`.
  //
  // Example:
  //     const result = regexFromLexeme(lexeme, span);
  // Description:
  //     RegexFromLexeme.
  //
  // Inputs:
  //     lexeme: string
  //         Caller-supplied lexeme.
  //     span: Span
  //         Caller-supplied span.
  //
  // Outputs:
  //     result: RegexPattern
  //         Return value from `regexFromLexeme`.
  //
  // Example:

  //     const result = regexFromLexeme(lexeme, span);

  const trimmed = lexeme.trimStart().slice(1);
  const slashIdx = trimmed.lastIndexOf("/");
  if (slashIdx < 0) {
    throw new RegexError(`Malformed regex literal '${lexeme}'`, span.start.line, span.start.column);
  }
  const source = trimmed.slice(0, slashIdx);
  const flags = trimmed.slice(slashIdx + 1);
  return { source, flags, span };
}
