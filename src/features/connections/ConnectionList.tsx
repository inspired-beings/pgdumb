import { useEffect, useState } from "react";
import type { ConnectionProfile } from "../../types/connection";
import { connectToDatabase, deleteConnection, listConnections } from "../../api/connections";
import { ConnectionForm } from "./ConnectionForm";

interface ConnectionListProps {
  onConnected: (profile: ConnectionProfile) => void;
}

type FormState = { mode: "closed" } | { mode: "create" } | { mode: "edit"; profile: ConnectionProfile };

export function ConnectionList({ onConnected }: ConnectionListProps) {
  const [connections, setConnections] = useState<ConnectionProfile[]>([]);
  const [form, setForm] = useState<FormState>({ mode: "closed" });
  const [passwordPromptId, setPasswordPromptId] = useState<string | null>(null);
  const [passwordInput, setPasswordInput] = useState("");
  const [connectingId, setConnectingId] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  async function refresh() {
    setConnections(await listConnections());
  }

  useEffect(() => {
    refresh();
  }, []);

  async function handleConnect(profile: ConnectionProfile, password?: string) {
    setError(null);
    setConnectingId(profile.id);
    try {
      await connectToDatabase(profile.id, password);
      onConnected(profile);
    } catch (err) {
      setError(String(err));
    } finally {
      setConnectingId(null);
      setPasswordPromptId(null);
      setPasswordInput("");
    }
  }

  function startConnect(profile: ConnectionProfile) {
    if (profile.rememberPassword) {
      handleConnect(profile);
    } else {
      setPasswordPromptId(profile.id);
      setPasswordInput("");
    }
  }

  async function handleDelete(id: string) {
    if (!confirm("Delete this connection?")) return;
    await deleteConnection(id);
    await refresh();
  }

  if (form.mode !== "closed") {
    return (
      <ConnectionForm
        initial={form.mode === "edit" ? form.profile : undefined}
        onCancel={() => setForm({ mode: "closed" })}
        onSaved={async () => {
          setForm({ mode: "closed" });
          await refresh();
        }}
      />
    );
  }

  return (
    <div className="connection-list">
      <div className="connection-list__header">
        <h1>pgDumb</h1>
        <button onClick={() => setForm({ mode: "create" })}>New connection</button>
      </div>

      {error && <p className="connection-list__error">{error}</p>}

      {connections.length === 0 ? (
        <p>No saved connections yet.</p>
      ) : (
        <ul className="connection-list__items">
          {connections.map((profile) => (
            <li key={profile.id} className="connection-list__item">
              <div>
                <strong>{profile.name}</strong>
                <div>
                  {profile.user}@{profile.host}:{profile.port}/{profile.database}
                </div>
              </div>

              {passwordPromptId === profile.id ? (
                <form
                  onSubmit={(e) => {
                    e.preventDefault();
                    handleConnect(profile, passwordInput);
                  }}
                >
                  <input
                    type="password"
                    autoFocus
                    placeholder="Password"
                    value={passwordInput}
                    onChange={(e) => setPasswordInput(e.currentTarget.value)}
                  />
                  <button type="submit" disabled={connectingId === profile.id}>
                    Connect
                  </button>
                  <button type="button" onClick={() => setPasswordPromptId(null)}>
                    Cancel
                  </button>
                </form>
              ) : (
                <div className="connection-list__actions">
                  <button onClick={() => startConnect(profile)} disabled={connectingId === profile.id}>
                    {connectingId === profile.id ? "Connecting…" : "Connect"}
                  </button>
                  <button onClick={() => setForm({ mode: "edit", profile })}>Edit</button>
                  <button onClick={() => handleDelete(profile.id)}>Delete</button>
                </div>
              )}
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
