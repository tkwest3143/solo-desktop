import { formatDate } from "date-fns";
export function getMonthTextForDB(date: Date): string {
  return formatDate(date, "yyyy-MM");
}
export function getDayTextForDB(date: Date): string {
  return formatDate(date, "yyyy-MM-dd");
}

export function getDateTextForDB(date: Date): string {
  return formatDate(date, "yyyy-MM-dd HH:mm:00");
}
