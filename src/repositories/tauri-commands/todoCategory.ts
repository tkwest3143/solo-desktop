import { invoke } from "@tauri-apps/api/core";
import type { TodoCategory, TodoCategoryForInsert, TodoCategoryForUpdate } from "~/models/todo";

export class TodoCategoryRepository {
  static async getTodoCategoriesByUserId(userId: number): Promise<TodoCategory[]> {
    const res = await invoke<string>("get_todo_categories_by_user_id", { userId });
    return JSON.parse(res) as TodoCategory[];
  }

  static async createTodoCategory(category: TodoCategoryForInsert): Promise<string> {
    const res = await invoke<string>("create_todo_category", { 
      category: JSON.stringify(category) 
    });
    return res;
  }

  static async updateTodoCategory(category: TodoCategoryForUpdate): Promise<string> {
    const res = await invoke<string>("update_todo_category", { 
      category: JSON.stringify(category) 
    });
    return res;
  }

  static async deleteTodoCategory(id: number): Promise<string> {
    const res = await invoke<string>("delete_todo_category", { id });
    return res;
  }
}