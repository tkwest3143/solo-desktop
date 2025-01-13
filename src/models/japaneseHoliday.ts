import { differenceInDays } from "date-fns";

export class JapaneseHolidayData {
  constructor(public prop: japaneseHoliday) {}
  isDay(date: Date) {
    const targetDate = new Date(
      date.getFullYear(),
      date.getMonth(),
      date.getDate()
    );
    const propDate = new Date(
      this.prop.day.getFullYear(),
      this.prop.day.getMonth(),
      this.prop.day.getDate()
    );
    return differenceInDays(propDate, targetDate) === 0;
  }
}

export type japaneseHoliday = {
  id: number;
  day: Date;
  subject: string;
};
