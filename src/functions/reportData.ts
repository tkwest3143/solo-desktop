import { workTimeData } from "~/models/workTime";

export interface MonthlyReportData {
  year: number;
  month: number;
  totalMinutes: number;
  totalHours: number;
  workDays: number;
  averageHoursPerDay: number;
}

export interface YearlyReportData {
  year: number;
  totalMinutes: number;
  totalHours: number;
  workDays: number;
  averageHoursPerDay: number;
  monthlyData: MonthlyReportData[];
}

export function aggregateWorkTimeByMonth(workTimes: workTimeData[]): MonthlyReportData[] {
  const monthlyMap = new Map<string, MonthlyReportData>();

  workTimes.forEach((workTime) => {
    const date = new Date(workTime.prop.target_day);
    const year = date.getFullYear();
    const month = date.getMonth() + 1;
    const key = `${year}-${month}`;
    
    const workMinutes = workTime.workDurationByMinute;
    
    if (workMinutes > 0) {
      if (!monthlyMap.has(key)) {
        monthlyMap.set(key, {
          year,
          month,
          totalMinutes: 0,
          totalHours: 0,
          workDays: 0,
          averageHoursPerDay: 0,
        });
      }
      
      const monthData = monthlyMap.get(key)!;
      monthData.totalMinutes += workMinutes;
      monthData.workDays += 1;
    }
  });

  // Calculate derived values
  monthlyMap.forEach((data) => {
    data.totalHours = data.totalMinutes / 60;
    data.averageHoursPerDay = data.workDays > 0 ? data.totalHours / data.workDays : 0;
  });

  return Array.from(monthlyMap.values()).sort((a, b) => {
    if (a.year !== b.year) return a.year - b.year;
    return a.month - b.month;
  });
}

export function aggregateWorkTimeByYear(workTimes: workTimeData[]): YearlyReportData[] {
  const monthlyData = aggregateWorkTimeByMonth(workTimes);
  const yearlyMap = new Map<number, YearlyReportData>();

  monthlyData.forEach((monthData) => {
    const year = monthData.year;
    
    if (!yearlyMap.has(year)) {
      yearlyMap.set(year, {
        year,
        totalMinutes: 0,
        totalHours: 0,
        workDays: 0,
        averageHoursPerDay: 0,
        monthlyData: [],
      });
    }
    
    const yearData = yearlyMap.get(year)!;
    yearData.totalMinutes += monthData.totalMinutes;
    yearData.workDays += monthData.workDays;
    yearData.monthlyData.push(monthData);
  });

  // Calculate derived values
  yearlyMap.forEach((data) => {
    data.totalHours = data.totalMinutes / 60;
    data.averageHoursPerDay = data.workDays > 0 ? data.totalHours / data.workDays : 0;
    data.monthlyData.sort((a, b) => a.month - b.month);
  });

  return Array.from(yearlyMap.values()).sort((a, b) => a.year - b.year);
}