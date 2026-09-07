import { useState, type KeyboardEvent } from "react";
import type { StatementOutcome } from "../../types/query";
import { executeQuery } from "../../api/query";
import { QueryResultBlock } from "./QueryResultBlock";

const MAX_TRANSCRIPT_ENTRIES = 50;

interface TranscriptEntry {
  id: number;
  sql: string;
  outcomes?: StatementOutcome[];
  error?: string;
}

let nextEntryId = 0;

export function QueryPanel() {
  const [sql, setSql] = useState("");
  const [transcript, setTranscript] = useState<TranscriptEntry[]>([]);
  const [running, setRunning] = useState(false);

  async function run() {
    const trimmed = sql.trim();
    if (trimmed === "" || running) return;

    setRunning(true);
    const entry: TranscriptEntry = { id: nextEntryId++, sql: trimmed };
    try {
      entry.outcomes = await executeQuery(trimmed);
    } catch (err) {
      entry.error = String(err);
    } finally {
      setTranscript((prev) => [...prev, entry].slice(-MAX_TRANSCRIPT_ENTRIES));
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
        <button type="button" onClick={() => setTranscript([])} disabled={transcript.length === 0}>
          Clear
        </button>
      </div>

      <div className="query-panel__transcript">
        {transcript.map((entry) => (
          <div key={entry.id} className="query-panel__entry">
            <pre className="query-panel__echo">{entry.sql}</pre>
            {entry.error && <p className="query-result query-result--error">{entry.error}</p>}
            {entry.outcomes?.map((outcome, i) => (
              <QueryResultBlock key={i} outcome={outcome} />
            ))}
          </div>
        ))}
      </div>
    </div>
  );
}
