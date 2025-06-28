import { BaseDirectory, writeFile } from "@tauri-apps/plugin-fs";
import type { TodoItem } from "~/models/todo";

export interface IcsTodo {
  title: string;
  description?: string;
  start: Date;
  end: Date;
}

export function todoToIcsEvent(todo: IcsTodo): string {
  const pad = (n: number) => n.toString().padStart(2, "0");
  const formatDate = (date: Date) =>
    `${date.getUTCFullYear()}${pad(date.getUTCMonth() + 1)}${pad(
      date.getUTCDate()
    )}T${pad(date.getUTCHours())}${pad(date.getUTCMinutes())}${pad(
      date.getUTCSeconds()
    )}`;
  return [
    "BEGIN:VEVENT",
    `SUMMARY:${todo.title}`,
    `DESCRIPTION:${todo.description || ""}`,
    `DTSTART:${formatDate(todo.start)}`,
    `DTEND:${formatDate(todo.end)}`,
    "END:VEVENT",
    "",
  ].join("\r\n");
}

export async function createIcsFile(
  todos: IcsTodo[],
  fileName: string
): Promise<string> {
  let ics = "BEGIN:VCALENDAR\r\nVERSION:2.0\r\nPRODID:-//solo-desktop//EN\r\n";
  for (const todo of todos) {
    ics += todoToIcsEvent(todo);
  }
  ics += "END:VCALENDAR\r\n";
  // ダウンロードディレクトリに保存
  const encoder = new TextEncoder();
  const icsBytes = encoder.encode(ics);
  await writeFile(fileName, icsBytes, { baseDir: BaseDirectory.Download });
  return ics;
}

export function todoItemToIcsTodo(todo: TodoItem): IcsTodo {
  const start = new Date(todo.due_date);
  // 終了時刻は+1時間（必要に応じて調整）
  const end = new Date(start.getTime() + 60 * 60 * 1000);
  return {
    title: todo.title,
    description: todo.content,
    start,
    end,
  };
}

export function createIcsFileFromTodoItems(
  todos: TodoItem[],
  fileName: string
): Promise<string> {
  const icsTodos = todos.map(todoItemToIcsTodo);
  return createIcsFile(icsTodos, fileName);
}
