export function renderRowsAsText(
  columns: string[],
  rows: (string | null)[][],
  truncated: boolean
): string {
  const cellText = (value: string | null) => value ?? "";

  const widths = columns.map((column, i) =>
    Math.max(column.length, ...rows.map((row) => cellText(row[i]).length))
  );

  const pad = (text: string, width: number) => text + " ".repeat(Math.max(0, width - text.length));

  const headerLine = " " + columns.map((c, i) => pad(c, widths[i])).join(" | ") + " ";
  const separatorLine = widths.map((w) => "-".repeat(w + 2)).join("+");
  const rowLines = rows.map(
    (row) => " " + row.map((cell, i) => pad(cellText(cell), widths[i])).join(" | ") + " "
  );

  const countLabel = rows.length === 1 ? "row" : "rows";
  const truncatedSuffix = truncated ? ", truncated" : "";
  const footer = `(${rows.length} ${countLabel}${truncatedSuffix})`;

  return [headerLine.trimEnd(), separatorLine, ...rowLines.map((line) => line.trimEnd()), footer].join(
    "\n"
  );
}
