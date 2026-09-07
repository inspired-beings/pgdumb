import { useEffect, useState } from "react";
import type { StatementOutcome } from "../../types/query";
import { executeQuery } from "../../api/query";
import { QueryResultBlock } from "./QueryResultBlock";
import { SqlEditor } from "./editor/SqlEditor";
import { SplitView } from "../../components/SplitView";
import { getSplitOrientation, setSplitOrientation, type SplitOrientation } from "../../lib/settings";

interface QueryResult {
  outcomes?: StatementOutcome[];
  error?: string;
}

export function QueryPanel() {
  const [sql, setSql] = useState("");
  const [result, setResult] = useState<QueryResult | null>(null);
  const [running, setRunning] = useState(false);
  const [orientation, setOrientation] = useState<SplitOrientation>("vertical");

  useEffect(() => {
    getSplitOrientation().then(setOrientation);
  }, []);

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

  async function toggleOrientation() {
    const next: SplitOrientation = orientation === "vertical" ? "horizontal" : "vertical";
    setOrientation(next);
    await setSplitOrientation(next);
  }

  return (
    <div className="query-panel">
      <div className="query-panel__actions">
        <button onClick={run} disabled={running || sql.trim() === ""}>
          {running ? "Running…" : "Run"}
        </button>
        <button
          className="query-panel__orientation-toggle"
          onClick={toggleOrientation}
          aria-label="Toggle split orientation"
          title={orientation === "vertical" ? "Switch to stacked layout" : "Switch to side-by-side layout"}
        >
          {orientation === "vertical" ? "⬍" : "⬌"}
        </button>
      </div>

      <SplitView
        orientation={orientation}
        first={<SqlEditor value={sql} onChange={setSql} onRun={run} />}
        second={
          <div className="query-panel__result">
            {!result && <p className="query-panel__result-placeholder">Run a query to see its output here.</p>}
            {result?.error && <p className="query-result query-result--error">{result.error}</p>}
            {result?.outcomes?.map((outcome, i) => (
              <QueryResultBlock key={i} outcome={outcome} />
            ))}
          </div>
        }
      />
    </div>
  );
}
