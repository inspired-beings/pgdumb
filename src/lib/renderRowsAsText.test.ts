import { describe, expect, test } from "bun:test";
import { renderRowsAsText } from "./renderRowsAsText";

describe("renderRowsAsText", () => {
  test("aligns columns to the widest of header or any cell", () => {
    const output = renderRowsAsText(
      ["id", "name"],
      [
        ["1", "Alice"],
        ["2", "Bob"],
      ],
      false
    );

    expect(output).toBe(
      [
        " id | name",
        "----+-------",
        " 1  | Alice",
        " 2  | Bob",
        "(2 rows)",
      ].join("\n")
    );
  });

  test("renders null cells as blank", () => {
    const output = renderRowsAsText(["n"], [[null]], false);

    expect(output).toBe([" n", "---", "", "(1 row)"].join("\n"));
  });

  test("uses singular row wording for exactly one row", () => {
    const output = renderRowsAsText(["n"], [["1"]], false);

    expect(output.endsWith("(1 row)")).toBe(true);
  });

  test("marks the footer as truncated when the truncated flag is set", () => {
    const output = renderRowsAsText(["n"], [["1"]], true);

    expect(output.endsWith("(1 row, truncated)")).toBe(true);
  });

  test("renders a header and separator with zero rows", () => {
    const output = renderRowsAsText(["id", "name"], [], false);

    expect(output).toBe([" id | name", "----+------", "(0 rows)"].join("\n"));
  });
});
