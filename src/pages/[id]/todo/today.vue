<template>
  <div class="h-full bg-white">
    <!-- Page Header -->
    <div class="p-8 border-b border-slate-200">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-3xl font-bold text-slate-800 mb-2">今日のTodo</h1>
          <p class="text-slate-600">{{ currentDate }}のタスク一覧</p>
        </div>
        <NuxtLink
          :to="{ name: 'id-todo-add', params: { id: $route.params.id } }"
          class="bg-blue-500 hover:bg-blue-600 text-white px-6 py-3 rounded-lg font-medium transition-colors shadow-lg"
        >
          <Icon name="fluent:add-20-filled" class="mr-2" />
          新規作成
        </NuxtLink>
      </div>
    </div>

    <!-- Task List -->
    <div class="p-8">
      <!-- Filter/Sort Options -->
      <div class="flex items-center justify-between mb-6">
        <div class="flex items-center space-x-4">
          <select class="border border-slate-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500">
            <option>すべて</option>
            <option>未完了</option>
            <option>完了済み</option>
          </select>
          <select class="border border-slate-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500">
            <option>すべてのカテゴリ</option>
            <option>実装</option>
            <option>デザイン</option>
            <option>会議</option>
          </select>
        </div>
        <div class="text-sm text-slate-500">
          5件のタスク
        </div>
      </div>

      <!-- Task Items -->
      <div class="space-y-4">
        <!-- High Priority Task -->
        <div class="bg-red-50 border-2 border-red-200 rounded-xl p-6 transition-all hover:shadow-lg">
          <div class="flex items-start space-x-4">
            <div class="mt-1">
              <input type="checkbox" class="w-5 h-5 text-red-500 rounded border-2 border-red-300 focus:ring-red-500" />
            </div>
            <div class="flex-1">
              <div class="flex items-center space-x-3 mb-2">
                <h3 class="text-lg font-semibold text-slate-800">緊急バグ修正</h3>
                <span class="bg-red-100 text-red-800 text-xs px-2 py-1 rounded-full font-medium">高優先度</span>
                <div class="w-3 h-3 rounded-full bg-red-400"></div>
              </div>
              <p class="text-slate-600 mb-3">ユーザーログイン機能の重要なバグを修正する必要があります</p>
              <div class="flex items-center space-x-4 text-sm text-slate-500">
                <span>期限: 今日 18:00</span>
                <span>カテゴリ: 実装</span>
                <span>作成日: 2024-06-18</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Normal Task -->
        <div class="bg-white border-2 border-slate-200 rounded-xl p-6 transition-all hover:shadow-lg hover:border-slate-300">
          <div class="flex items-start space-x-4">
            <div class="mt-1">
              <input type="checkbox" class="w-5 h-5 text-blue-500 rounded border-2 border-slate-300 focus:ring-blue-500" />
            </div>
            <div class="flex-1">
              <div class="flex items-center space-x-3 mb-2">
                <h3 class="text-lg font-semibold text-slate-800">UIデザイン見直し</h3>
                <span class="bg-blue-100 text-blue-800 text-xs px-2 py-1 rounded-full font-medium">通常</span>
                <div class="w-3 h-3 rounded-full bg-green-400"></div>
              </div>
              <p class="text-slate-600 mb-3">メインページのUIを現代的なデザインに更新</p>
              <div class="flex items-center space-x-4 text-sm text-slate-500">
                <span>期限: 今日 17:00</span>
                <span>カテゴリ: デザイン</span>
                <span>作成日: 2024-06-17</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Completed Task -->
        <div class="bg-green-50 border-2 border-green-200 rounded-xl p-6 transition-all hover:shadow-lg opacity-75">
          <div class="flex items-start space-x-4">
            <div class="mt-1">
              <input type="checkbox" checked class="w-5 h-5 text-green-500 rounded border-2 border-green-300 focus:ring-green-500" />
            </div>
            <div class="flex-1">
              <div class="flex items-center space-x-3 mb-2">
                <h3 class="text-lg font-semibold text-slate-800 line-through">チーム会議準備</h3>
                <span class="bg-green-100 text-green-800 text-xs px-2 py-1 rounded-full font-medium">完了</span>
                <div class="w-3 h-3 rounded-full bg-blue-400"></div>
              </div>
              <p class="text-slate-600 mb-3 line-through">明日の会議のアジェンダと資料を準備</p>
              <div class="flex items-center space-x-4 text-sm text-slate-500">
                <span>完了時刻: 今日 14:30</span>
                <span>カテゴリ: 会議</span>
                <span>作成日: 2024-06-15</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Additional Tasks -->
        <div class="bg-white border-2 border-slate-200 rounded-xl p-6 transition-all hover:shadow-lg hover:border-slate-300">
          <div class="flex items-start space-x-4">
            <div class="mt-1">
              <input type="checkbox" class="w-5 h-5 text-blue-500 rounded border-2 border-slate-300 focus:ring-blue-500" />
            </div>
            <div class="flex-1">
              <div class="flex items-center space-x-3 mb-2">
                <h3 class="text-lg font-semibold text-slate-800">データベース最適化</h3>
                <span class="bg-blue-100 text-blue-800 text-xs px-2 py-1 rounded-full font-medium">通常</span>
                <div class="w-3 h-3 rounded-full bg-red-400"></div>
              </div>
              <p class="text-slate-600 mb-3">クエリのパフォーマンスを改善してレスポンス時間を短縮</p>
              <div class="flex items-center space-x-4 text-sm text-slate-500">
                <span>期限: 今日 19:00</span>
                <span>カテゴリ: 実装</span>
                <span>作成日: 2024-06-16</span>
              </div>
            </div>
          </div>
        </div>

        <div class="bg-white border-2 border-slate-200 rounded-xl p-6 transition-all hover:shadow-lg hover:border-slate-300">
          <div class="flex items-start space-x-4">
            <div class="mt-1">
              <input type="checkbox" class="w-5 h-5 text-blue-500 rounded border-2 border-slate-300 focus:ring-blue-500" />
            </div>
            <div class="flex-1">
              <div class="flex items-center space-x-3 mb-2">
                <h3 class="text-lg font-semibold text-slate-800">ドキュメント更新</h3>
                <span class="bg-yellow-100 text-yellow-800 text-xs px-2 py-1 rounded-full font-medium">低優先度</span>
                <div class="w-3 h-3 rounded-full bg-green-400"></div>
              </div>
              <p class="text-slate-600 mb-3">API仕様書とユーザーマニュアルの更新</p>
              <div class="flex items-center space-x-4 text-sm text-slate-500">
                <span>期限: 今日 16:00</span>
                <span>カテゴリ: デザイン</span>
                <span>作成日: 2024-06-14</span>
              </div>
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
      currentDate: "",
    };
  },
  mounted() {
    this.updateDateTime();
  },
  methods: {
    updateDateTime() {
      const now = new Date();
      const options: Intl.DateTimeFormatOptions = {
        year: "numeric",
        month: "long",
        day: "numeric",
      };
      this.currentDate = now.toLocaleDateString("ja-JP", options);
    },
  },
});
</script>