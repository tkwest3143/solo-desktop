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
        <!-- Sample Categories -->
        <div class="bg-white border-2 border-slate-200 rounded-xl p-6 transition-all hover:shadow-lg">
          <div class="flex items-center justify-between mb-4">
            <div class="flex items-center space-x-3">
              <div class="w-4 h-4 rounded-full bg-red-400"></div>
              <h3 class="text-xl font-semibold text-slate-800">実装</h3>
            </div>
            <div class="flex space-x-2">
              <button class="p-2 text-slate-400 hover:text-blue-500 transition-colors">
                <Icon name="fluent:edit-20-filled" size="1.2em" />
              </button>
              <button class="p-2 text-slate-400 hover:text-red-500 transition-colors">
                <Icon name="fluent:delete-20-filled" size="1.2em" />
              </button>
            </div>
          </div>
          <p class="text-slate-600 mb-4">開発・実装に関するタスク</p>
          <div class="flex items-center justify-between">
            <span class="text-sm text-slate-500">3個のタスク</span>
            <span class="bg-red-100 text-red-800 text-xs px-2 py-1 rounded-full font-medium">アクティブ</span>
          </div>
        </div>

        <div class="bg-white border-2 border-slate-200 rounded-xl p-6 transition-all hover:shadow-lg">
          <div class="flex items-center justify-between mb-4">
            <div class="flex items-center space-x-3">
              <div class="w-4 h-4 rounded-full bg-green-400"></div>
              <h3 class="text-xl font-semibold text-slate-800">デザイン</h3>
            </div>
            <div class="flex space-x-2">
              <button class="p-2 text-slate-400 hover:text-blue-500 transition-colors">
                <Icon name="fluent:edit-20-filled" size="1.2em" />
              </button>
              <button class="p-2 text-slate-400 hover:text-red-500 transition-colors">
                <Icon name="fluent:delete-20-filled" size="1.2em" />
              </button>
            </div>
          </div>
          <p class="text-slate-600 mb-4">UI/UXデザインに関するタスク</p>
          <div class="flex items-center justify-between">
            <span class="text-sm text-slate-500">2個のタスク</span>
            <span class="bg-green-100 text-green-800 text-xs px-2 py-1 rounded-full font-medium">アクティブ</span>
          </div>
        </div>

        <div class="bg-white border-2 border-slate-200 rounded-xl p-6 transition-all hover:shadow-lg">
          <div class="flex items-center justify-between mb-4">
            <div class="flex items-center space-x-3">
              <div class="w-4 h-4 rounded-full bg-blue-400"></div>
              <h3 class="text-xl font-semibold text-slate-800">会議</h3>
            </div>
            <div class="flex space-x-2">
              <button class="p-2 text-slate-400 hover:text-blue-500 transition-colors">
                <Icon name="fluent:edit-20-filled" size="1.2em" />
              </button>
              <button class="p-2 text-slate-400 hover:text-red-500 transition-colors">
                <Icon name="fluent:delete-20-filled" size="1.2em" />
              </button>
            </div>
          </div>
          <p class="text-slate-600 mb-4">会議・打ち合わせに関するタスク</p>
          <div class="flex items-center justify-between">
            <span class="text-sm text-slate-500">4個のタスク</span>
            <span class="bg-blue-100 text-blue-800 text-xs px-2 py-1 rounded-full font-medium">アクティブ</span>
          </div>
        </div>

        <div class="bg-white border-2 border-slate-200 rounded-xl p-6 transition-all hover:shadow-lg">
          <div class="flex items-center justify-between mb-4">
            <div class="flex items-center space-x-3">
              <div class="w-4 h-4 rounded-full bg-yellow-400"></div>
              <h3 class="text-xl font-semibold text-slate-800">レビュー</h3>
            </div>
            <div class="flex space-x-2">
              <button class="p-2 text-slate-400 hover:text-blue-500 transition-colors">
                <Icon name="fluent:edit-20-filled" size="1.2em" />
              </button>
              <button class="p-2 text-slate-400 hover:text-red-500 transition-colors">
                <Icon name="fluent:delete-20-filled" size="1.2em" />
              </button>
            </div>
          </div>
          <p class="text-slate-600 mb-4">コードレビュー・品質チェック</p>
          <div class="flex items-center justify-between">
            <span class="text-sm text-slate-500">1個のタスク</span>
            <span class="bg-yellow-100 text-yellow-800 text-xs px-2 py-1 rounded-full font-medium">アクティブ</span>
          </div>
        </div>

        <div class="bg-white border-2 border-slate-200 rounded-xl p-6 transition-all hover:shadow-lg">
          <div class="flex items-center justify-between mb-4">
            <div class="flex items-center space-x-3">
              <div class="w-4 h-4 rounded-full bg-purple-400"></div>
              <h3 class="text-xl font-semibold text-slate-800">学習</h3>
            </div>
            <div class="flex space-x-2">
              <button class="p-2 text-slate-400 hover:text-blue-500 transition-colors">
                <Icon name="fluent:edit-20-filled" size="1.2em" />
              </button>
              <button class="p-2 text-slate-400 hover:text-red-500 transition-colors">
                <Icon name="fluent:delete-20-filled" size="1.2em" />
              </button>
            </div>
          </div>
          <p class="text-slate-600 mb-4">技術学習・研修に関するタスク</p>
          <div class="flex items-center justify-between">
            <span class="text-sm text-slate-500">2個のタスク</span>
            <span class="bg-purple-100 text-purple-800 text-xs px-2 py-1 rounded-full font-medium">アクティブ</span>
          </div>
        </div>

        <div class="bg-slate-50 border-2 border-slate-200 rounded-xl p-6 transition-all hover:shadow-lg opacity-60">
          <div class="flex items-center justify-between mb-4">
            <div class="flex items-center space-x-3">
              <div class="w-4 h-4 rounded-full bg-slate-400"></div>
              <h3 class="text-xl font-semibold text-slate-800">その他</h3>
            </div>
            <div class="flex space-x-2">
              <button class="p-2 text-slate-400 hover:text-blue-500 transition-colors">
                <Icon name="fluent:edit-20-filled" size="1.2em" />
              </button>
              <button class="p-2 text-slate-400 hover:text-red-500 transition-colors">
                <Icon name="fluent:delete-20-filled" size="1.2em" />
              </button>
            </div>
          </div>
          <p class="text-slate-600 mb-4">その他の雑多なタスク</p>
          <div class="flex items-center justify-between">
            <span class="text-sm text-slate-500">0個のタスク</span>
            <span class="bg-slate-100 text-slate-800 text-xs px-2 py-1 rounded-full font-medium">非アクティブ</span>
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

export default defineComponent({
  layout: "todo",
  data() {
    return {
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
  methods: {
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
    saveCategory() {
      console.log("Category data to save:", this.categoryForm);
      
      // TODO: Save to backend using Tauri commands
      // For now, just show success message
      alert(`カテゴリ「${this.categoryForm.name}」が作成されました！\n（実際の保存機能は今後実装予定です）`);
      this.closeModal();
    },
  },
});
</script>