/**
 * Fetch continuity status from a deploy or fleet agent.
 * @module
 */

export type AgentContinuityResponse = {
  ok?: boolean;
  continuity_active?: string | null;
  continuity_successor?: string | null;
  continuity_mode?: string | null;
  continuity_validation?: string | null;
  continuity_engine?: string | null;
  mission_progress_percent?: number | null;
  mission_handoff_from?: string | null;
  last_continuity_commands?: string[];
};

export async function fetchAgentContinuity(agentUrl: string): Promise<AgentContinuityResponse> {
  const base = agentUrl.replace(/\/$/, "");
  const response = await fetch(`${base}/v1/status`, {
    headers: { Accept: "application/json" },
  });
  if (!response.ok) {
    throw new Error(`Agent continuity HTTP ${response.status}`);
  }
  return (await response.json()) as AgentContinuityResponse;
}
