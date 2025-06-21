import { invoke } from "@tauri-apps/api/core";
import { getDateTextForDB } from "~/functions/date";
import type { workSetting } from "~/models/workSetting";

export class WorkSettingRepository {
  static async create(record: {
    userId: number;
    title: string;
    start: Date;
    end: Date;
    restStart: Date;
    restEnd: Date;
    workingUnit: number;
    memo?: string;
    isDefault: boolean;
  }) {
    const start = getDateTextForDB(record.start);
    const end = getDateTextForDB(record.end);
    const restStart = getDateTextForDB(record.restStart);
    const restEnd = getDateTextForDB(record.restEnd);
    return await invoke<String>("create_work_time_setting", {
      body: JSON.stringify({
        user_id: record.userId,
        title: record.title,
        start,
        end,
        rest_start: restStart,
        rest_end: restEnd,
        working_unit: record.workingUnit,
        memo: record.memo,
        is_default: record.isDefault,
      }),
    });
  }

  static async update(record: {
    id: number;
    title?: string;
    start?: Date;
    end?: Date;
    restStart?: Date;
    restEnd?: Date;
    workingUnit?: number;
    memo?: string;
    isDefault?: boolean;
  }) {
    const start = record.start ? getDateTextForDB(record.start) : undefined;
    const end = record.end ? getDateTextForDB(record.end) : undefined;
    const restStart = record.restStart
      ? getDateTextForDB(record.restStart)
      : undefined;
    const restEnd = record.restEnd
      ? getDateTextForDB(record.restEnd)
      : undefined;
    return await invoke<String>("update_work_time_setting", {
      body: JSON.stringify({
        id: record.id,
        title: record.title,
        start,
        end,
        rest_start: restStart,
        rest_end: restEnd,
        working_unit: record.workingUnit,
        memo: record.memo,
        is_default: record.isDefault,
      }),
    });
  }
  static async getByUserId(userId: number) {
    const res = await invoke<String>("get_work_setting_by_user_id", {
      userId,
    });
    const data = JSON.parse(res as string) as workSetting[];
    return data;
  }

  static async getById(id: number, userId: number) {
    const workSettings = await this.getByUserId(userId);
    const workSetting = workSettings.find(ws => ws.id === id);
    if (!workSetting) {
      throw new Error(`Work setting with ID ${id} not found for user ${userId}`);
    }
    return workSetting;
  }
}
