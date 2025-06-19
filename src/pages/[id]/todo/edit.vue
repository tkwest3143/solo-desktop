<template>
  <div class="h-full bg-white">
    <!-- Page Header -->
    <div class="p-8 border-b border-slate-200">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-3xl font-bold text-slate-800 mb-2">Todoを編集</h1>
          <p class="text-slate-600">タスクの詳細を編集してください</p>
        </div>
        <NuxtLink
          :to="{ name: 'id-todo', params: { id: $route.params.id } }"
          class="text-slate-600 hover:text-slate-800 px-4 py-2 rounded-lg transition-colors"
        >
          <Icon name="fluent:arrow-left-20-filled" class="mr-2" />
          戻る
        </NuxtLink>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="loading" class="p-8 text-center">
      <div class="text-slate-500">読み込み中...</div>
    </div>

    <!-- Form Content -->
    <div v-else-if="todo" class="p-8">
      <div class="max-w-2xl mx-auto">
        <form @submit.prevent="updateTodo" class="space-y-6">
          <!-- Title -->
          <div>
            <label for="title" class="block text-sm font-semibold text-slate-700 mb-2">
              タスクタイトル *
            </label>
            <input
              id="title"
              v-model="todo.title"
              type="text"
              placeholder="例: プロジェクトの進捗報告書作成"
              class="w-full px-4 py-3 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-colors"
              required
            />
          </div>

          <!-- Description -->
          <div>
            <label for="content" class="block text-sm font-semibold text-slate-700 mb-2">
              詳細説明
            </label>
            <textarea
              id="content"
              v-model="todo.content"
              rows="4"
              placeholder="タスクの詳細な説明を入力してください..."
              class="w-full px-4 py-3 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-colors resize-vertical"
            ></textarea>
          </div>

          <!-- Due Date and Time -->
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label for="due_date" class="block text-sm font-semibold text-slate-700 mb-2">
                期日 *
              </label>
              <input
                id="due_date"
                v-model="dueDateFormatted"
                type="date"
                class="w-full px-4 py-3 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-colors"
                required
              />
            </div>
            <div>
              <label for="due_time" class="block text-sm font-semibold text-slate-700 mb-2">
                時刻
              </label>
              <input
                id="due_time"
                v-model="dueTimeFormatted"
                type="time"
                class="w-full px-4 py-3 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-colors"
              />
            </div>
          </div>

          <!-- Category -->
          <div>
            <label for="category" class="block text-sm font-semibold text-slate-700 mb-2">
              カテゴリ
            </label>
            <select
              id="category"
              v-model="todo.category_id"
              class="w-full px-4 py-3 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-colors"
            >
              <option value="">カテゴリを選択</option>
              <option v-for="category in categories" :key="category.id" :value="category.id">
                {{ category.name }}
              </option>
            </select>
          </div>

          <!-- Submit Button -->
          <div class="flex justify-end space-x-4">
            <NuxtLink
              :to="{ name: 'id-todo', params: { id: $route.params.id } }"
              class="px-6 py-3 border border-slate-300 text-slate-700 rounded-lg hover:bg-slate-50 transition-colors"
            >
              キャンセル
            </NuxtLink>
            <button
              type="submit"
              :disabled="saving"
              class="px-6 py-3 bg-blue-500 hover:bg-blue-600 disabled:bg-slate-400 text-white rounded-lg transition-colors"
            >
              {{ saving ? '保存中...' : 'Todoを更新' }}
            </button>
          </div>
        </form>
      </div>
    </div>

    <!-- Error State -->
    <div v-else class="p-8 text-center">
      <div class="text-red-500">タスクが見つかりませんでした</div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { TodoItemRepository } from "~/repositories/tauri-commands/todoItem";
import { TodoCategoryRepository } from "~/repositories/tauri-commands/todoCategory";
import type { TodoItem, TodoCategory } from "~/models/todo";

definePageMeta({
  layout: 'todo'
});

export default defineComponent({
  data() {
    return {
      todo: null as TodoItem | null,
      categories: [] as TodoCategory[],
      loading: true,
      saving: false,
    };
  },
  computed: {
    dueDateFormatted: {
      get(): string {
        if (!this.todo?.due_date) return '';
        const date = new Date(this.todo.due_date);
        return date.toISOString().split('T')[0];
      },
      set(value: string) {
        if (this.todo && value) {
          // Preserve existing time if it exists
          const existingDate = new Date(this.todo.due_date);
          const newDate = new Date(value);
          newDate.setHours(existingDate.getHours(), existingDate.getMinutes());
          this.todo.due_date = newDate.toISOString();
        }
      }
    },
    dueTimeFormatted: {
      get(): string {
        if (!this.todo?.due_date) return '';
        const date = new Date(this.todo.due_date);
        return date.toTimeString().slice(0, 5);
      },
      set(value: string) {
        if (this.todo && value) {
          const [hours, minutes] = value.split(':');
          const date = new Date(this.todo.due_date);
          date.setHours(parseInt(hours), parseInt(minutes));
          this.todo.due_date = date.toISOString();
        }
      }
    }
  },
  async mounted() {
    await this.loadData();
  },
  methods: {
    async loadData() {
      try {
        this.loading = true;
        const todoId = parseInt(this.$route.query.id as string);
        const userId = parseInt(this.$route.params.id as string);

        if (!todoId) {
          throw new Error('Todo ID is required');
        }

        // Load todo and categories in parallel
        const [todo, categories] = await Promise.all([
          TodoItemRepository.getTodoItem(todoId),
          TodoCategoryRepository.getTodoCategoriesByUserId(userId)
        ]);

        this.todo = todo;
        this.categories = categories;
      } catch (error) {
        console.error("Failed to load data:", error);
        alert("データの読み込みに失敗しました");
      } finally {
        this.loading = false;
      }
    },
    async updateTodo() {
      if (!this.todo) return;

      try {
        this.saving = true;
        await TodoItemRepository.updateTodoItem(this.todo);
        
        alert("Todoを更新しました");
        await this.$router.push({ 
          name: 'id-todo', 
          params: { id: this.$route.params.id } 
        });
      } catch (error) {
        console.error("Failed to update todo:", error);
        alert("Todoの更新に失敗しました");
      } finally {
        this.saving = false;
      }
    },
  },
});
</script>