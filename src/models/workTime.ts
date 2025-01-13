import { differenceInMinutes, format } from "date-fns";
import type { JapaneseHolidayData } from "./japaneseHoliday";

export class workTimeData {
  constructor(public prop: workTime) {}
  getDayTextWithWeek(holidays: JapaneseHolidayData[]) {
    const date = new Date(this.prop.target_day);
    const day = date.getDay();
    const dayText = ["日", "月", "火", "水", "木", "金", "土"][day];
    const holiday = holidays.find((d) => {
      console.log(
        typeof d.prop.day,
        typeof this.prop.start,
        format(d.prop.day, "HH:mm")
      );
      return d.isDay(date);
    });
    if (holiday) {
      return `${format(date, "yyyy/M/d")}(${dayText}・祝)`;
    }
    return `${format(date, "yyyy/M/d")}(${dayText})`;
  }
  get startByText() {
    if (this.prop.start) {
      return format(this.prop.start, "HH:mm");
    }
    return "00:00";
  }
  get endByText() {
    if (this.prop.end) {
      return format(this.prop.end, "HH:mm");
    }
    return "00:00";
  }
  get restStartByText() {
    if (this.prop.rest_start) {
      return format(this.prop.rest_start, "HH:mm");
    }
    return "00:00";
  }
  get restEndByText() {
    if (this.prop.rest_end) {
      return format(this.prop.rest_end, "HH:mm");
    }
    return "00:00";
  }
  get restDurationByText() {
    if (this.prop.rest_start && this.prop.rest_end) {
      const diff = differenceInMinutes(
        this.prop.rest_end,
        this.prop.rest_start
      );
      const hours = Math.floor(diff / 60);
      const minutes = diff % 60;
      return `${hours.toString().padStart(2, "0")}:${minutes
        .toString()
        .padStart(2, "0")}`;
    }
    return "00:00";
  }

  get workDurationByMinute() {
    if (this.prop.start && this.prop.end) {
      let diff = differenceInMinutes(this.prop.end, this.prop.start);
      if (this.prop.rest_start && this.prop.rest_end) {
        const restDiff = differenceInMinutes(
          this.prop.rest_end,
          this.prop.rest_start
        );
        diff -= restDiff;
      }
      return diff;
    }
    return 0;
  }

  get workDurationByText() {
    if (this.prop.start && this.prop.end) {
      let diff = differenceInMinutes(this.prop.end, this.prop.start);
      if (this.prop.rest_start && this.prop.rest_end) {
        const restDiff = differenceInMinutes(
          this.prop.rest_end,
          this.prop.rest_start
        );
        diff -= restDiff;
      }
      const hours = Math.floor(diff / 60);
      const minutes = diff % 60;
      return `${hours.toString().padStart(2, "0")}:${minutes
        .toString()
        .padStart(2, "0")}`;
    }
    return "00:00";
  }
  set start(date: Date) {
    this.prop.start = date;
  }
  set end(date: Date) {
    this.prop.end = date;
  }
  set restStart(date: Date) {
    this.prop.rest_start = date;
  }
  set restEnd(date: Date) {
    this.prop.rest_end = date;
  }
  set memo(text: string) {
    this.prop.memo = text;
  }
}

export type workTime = {
  id: number;
  target_day: string;
  start?: Date;
  end?: Date;
  rest_start?: Date;
  rest_end?: Date;
  memo?: string;
  created_at: Date;
  updated_at: Date;
};
