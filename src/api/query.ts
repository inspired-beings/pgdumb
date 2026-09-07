import { invoke } from "@tauri-apps/api/core";
import type { StatementOutcome } from "../types/query";

export function executeQuery(sql: string): Promise<StatementOutcome[]> {
  return invoke("execute_query", { sql });
}
