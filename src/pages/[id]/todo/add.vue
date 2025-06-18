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
                <option value="1">実装</option>
                <option value="2">デザイン</option>
                <option value="3">会議</option>
                <option value="4">その他</option>
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
              class="px-6 py-3 bg-blue-500 hover:bg-blue-600 text-white rounded-lg font-medium transition-colors shadow-lg"
            >
              <Icon name="fluent:save-20-filled" class="mr-2" />
              保存
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
    };
  },
  mounted() {
    // Set default due date to today
    const today = new Date();
    this.todo.due_date = today.toISOString().split('T')[0];
  },
  methods: {
    saveTodo() {
      // Create full datetime string
      const dueDateTime = this.todo.due_time 
        ? `${this.todo.due_date} ${this.todo.due_time}:00`
        : `${this.todo.due_date} 23:59:59`;

      const todoData = {
        title: this.todo.title,
        content: this.todo.content || null,
        due_date: dueDateTime,
        category_id: this.todo.category_id ? parseInt(this.todo.category_id) : null,
        color: this.todo.color,
        link: this.todo.link || null,
      };

      console.log("Todo data to save:", todoData);
      
      // TODO: Save to backend using Tauri commands
      // For now, just show success message and navigate back
      alert("Todoが作成されました！\n（実際の保存機能は今後実装予定です）");
      this.$router.push({ name: 'id-todo', params: { id: this.$route.params.id } });
    },
  },
});
</script>