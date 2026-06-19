import type { AgentDecl, Stmt } from "../ast/nodes.js";
import type { MemoryStore } from "./MemoryStore.js";

export type AgentRuntime = {
  decl: AgentDecl;
  memory: MemoryStore | null;
};

export function createAgentRuntime(decl: AgentDecl, memory: MemoryStore | null): AgentRuntime {
  return { decl, memory };
}

export function agentToolNames(decl: AgentDecl): string[] {
  return decl.tools;
}

export function agentUsesModels(decl: AgentDecl): string[] {
  return decl.usesAi;
}

export type PlanExecutor = {
  executeBlock(stmts: Stmt[]): void;
};

export function executeAgentPlan(agent: AgentRuntime, executor: PlanExecutor): void {
  executor.executeBlock(agent.decl.planBody);
}
