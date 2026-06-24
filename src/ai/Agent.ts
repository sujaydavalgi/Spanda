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
  // Description:
  //     CreateAgentRuntime.
  //
  // Inputs:
  //     decl: AgentDecl
  //         Caller-supplied decl.
  //     memory: MemoryStore | null
  //         Caller-supplied memory.
  //
  // Outputs:
  //     result: AgentRuntime
  //         Return value from `createAgentRuntime`.
  //
  // Example:
  //     const result = createAgentRuntime(decl, memory);
  // Description:
  //     CreateAgentRuntime.
  //
  // Inputs:
  //     decl: AgentDecl
  //         Caller-supplied decl.
  //     memory: MemoryStore | null
  //         Caller-supplied memory.
  //
  // Outputs:
  //     result: AgentRuntime
  //         Return value from `createAgentRuntime`.
  //
  // Example:
  //     const result = createAgentRuntime(decl, memory);

  // const result = createAgentRuntime(decl, memory);
  return { decl, memory };
}

export function agentToolNames(decl: AgentDecl): string[] {
  // Description:
  //     AgentToolNames.
  //
  // Inputs:
  //     decl: AgentDecl
  //         Caller-supplied decl.
  //
  // Outputs:
  //     result: string[]
  //         Return value from `agentToolNames`.
  //
  // Example:
  //     const result = agentToolNames(decl);
  // Description:
  //     AgentToolNames.
  //
  // Inputs:
  //     decl: AgentDecl
  //         Caller-supplied decl.
  //
  // Outputs:
  //     result: string[]
  //         Return value from `agentToolNames`.
  //
  // Example:
  //     const result = agentToolNames(decl);

  // const result = agentToolNames(decl);
  return decl.tools;
}

export function agentUsesModels(decl: AgentDecl): string[] {
  // Description:
  //     AgentUsesModels.
  //
  // Inputs:
  //     decl: AgentDecl
  //         Caller-supplied decl.
  //
  // Outputs:
  //     result: string[]
  //         Return value from `agentUsesModels`.
  //
  // Example:
  //     const result = agentUsesModels(decl);
  // Description:
  //     AgentUsesModels.
  //
  // Inputs:
  //     decl: AgentDecl
  //         Caller-supplied decl.
  //
  // Outputs:
  //     result: string[]
  //         Return value from `agentUsesModels`.
  //
  // Example:
  //     const result = agentUsesModels(decl);

  // const result = agentUsesModels(decl);
  return decl.usesAi;
}

export type PlanExecutor = {
  executeBlock(stmts: Stmt[]): void;
};

export function executeAgentPlan(agent: AgentRuntime, executor: PlanExecutor): void {
  // Description:
  //     ExecuteAgentPlan.
  //
  // Inputs:
  //     agent: AgentRuntime
  //         Caller-supplied agent.
  //     executor: PlanExecutor
  //         Caller-supplied executor.
  //
  // Outputs:
  //     None.
  //
  // Example:
  //     const result = executeAgentPlan(agent, executor);
  // Description:
  //     ExecuteAgentPlan.
  //
  // Inputs:
  //     agent: AgentRuntime
  //         Caller-supplied agent.
  //     executor: PlanExecutor
  //         Caller-supplied executor.
  //
  // Outputs:
  //     None.
  //
  // Example:
  //     const result = executeAgentPlan(agent, executor);

  // const result = executeAgentPlan(agent, executor);
  executor.executeBlock(agent.decl.planBody);
}
