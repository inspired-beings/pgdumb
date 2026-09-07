import { EditorView } from "@codemirror/view";
import { HighlightStyle } from "@codemirror/language";
import { tags } from "@lezer/highlight";

export const sqlEditorBaseTheme = EditorView.theme({
  "&": {
    height: "100%",
    fontSize: "1em",
    borderRadius: "8px",
  },
  ".cm-scroller": {
    fontFamily: "ui-monospace, SFMono-Regular, Menlo, Consolas, monospace",
  },
  ".cm-content": {
    padding: "0.5em 0.8em",
  },
});

export const sqlHighlightStyle = HighlightStyle.define([
  { tag: tags.keyword, class: "cm-sql-keyword" },
  { tag: tags.string, class: "cm-sql-string" },
  { tag: tags.number, class: "cm-sql-number" },
  { tag: tags.comment, class: "cm-sql-comment" },
  { tag: tags.operator, class: "cm-sql-operator" },
]);
