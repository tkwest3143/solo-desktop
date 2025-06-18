<template>
  <div class="h-full bg-white">
    <!-- Page Header -->
    <div class="p-8 border-b border-slate-200">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-3xl font-bold text-slate-800 mb-2">新しいTodoを作成</h1>
          <p class="text-slate-600">効率的なタスク管理のために詳細を入力してください</p>
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

    <!-- Form Content -->
    <div class="p-8">
      <div class="max-w-2xl mx-auto">
        <form @submit.prevent="saveTodo" class="space-y-6">
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
                v-model="todo.due_date"
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
                v-model="todo.due_time"
                type="time"
                class="w-full px-4 py-3 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-colors"
              />
            </div>
          </div>

          <!-- Category and Priority -->
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
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
            <div>
              <label for="priority" class="block text-sm font-semibold text-slate-700 mb-2">
                優先度
              </label>
              <select
                id="priority"
                v-model="todo.priority"
                class="w-full px-4 py-3 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-colors"
              >
                <option value="low">低優先度</option>
                <option value="normal" selected>通常</option>
                <option value="high">高優先度</option>
              </select>
            </div>
          </div>

          <!-- Color Selection -->
          <div>
            <label class="block text-sm font-semibold text-slate-700 mb-2">
              ラベル色
            </label>
            <div class="flex space-x-3">
              <div
                v-for="color in colorOptions"
                :key="color.value"
                @click="todo.color = color.value"
                :class="[
                  'w-8 h-8 rounded-full cursor-pointer border-4 transition-all',
                  todo.color === color.value ? 'border-slate-400 scale-110' : 'border-transparent hover:scale-105'
                ]"
                :style="{ backgroundColor: color.bg }"
                :title="color.name"
              ></div>
            </div>
          </div>

          <!-- Link -->
          <div>
            <label for="link" class="block text-sm font-semibold text-slate-700 mb-2">
              関連リンク
            </label>
            <input
              id="link"
              v-model="todo.link"
              type="url"
              placeholder="https://example.com"
              class="w-full px-4 py-3 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-colors"
            />
          </div>

          <!-- Action Buttons -->
          <div class="flex justify-end space-x-4 pt-6">
            <NuxtLink
              :to="{ name: 'id-todo', params: { id: $route.params.id } }"
              class="px-6 py-3 border border-slate-300 text-slate-700 rounded-lg hover:bg-slate-50 transition-colors"
            >
              キャンセル
            </NuxtLink>
            <button
              type="submit"
              :disabled="loading"
              class="px-6 py-3 bg-blue-500 hover:bg-blue-600 text-white rounded-lg font-medium transition-colors shadow-lg disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <Icon v-if="loading" name="fluent:spinner-ios-20-filled" class="mr-2 animate-spin" />
              <Icon v-else name="fluent:save-20-filled" class="mr-2" />
              {{ loading ? '保存中...' : '保存' }}
            </button>
          </div>
        </form>

        <!-- Tips Section -->
        <div class="mt-8 bg-blue-50 border border-blue-200 rounded-xl p-6">
          <div class="flex items-start space-x-3">
            <Icon name="fluent:lightbulb-20-filled" class="text-blue-500 mt-1" size="1.5em" />
            <div>
              <h3 class="font-semibold text-slate-800 mb-2">効果的なタスク作成のヒント</h3>
              <ul class="text-sm text-slate-600 space-y-1">
                <li>• タスクタイトルは具体的で分かりやすく書く</li>
                <li>• 大きなタスクは小さなタスクに分割する</li>
                <li>• 期日は現実的で達成可能な日時に設定する</li>
                <li>• カテゴリを適切に選択して整理する</li>
                <li>• 優先度を正しく設定して効率的に作業する</li>
              </ul>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { TodoItemRepository } from "~/repositories/tauri-commands/todoItem";
import { TodoCategoryRepository } from "~/repositories/tauri-commands/todoCategory";
import type { TodoCategory, TodoItemForInsert } from "~/models/todo";

export default defineComponent({
  layout: "todo",
  data() {
    return {
      todo: {
        title: "",
        content: "",
        due_date: "",
        due_time: "",
        category_id: "",
        priority: "normal",
        color: "#3b82f6",
        link: "",
      },
      categories: [] as TodoCategory[],
      colorOptions: [
        { name: "ブルー", value: "#3b82f6", bg: "#3b82f6" },
        { name: "レッド", value: "#ef4444", bg: "#ef4444" },
        { name: "グリーン", value: "#10b981", bg: "#10b981" },
        { name: "オレンジ", value: "#f59e0b", bg: "#f59e0b" },
        { name: "パープル", value: "#8b5cf6", bg: "#8b5cf6" },
        { name: "ピンク", value: "#ec4899", bg: "#ec4899" },
        { name: "インディゴ", value: "#6366f1", bg: "#6366f1" },
        { name: "グレー", value: "#6b7280", bg: "#6b7280" },
      ],
      loading: false,
    };
  },
  async mounted() {
    // Set default due date to today
    const today = new Date();
    this.todo.due_date = today.toISOString().split('T')[0];
    
    // Load categories
    await this.loadCategories();
  },
  methods: {
    async loadCategories() {
      try {
        const userId = 1; // TODO: Get from user context/auth
        this.categories = await TodoCategoryRepository.getTodoCategoriesByUserId(userId);
      } catch (error) {
        console.error("Failed to load categories:", error);
      }
    },
    async saveTodo() {
      if (!this.todo.title.trim()) {
        alert("タイトルを入力してください");
        return;
      }

      try {
        this.loading = true;
        
        // Create full datetime string
        const dueDateTime = this.todo.due_time 
          ? `${this.todo.due_date} ${this.todo.due_time}:00`
          : `${this.todo.due_date} 23:59:59`;

        const todoData: TodoItemForInsert = {
          title: this.todo.title.trim(),
          content: this.todo.content.trim() || undefined,
          due_date: dueDateTime,
          category_id: this.todo.category_id ? parseInt(this.todo.category_id) : undefined,
          color: this.todo.color,
          link: this.todo.link.trim() || undefined,
          user_id: 1, // TODO: Get from user context/auth
        };

        console.log("Saving todo data:", todoData);
        
        const result = await TodoItemRepository.createTodoItem(todoData);
        console.log("Save result:", result);
        
        alert("Todoが正常に作成されました！");
        this.$router.push({ name: 'id-todo', params: { id: this.$route.params.id } });
      } catch (error) {
        console.error("Failed to save todo:", error);
        alert("Todoの作成に失敗しました。もう一度試してください。");
      } finally {
        this.loading = false;
      }
    },
  },
});
</script>