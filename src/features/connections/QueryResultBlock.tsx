import type { StatementOutcome } from "../../types/query";
import { renderRowsAsText } from "../../lib/renderRowsAsText";

interface QueryResultBlockProps {
  outcome: StatementOutcome;
}

export function QueryResultBlock({ outcome }: QueryResultBlockProps) {
  if (outcome.type === "rows") {
    return <pre className="query-result">{renderRowsAsText(outcome.columns, outcome.rows, outcome.truncated)}</pre>;
  }

  if (outcome.type === "commandTag") {
    return <p className="query-result query-result--tag">{outcome.tag}</p>;
  }

  return <p className="query-result query-result--error">{outcome.message}</p>;
}
