import { describe, expect, test } from "bun:test";
import { formatSql } from "./formatSql";

describe("formatSql", () => {
  test("reformats messy SQL into sql-formatter's canonical layout", () => {
    const output = formatSql("select id,name from users where id=1;");

    expect(output).toBe("select\n    id,\n    name\nfrom\n    users\nwhere\n    id = 1;");
  });

  test("is a no-op on SQL that's already in canonical layout", () => {
    const clean = "select\n    id,\n    name\nfrom\n    users\nwhere\n    id = 1;";

    expect(formatSql(clean)).toBe(clean);
  });

  test("passes whitespace-only input through unchanged", () => {
    const whitespace = "   \n\t  ";

    expect(formatSql(whitespace)).toBe(whitespace);
  });

  test("passes malformed SQL through unchanged instead of throwing", () => {
    const malformed = "select * from (((";

    expect(formatSql(malformed)).toBe(malformed);
  });
});
