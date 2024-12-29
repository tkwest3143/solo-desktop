import { invoke } from "@tauri-apps/api/core";
import { getDateTextForDB, getDayTextForDB } from "~/functions/date";

export class WorkTimeRepository {
  static async getWorkTimeByMonth(userId: number, targetMonth: string) {}
  static async create(record: { userId: number; start: Date }) {
    const res = await invoke<String>("create_work_time", {
      workTime: JSON.stringify({
        user_id: record.userId,
        target_day: getDayTextForDB(record.start),
        start: getDateTextForDB(record.start),
      }),
    });
    console.log(res);
  }
  static async update(record: {
    userId: number;
    targetDay: string;
    start?: Date;
    end?: Date;
    restStart?: Date;
    restEnd?: Date;
    memo?: string;
  }) {
    const start = record.start ? getDateTextForDB(record.start) : undefined;
    const end = record.end ? getDateTextForDB(record.end) : undefined;
    const restStart = record.restStart
      ? getDateTextForDB(record.restStart)
      : undefined;
    const restEnd = record.restEnd
      ? getDateTextForDB(record.restEnd)
      : undefined;

    return await invoke<String>("update_work_time", {
      workTime: JSON.stringify({
        user_id: record.userId,
        target_day: record.targetDay,
        start,
        end,
        rest_start: restStart,
        rest_end: restEnd,
        memo: record.memo,
      }),
    });
  }
}
