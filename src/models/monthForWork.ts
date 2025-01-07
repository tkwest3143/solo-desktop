import { getDayTextForDB, getMonthTextForDB } from "~/functions/date";
import { workTimeData } from "./workTime";

export class MonthForWork {
  constructor(public month: Date, private workTimes: workTimeData[]) {}
  get byText() {
    return getMonthTextForDB(this.month);
  }
  get yearText() {
    return this.month.getFullYear().toString();
  }

  get monthText() {
    return (this.month.getMonth() + 1).toString();
  }

  get monthWorkTimes() {
    const data = this.days.map((day) => {
      const workTime = this.workTimes.find((wt) => {
        return wt.prop.target_day === day;
      });
      console.log(workTime);
      const startDayTime = new Date(day);
      startDayTime.setHours(0, 0, 0, 0);
      return new workTimeData({
        id: workTime?.prop.id ?? -1,
        target_day: day,
        start: workTime?.prop.start ?? startDayTime,
        end: workTime?.prop.end ?? startDayTime,
        rest_start: workTime?.prop.rest_start ?? startDayTime,
        rest_end: workTime?.prop.rest_end ?? startDayTime,
        created_at: workTime?.prop.created_at ?? startDayTime,
        updated_at: workTime?.prop.updated_at ?? startDayTime,
      });
    });
    console.log(data);
    return data;
  }

  get days() {
    // 月の日付リストをyyyy-mm-ddの形式で作成
    const days = [] as string[];
    const lastDate = new Date(
      this.month.getFullYear(),
      this.month.getMonth() + 1,
      0
    );
    for (let i = 1; i <= lastDate.getDate(); i++) {
      const day = new Date(this.month.getFullYear(), this.month.getMonth(), i);
      days.push(getDayTextForDB(day));
    }
    return days;
  }
  get previousMonth() {
    return new MonthForWork(
      new Date(this.month.getFullYear(), this.month.getMonth() - 1),
      this.workTimes
    );
  }
  get nextMonth() {
    return new MonthForWork(
      new Date(this.month.getFullYear(), this.month.getMonth() + 1),
      this.workTimes
    );
  }
  set workTimeDatas(workTimes: workTimeData[]) {
    this.workTimes = workTimes;
  }
}
