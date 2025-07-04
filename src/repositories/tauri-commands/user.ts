import { invoke } from "@tauri-apps/api/core";
import { info } from "@tauri-apps/plugin-log";
import { getDateTextForDB } from "~/functions/date";
import type { user } from "~/models/user";

export class UserRepository {
  static async create(user: { name: string; email: string }) {
    const res = await invoke<String>("create_user", {
      user: JSON.stringify(user),
    });
  }

  static async update(user: {
    id: number;
    name?: string;
    email?: string;
    lastLoginTime?: Date;
  }) {
    const res = await invoke<String>("update_user", {
      user: JSON.stringify({
        id: user.id,
        name: user.name,
        email: user.email,
        last_login_time: user.lastLoginTime
          ? getDateTextForDB(user.lastLoginTime)
          : undefined,
      }),
    });
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
  static async delete(id: number) {
    await invoke<String>("delete_user", { id });
  }
}
