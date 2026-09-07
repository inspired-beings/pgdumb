import type { ConnectionProfile } from "../../types/connection";
import { disconnectFromDatabase } from "../../api/connections";

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
      <h1>Connected to {profile.name}</h1>
      <p>
        {profile.user}@{profile.host}:{profile.port}/{profile.database}
      </p>
      <button onClick={handleDisconnect}>Disconnect</button>
    </div>
  );
}
