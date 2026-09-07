export type StatementOutcome =
  | { type: "rows"; columns: string[]; rows: (string | null)[][]; truncated: boolean }
  | { type: "commandTag"; tag: string }
  | { type: "error"; message: string };
