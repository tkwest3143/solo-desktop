import { invoke } from "@tauri-apps/api/core";
import { info } from "@tauri-apps/plugin-log";
import type { user } from "~/models/user";

export class UserRepository {
  static async create(user: { name: string; email: string }) {
    const res = await invoke<String>("create_user", {
      user: JSON.stringify(user),
    });
    info(res.toString());
  }

  static async getAll() {
    const res = await invoke<String>("get_all_users");
    info(res.toString());
    const data = JSON.parse(res as string) as user[];
    return data;
  }
  static async getById(id: number) {
    const res = await invoke<String>("get_user_by_id", { id });
    const data = JSON.parse(res as string) as user;
    return data;
  }
}
