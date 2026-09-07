import { format } from "sql-formatter";

export function formatSql(sql: string): string {
  if (sql.trim() === "") {
    return sql;
  }

  try {
    return format(sql, { language: "postgresql", tabWidth: 4 });
  } catch {
    return sql;
  }
}
