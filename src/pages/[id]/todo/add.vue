<template>
  <div class="h-full bg-white dark:bg-slate-900 transition-colors">
    <!-- Page Header -->
    <div class="p-8 border-b border-slate-200 dark:border-slate-700">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-3xl font-bold text-slate-800 dark:text-slate-100 mb-2">新しいTodoを作成</h1>
          <p class="text-slate-600 dark:text-slate-400">効率的なタスク管理のために詳細を入力してください</p>
        </div>
        <NuxtLink
          :to="{ name: 'id-todo', params: { id: $route.params.id } }"
          class="text-slate-600 dark:text-slate-400 hover:text-slate-800 dark:hover:text-slate-200 px-4 py-2 rounded-lg transition-colors"
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
            <label for="title" class="block text-sm font-semibold text-slate-700 dark:text-slate-300 mb-2">
              タスクタイトル *
            </label>
            <input
              id="title"
              v-model="todo.title"
              type="text"
              placeholder="例: プロジェクトの進捗報告書作成"
              class="w-full px-4 py-3 border border-slate-300 dark:border-slate-600 bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 focus:border-transparent transition-colors placeholder:text-slate-500 dark:placeholder:text-slate-400"
              required
            />
          </div>

          <!-- Description -->
          <div>
            <label for="content" class="block text-sm font-semibold text-slate-700 dark:text-slate-300 mb-2">
              詳細説明
            </label>
            <textarea
              id="content"
              v-model="todo.content"
              rows="4"
              placeholder="タスクの詳細な説明を入力してください..."
              class="w-full px-4 py-3 border border-slate-300 dark:border-slate-600 bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 focus:border-transparent transition-colors resize-vertical placeholder:text-slate-500 dark:placeholder:text-slate-400"
            ></textarea>
          </div>

          <!-- Due Date and Time -->
          <CustomDateTimePicker
            :date="todo.due_date"
            :time="todo.due_time"
            :required="true"
            @update:date="todo.due_date = $event"
            @update:time="todo.due_time = $event"
          />

          <!-- Category and Priority -->
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label for="category" class="block text-sm font-semibold text-slate-700 mb-2">
                カテゴリ
              </label>
              <CustomSelect
                v-model="todo.category_id"
                :options="categoryOptions"
                placeholder="カテゴリを選択"
              />
            </div>
            <div>
              <label for="priority" class="block text-sm font-semibold text-slate-700 mb-2">
                優先度
              </label>
              <CustomSelect
                v-model="todo.priority"
                :options="priorityOptions"
                placeholder="優先度を選択"
              />
            </div>
          </div>

          <!-- Color Selection -->
          <div>
            <label class="block text-sm font-semibold text-slate-700 dark:text-slate-300 mb-2">
              ラベル色
            </label>
            <div class="flex space-x-3">
              <div
                v-for="color in colorOptions"
                :key="color.value"
                @click="todo.color = color.value"
                :class="[
                  'w-8 h-8 rounded-full cursor-pointer border-4 transition-all',
                  todo.color === color.value ? 'border-slate-400 dark:border-slate-500 scale-110' : 'border-transparent hover:scale-105'
                ]"
                :style="{ backgroundColor: color.bg }"
                :title="color.name"
              ></div>
            </div>
          </div>

          <!-- Link -->
          <div>
            <label for="link" class="block text-sm font-semibold text-slate-700 dark:text-slate-300 mb-2">
              関連リンク
            </label>
            <input
              id="link"
              v-model="todo.link"
              type="url"
              placeholder="https://example.com"
              class="w-full px-4 py-3 border border-slate-300 dark:border-slate-600 bg-white dark:bg-slate-800 text-slate-900 dark:text-slate-100 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 focus:border-transparent transition-colors placeholder:text-slate-500 dark:placeholder:text-slate-400"
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
import CustomSelect from "~/components/CustomSelect.vue";
import CustomDateTimePicker from "~/components/CustomDateTimePicker.vue";
import type { SelectOption } from "~/components/CustomSelect.vue";

definePageMeta({
  layout: 'todo'
});

export default defineComponent({
  components: {
    CustomSelect,
    CustomDateTimePicker,
  },
  setup() {
    const { getColorOptions, defaultColor } = useTheme();
    
    return {
      colorOptions: getColorOptions(),
      defaultColor
    };
  },
  data() {
    return {
      todo: {
        title: "",
        content: "",
        due_date: "",
        due_time: "",
        category_id: "",
        priority: "normal",
        color: this.defaultColor,
        link: "",
      },
      categories: [] as TodoCategory[],
      loading: false,
    };
  },
  computed: {
    categoryOptions(): SelectOption[] {
      return this.categories.map(category => ({
        value: category.id,
        label: category.name,
        color: category.color
      }));
    },
    priorityOptions(): SelectOption[] {
      return [
        { value: 'low', label: '低優先度' },
        { value: 'normal', label: '通常' },
        { value: 'high', label: '高優先度' },
      ];
    }
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
        const userId = parseInt(this.$route.params.id as string);
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
          priority: this.todo.priority,
          link: this.todo.link.trim() || undefined,
          user_id: parseInt(this.$route.params.id as string),
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