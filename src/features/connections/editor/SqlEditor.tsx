import { useEffect, useRef } from "react";
import { EditorState } from "@codemirror/state";
import { EditorView, keymap, lineNumbers, type KeyBinding } from "@codemirror/view";
import { defaultKeymap, history, historyKeymap, indentWithTab } from "@codemirror/commands";
import { indentUnit, syntaxHighlighting } from "@codemirror/language";
import { sql, SQLDialect, PostgreSQL } from "@codemirror/lang-sql";
import { sqlEditorBaseTheme, sqlHighlightStyle } from "./sqlEditorTheme";
import { formatSql } from "./formatSql";

// PostgreSQL's dialect sets doubleDollarQuotedStrings: true, which makes the
// tokenizer swallow an entire $$...$$ body (e.g. a PL/pgSQL function) as one
// opaque string token with no highlighting inside. Reuse PostgreSQL's full
// spec but turn that flag off so the ordinary tokenizer keeps running through
// dollar-quoted bodies instead of collapsing them.
const PostgreSQLNoDollarStrings = SQLDialect.define({
  ...PostgreSQL.spec,
  doubleDollarQuotedStrings: false,
});

interface SqlEditorProps {
  value: string;
  onChange: (value: string) => void;
  onRun?: () => void;
}

export function SqlEditor({ value, onChange, onRun }: SqlEditorProps) {
  const containerRef = useRef<HTMLDivElement | null>(null);
  const onChangeRef = useRef(onChange);
  const onRunRef = useRef(onRun);
  onChangeRef.current = onChange;
  onRunRef.current = onRun;

  useEffect(() => {
    if (!containerRef.current) return;

    const customBindings: KeyBinding[] = [
      {
        key: "Mod-s",
        preventDefault: true,
        run(view) {
          const formatted = formatSql(view.state.doc.toString());
          view.dispatch({
            changes: { from: 0, to: view.state.doc.length, insert: formatted },
          });
          return true;
        },
      },
      {
        key: "Mod-Enter",
        preventDefault: true,
        run() {
          onRunRef.current?.();
          return true;
        },
      },
      indentWithTab,
    ];

    const state = EditorState.create({
      doc: value,
      extensions: [
        lineNumbers(),
        history(),
        indentUnit.of("    "),
        sql({ dialect: PostgreSQLNoDollarStrings }),
        syntaxHighlighting(sqlHighlightStyle),
        sqlEditorBaseTheme,
        keymap.of(customBindings),
        keymap.of([...defaultKeymap, ...historyKeymap]),
        EditorView.updateListener.of((update) => {
          if (update.docChanged) {
            onChangeRef.current(update.state.doc.toString());
          }
        }),
      ],
    });

    const view = new EditorView({ state, parent: containerRef.current });

    return () => {
      view.destroy();
    };
    // Mount-once: the editor owns its own document after creation, same as
    // every CodeMirror-in-React integration guide.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return <div className="sql-editor" ref={containerRef} />;
}
