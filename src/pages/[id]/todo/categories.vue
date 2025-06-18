<template>
  <div class="h-full bg-white">
    <!-- Page Header -->
    <div class="p-8 border-b border-slate-200">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-3xl font-bold text-slate-800 mb-2">カテゴリ管理</h1>
          <p class="text-slate-600">タスクのカテゴリを管理・編集</p>
        </div>
        <button
          @click="openCreateModal"
          class="bg-purple-500 hover:bg-purple-600 text-white px-6 py-3 rounded-lg font-medium transition-colors shadow-lg"
        >
          <Icon name="fluent:add-20-filled" class="mr-2" />
          新しいカテゴリ
        </button>
      </div>
    </div>

    <!-- Categories List -->
    <div class="p-8">
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <!-- Dynamic Categories -->
        <div
          v-for="category in categories"
          :key="category.id"
          class="bg-white border-2 border-slate-200 rounded-xl p-6 transition-all hover:shadow-lg"
        >
          <div class="flex items-center justify-between mb-4">
            <div class="flex items-center space-x-3">
              <div 
                class="w-4 h-4 rounded-full"
                :style="{ backgroundColor: category.color || '#6b7280' }"
              ></div>
              <h3 class="text-xl font-semibold text-slate-800">{{ category.name }}</h3>
            </div>
            <div class="flex space-x-2">
              <button class="p-2 text-slate-400 hover:text-blue-500 transition-colors">
                <Icon name="fluent:edit-20-filled" size="1.2em" />
              </button>
              <button 
                @click="deleteCategory(category.id, category.name)"
                class="p-2 text-slate-400 hover:text-red-500 transition-colors"
              >
                <Icon name="fluent:delete-20-filled" size="1.2em" />
              </button>
            </div>
          </div>
          <p class="text-slate-600 mb-4">{{ category.memo || 'メモなし' }}</p>
          <div class="flex items-center justify-between">
            <span class="text-sm text-slate-500">
              {{ categoryTaskCounts[category.id] || 0 }}個のタスク
            </span>
            <span class="bg-blue-100 text-blue-800 text-xs px-2 py-1 rounded-full font-medium">アクティブ</span>
          </div>
        </div>
      </div>

      <!-- Tips Section -->
      <div class="mt-8 bg-purple-50 border border-purple-200 rounded-xl p-6">
        <div class="flex items-start space-x-3">
          <Icon name="fluent:lightbulb-20-filled" class="text-purple-500 mt-1" size="1.5em" />
          <div>
            <h3 class="font-semibold text-slate-800 mb-2">効果的なカテゴリ管理のヒント</h3>
            <ul class="text-sm text-slate-600 space-y-1">
              <li>• カテゴリは5-7個程度に絞って管理しやすくする</li>
              <li>• 色分けを活用して視覚的に区別しやすくする</li>
              <li>• 使用頻度の低いカテゴリは定期的に見直す</li>
              <li>• プロジェクトやチームに合わせてカテゴリを調整する</li>
            </ul>
          </div>
        </div>
      </div>
    </div>

    <!-- Create/Edit Modal -->
    <div v-if="showModal" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white rounded-xl p-8 max-w-md w-full mx-4">
        <div class="flex items-center justify-between mb-6">
          <h2 class="text-2xl font-bold text-slate-800">新しいカテゴリ</h2>
          <button @click="closeModal" class="text-slate-400 hover:text-slate-600">
            <Icon name="fluent:dismiss-20-filled" size="1.5em" />
          </button>
        </div>

        <form @submit.prevent="saveCategory" class="space-y-4">
          <div>
            <label for="category-name" class="block text-sm font-semibold text-slate-700 mb-2">
              カテゴリ名 *
            </label>
            <input
              id="category-name"
              v-model="categoryForm.name"
              type="text"
              placeholder="例: 開発"
              class="w-full px-4 py-3 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent"
              required
            />
          </div>

          <div>
            <label for="category-description" class="block text-sm font-semibold text-slate-700 mb-2">
              説明
            </label>
            <textarea
              id="category-description"
              v-model="categoryForm.description"
              rows="3"
              placeholder="カテゴリの説明を入力..."
              class="w-full px-4 py-3 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent resize-vertical"
            ></textarea>
          </div>

          <div>
            <label class="block text-sm font-semibold text-slate-700 mb-2">
              カテゴリ色
            </label>
            <div class="flex space-x-3">
              <div
                v-for="color in colorOptions"
                :key="color.value"
                @click="categoryForm.color = color.value"
                :class="[
                  'w-8 h-8 rounded-full cursor-pointer border-4 transition-all',
                  categoryForm.color === color.value ? 'border-slate-400 scale-110' : 'border-transparent hover:scale-105'
                ]"
                :style="{ backgroundColor: color.bg }"
                :title="color.name"
              ></div>
            </div>
          </div>

          <div class="flex justify-end space-x-4 pt-4">
            <button
              type="button"
              @click="closeModal"
              class="px-6 py-3 border border-slate-300 text-slate-700 rounded-lg hover:bg-slate-50 transition-colors"
            >
              キャンセル
            </button>
            <button
              type="submit"
              class="px-6 py-3 bg-purple-500 hover:bg-purple-600 text-white rounded-lg font-medium transition-colors shadow-lg"
            >
              <Icon name="fluent:save-20-filled" class="mr-2" />
              保存
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { TodoCategoryRepository } from "~/repositories/tauri-commands/todoCategory";
import { TodoItemRepository } from "~/repositories/tauri-commands/todoItem";
import type { TodoCategory } from "~/models/todo";

export default defineComponent({
  layout: "todo",
  data() {
    return {
      categories: [] as TodoCategory[],
      categoryTaskCounts: {} as Record<number, number>,
      loading: true,
      showModal: false,
      categoryForm: {
        name: "",
        description: "",
        color: "#8b5cf6",
      },
      colorOptions: [
        { name: "パープル", value: "#8b5cf6", bg: "#8b5cf6" },
        { name: "ブルー", value: "#3b82f6", bg: "#3b82f6" },
        { name: "レッド", value: "#ef4444", bg: "#ef4444" },
        { name: "グリーン", value: "#10b981", bg: "#10b981" },
        { name: "オレンジ", value: "#f59e0b", bg: "#f59e0b" },
        { name: "ピンク", value: "#ec4899", bg: "#ec4899" },
        { name: "インディゴ", value: "#6366f1", bg: "#6366f1" },
        { name: "イエロー", value: "#eab308", bg: "#eab308" },
      ],
    };
  },
  async mounted() {
    await this.fetchData();
  },
  methods: {
    async fetchData() {
      try {
        this.loading = true;
        const userId = parseInt(this.$route.params.id as string);
        
        // Fetch categories
        this.categories = await TodoCategoryRepository.getTodoCategoriesByUserId(userId);
        
        // Fetch task counts for each category
        for (const category of this.categories) {
          try {
            const todos = await TodoItemRepository.getTodoItemsByCategoryId(category.id);
            this.categoryTaskCounts[category.id] = todos.length;
          } catch (error) {
            console.error(`Failed to fetch todos for category ${category.id}:`, error);
            this.categoryTaskCounts[category.id] = 0;
          }
        }
      } catch (error) {
        console.error("Failed to fetch categories:", error);
      } finally {
        this.loading = false;
      }
    },
    openCreateModal() {
      this.showModal = true;
      this.categoryForm = {
        name: "",
        description: "",
        color: "#8b5cf6",
      };
    },
    closeModal() {
      this.showModal = false;
    },
    async saveCategory() {
      if (!this.categoryForm.name.trim()) {
        alert("カテゴリ名を入力してください");
        return;
      }

      try {
        const userId = parseInt(this.$route.params.id as string);
        await TodoCategoryRepository.createTodoCategory({
          name: this.categoryForm.name.trim(),
          memo: this.categoryForm.description.trim() || undefined,
          user_id: userId,
        });

        alert(`カテゴリ「${this.categoryForm.name}」が作成されました！`);
        this.closeModal();
        await this.fetchData(); // Refresh the list
      } catch (error) {
        console.error("Failed to create category:", error);
        alert("カテゴリの作成に失敗しました");
      }
    },
    async deleteCategory(id: number, name: string) {
      if (!confirm(`カテゴリ「${name}」を削除しますか？\n\nこのカテゴリに属するタスクも影響を受ける可能性があります。`)) {
        return;
      }

      try {
        await TodoCategoryRepository.deleteTodoCategory(id);
        alert("カテゴリが削除されました");
        await this.fetchData(); // Refresh the list
      } catch (error) {
        console.error("Failed to delete category:", error);
        alert("カテゴリの削除に失敗しました");
      }
    },
  },
});
</script>