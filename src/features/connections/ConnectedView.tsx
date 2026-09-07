import type { ConnectionProfile } from "../../types/connection";
import { disconnectFromDatabase } from "../../api/connections";
import { QueryPanel } from "./QueryPanel";

interface ConnectedViewProps {
  profile: ConnectionProfile;
  onDisconnected: () => void;
}

export function ConnectedView({ profile, onDisconnected }: ConnectedViewProps) {
  async function handleDisconnect() {
    await disconnectFromDatabase();
    onDisconnected();
  }

  return (
    <div className="connected-view">
      <div className="connected-view__header">
        <div>
          <h1>{profile.name}</h1>
          <p>
            {profile.user}@{profile.host}:{profile.port}/{profile.database}
          </p>
        </div>
        <button onClick={handleDisconnect}>Disconnect</button>
      </div>

      <QueryPanel />
    </div>
  );
}
