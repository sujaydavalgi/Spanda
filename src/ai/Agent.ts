/**
 * Agent module (ai/Agent.ts).
 * @module
 */

import type { AgentDecl, Stmt } from "../ast/nodes.js";
import type { MemoryStore } from "./MemoryStore.js";

export type AgentRuntime = {
  decl: AgentDecl;
  memory: MemoryStore | null;
};

export function createAgentRuntime(decl: AgentDecl, memory: MemoryStore | null): AgentRuntime {
  // CreateAgentRuntime.
  //
  // Parameters:
  // - `decl` — input value
  // - `memory` — input value
  //
  // Returns:
  // `AgentRuntime`.
  //
  // Options:
  // None.
  //
  // Example:
  // const result = createAgentRuntime(decl, memory);

  return { decl, memory };
}

export function agentToolNames(decl: AgentDecl): string[] {
  // AgentToolNames.
  //
  // Parameters:
  // - `decl` — input value
  //
  // Returns:
  // `string[]`.
  //
  // Options:
  // None.
  //
  // Example:
  // const result = agentToolNames(decl);

  return decl.tools;
}

export function agentUsesModels(decl: AgentDecl): string[] {
  // AgentUsesModels.
  //
  // Parameters:
  // - `decl` — input value
  //
  // Returns:
  // `string[]`.
  //
  // Options:
  // None.
  //
  // Example:
  // const result = agentUsesModels(decl);

  return decl.usesAi;
}

export type PlanExecutor = {
  executeBlock(stmts: Stmt[]): void;
};

export function executeAgentPlan(agent: AgentRuntime, executor: PlanExecutor): void {
  // ExecuteAgentPlan.
  //
  // Parameters:
  // - `agent` — input value
  // - `executor` — input value
  //
  // Returns:
  // Nothing.
  //
  // Options:
  // None.
  //
  // Example:
  // const result = executeAgentPlan(agent, executor);

  executor.executeBlock(agent.decl.planBody);
}
