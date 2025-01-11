import { differenceInDays } from "date-fns";

export class JapaneseHolidayData {
  constructor(public prop: japaneseHoliday) {}
  isDay(date: Date) {
    return differenceInDays(this.prop.day, date) === 0;
  }
}

export type japaneseHoliday = {
  id: number;
  day: Date;
  subject: string;
};
