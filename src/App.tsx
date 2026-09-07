import { useState } from "react";
import type { ConnectionProfile } from "./types/connection";
import { ConnectionList } from "./features/connections/ConnectionList";
import { ConnectedView } from "./features/connections/ConnectedView";
import "./App.css";

function App() {
  const [connected, setConnected] = useState<ConnectionProfile | null>(null);

  return (
    <main className="app">
      {connected ? (
        <ConnectedView profile={connected} onDisconnected={() => setConnected(null)} />
      ) : (
        <ConnectionList onConnected={setConnected} />
      )}
    </main>
  );
}

export default App;
