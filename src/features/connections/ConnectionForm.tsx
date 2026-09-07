import { useState, type FormEvent } from "react";
import type { ConnectionProfile, SaveConnectionInput, SslMode } from "../../types/connection";
import { saveConnection, testConnection } from "../../api/connections";

type TestState = { status: "idle" } | { status: "testing" } | { status: "success" } | { status: "error"; message: string };

interface ConnectionFormProps {
  initial?: ConnectionProfile;
  onCancel: () => void;
  onSaved: (profile: ConnectionProfile) => void;
}

export function ConnectionForm({ initial, onCancel, onSaved }: ConnectionFormProps) {
  const [name, setName] = useState(initial?.name ?? "");
  const [host, setHost] = useState(initial?.host ?? "localhost");
  const [port, setPort] = useState(initial?.port ?? 5432);
  const [database, setDatabase] = useState(initial?.database ?? "postgres");
  const [user, setUser] = useState(initial?.user ?? "postgres");
  const [sslMode, setSslMode] = useState<SslMode>(initial?.sslMode ?? "prefer");
  const [rememberPassword, setRememberPassword] = useState(initial?.rememberPassword ?? false);
  const [password, setPassword] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [saving, setSaving] = useState(false);
  const [testState, setTestState] = useState<TestState>({ status: "idle" });

  const alreadyRemembered = initial?.rememberPassword ?? false;
  const needsNewPassword = rememberPassword && !alreadyRemembered && password.trim() === "";

  async function handleTest() {
    setTestState({ status: "testing" });
    try {
      await testConnection({ host, port, database, user, sslMode, password });
      setTestState({ status: "success" });
    } catch (err) {
      setTestState({ status: "error", message: String(err) });
    }
  }

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();
    if (needsNewPassword) {
      setError("Enter a password to remember, or leave the checkbox unchecked.");
      return;
    }

    setError(null);
    setSaving(true);
    try {
      const input: SaveConnectionInput = {
        id: initial?.id,
        name,
        host,
        port,
        database,
        user,
        sslMode,
        rememberPassword,
        password: password.trim() === "" ? undefined : password,
      };
      const saved = await saveConnection(input);
      onSaved(saved);
    } catch (err) {
      setError(String(err));
    } finally {
      setSaving(false);
    }
  }

  return (
    <form className="connection-form" onSubmit={handleSubmit}>
      <h1>{initial ? "Edit connection" : "New connection"}</h1>

      <label>
        Name
        <input value={name} onChange={(e) => setName(e.currentTarget.value)} required />
      </label>

      <label>
        Host
        <input value={host} onChange={(e) => setHost(e.currentTarget.value)} required />
      </label>

      <label>
        Port
        <input
          type="number"
          min={1}
          max={65535}
          value={port}
          onChange={(e) => setPort(Number(e.currentTarget.value))}
          required
        />
      </label>

      <label>
        Database
        <input value={database} onChange={(e) => setDatabase(e.currentTarget.value)} required />
      </label>

      <label>
        User
        <input value={user} onChange={(e) => setUser(e.currentTarget.value)} required />
      </label>

      <label>
        SSL mode
        <select value={sslMode} onChange={(e) => setSslMode(e.currentTarget.value as SslMode)}>
          <option value="disable">Disable</option>
          <option value="prefer">Prefer</option>
          <option value="require">Require</option>
        </select>
      </label>

      <label>
        Password{alreadyRemembered ? " (leave blank to keep the remembered one)" : ""}
        <input
          type="password"
          value={password}
          onChange={(e) => {
            setPassword(e.currentTarget.value);
            setTestState({ status: "idle" });
          }}
        />
      </label>

      <div className="connection-form__test">
        <button
          type="button"
          onClick={handleTest}
          disabled={password.trim() === "" || testState.status === "testing"}
        >
          {testState.status === "testing" ? "Testing…" : "Test"}
        </button>
        {testState.status === "success" && (
          <p className="connection-form__test-result connection-form__test-result--success">Connection succeeded.</p>
        )}
        {testState.status === "error" && (
          <p className="connection-form__test-result connection-form__test-result--error">{testState.message}</p>
        )}
      </div>

      <label>
        <input
          type="checkbox"
          checked={rememberPassword}
          onChange={(e) => setRememberPassword(e.currentTarget.checked)}
        />
        Remember this password (stores it in your system's keychain for this account)
      </label>

      {error && <p className="connection-form__error">{error}</p>}

      <div className="connection-form__actions">
        <button type="submit" disabled={saving}>
          {saving ? "Saving…" : "Save"}
        </button>
        <button type="button" onClick={onCancel}>
          Cancel
        </button>
      </div>
    </form>
  );
}
