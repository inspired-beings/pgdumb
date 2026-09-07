import { useState, type KeyboardEvent } from "react";
import type { StatementOutcome } from "../../types/query";
import { executeQuery } from "../../api/query";
import { QueryResultBlock } from "./QueryResultBlock";

interface QueryResult {
  outcomes?: StatementOutcome[];
  error?: string;
}

export function QueryPanel() {
  const [sql, setSql] = useState("");
  const [result, setResult] = useState<QueryResult | null>(null);
  const [running, setRunning] = useState(false);

  async function run() {
    const trimmed = sql.trim();
    if (trimmed === "" || running) return;

    setRunning(true);
    try {
      const outcomes = await executeQuery(trimmed);
      setResult({ outcomes });
    } catch (err) {
      setResult({ error: String(err) });
    } finally {
      setRunning(false);
    }
  }

  function handleKeyDown(event: KeyboardEvent<HTMLTextAreaElement>) {
    if ((event.metaKey || event.ctrlKey) && event.key === "Enter") {
      event.preventDefault();
      run();
    }
  }

  return (
    <div className="query-panel">
      <textarea
        className="query-panel__input"
        value={sql}
        onChange={(e) => setSql(e.currentTarget.value)}
        onKeyDown={handleKeyDown}
        placeholder="SQL to run (Ctrl/Cmd+Enter to run)"
        rows={4}
      />

      <div className="query-panel__actions">
        <button onClick={run} disabled={running || sql.trim() === ""}>
          {running ? "Running…" : "Run"}
        </button>
      </div>

      {result && (
        <div className="query-panel__result">
          {result.error && <p className="query-result query-result--error">{result.error}</p>}
          {result.outcomes?.map((outcome, i) => (
            <QueryResultBlock key={i} outcome={outcome} />
          ))}
        </div>
      )}
    </div>
  );
}
