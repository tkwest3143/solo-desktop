export interface TodoItem {
  id: number;
  title: string;
  content?: string;
  link?: string;
  color?: string;
  priority?: string;
  due_date: string;
  created_at: string;
  updated_at: string;
  category_id?: number;
  user_id: number;
  status?: string;
}

export interface TodoCategory {
  id: number;
  name: string;
  memo?: string;
  color?: string;
  user_id: number;
  created_at: string;
  updated_at: string;
}

export interface TodoItemForInsert {
  title: string;
  content?: string;
  link?: string;
  color?: string;
  priority?: string;
  due_date: string;
  category_id?: number;
  user_id: number;
  status?: string;
}

export interface TodoItemForUpdate {
  id: number;
  title?: string;
  content?: string;
  link?: string;
  color?: string;
  priority?: string;
  due_date: string;
  category_id?: number;
  user_id: number;
  status?: string;
}

export interface TodoCategoryForInsert {
  name: string;
  memo?: string;
  color?: string;
  user_id: number;
}

export interface TodoCategoryForUpdate {
  id: number;
  name?: string;
  memo?: string;
  color?: string;
  user_id: number;
}
