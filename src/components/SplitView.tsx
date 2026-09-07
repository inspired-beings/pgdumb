import type { ReactNode } from "react";

interface SplitViewProps {
  orientation: "vertical" | "horizontal";
  first: ReactNode;
  second: ReactNode;
}

export function SplitView({ orientation, first, second }: SplitViewProps) {
  return (
    <div className={`split-view split-view--${orientation}`}>
      <div className="split-view__pane">{first}</div>
      <div className="split-view__pane">{second}</div>
    </div>
  );
}
