import { invoke } from "@tauri-apps/api/core";
import { getDateTextForDB } from "~/functions/date";
import { workTimeData, type workTime } from "~/models/workTime";
import { FileDownloadRepository } from "../file";

export class WorkTimeRepository {
  static async getWorkTimeByMonth(userId: string, targetMonth: string) {
    const res = await invoke<String>("get_work_time_by_month", {
      userId,
      targetMonth,
    });
    return JSON.parse(res as string) as workTime[];
  }
  static async create(record: {
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

    return await invoke<String>("create_work_time", {
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
  static async fileCreate(
    data: workTimeData[],
    filename: string,
    columns: string[],
    separator: string,
    isheader: boolean
  ) {
    const csvData = data.map((wt) => {
      const exportColumns = [...wt.exportFileColumn.entries()].filter(
        ([key]) => {
          return columns.includes(key);
        }
      );
      return exportColumns.map(([, value]) => value);
    });
    if (isheader) {
      csvData.unshift(columns);
    }
    await FileDownloadRepository.createText(csvData, filename, separator);
  }
}
