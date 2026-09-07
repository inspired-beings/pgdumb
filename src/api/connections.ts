import { invoke } from "@tauri-apps/api/core";
import type { ConnectionProfile, SaveConnectionInput, TestConnectionInput } from "../types/connection";

export function listConnections(): Promise<ConnectionProfile[]> {
  return invoke("list_connections");
}

export function saveConnection(input: SaveConnectionInput): Promise<ConnectionProfile> {
  return invoke("save_connection", { input });
}

export function deleteConnection(id: string): Promise<void> {
  return invoke("delete_connection", { id });
}

export function connectToDatabase(id: string, password?: string): Promise<ConnectionProfile> {
  return invoke("connect", { id, password });
}

export function testConnection(input: TestConnectionInput): Promise<void> {
  return invoke("test_connection", { input });
}

export function disconnectFromDatabase(): Promise<void> {
  return invoke("disconnect");
}
