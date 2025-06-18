<template>
  <div class="h-full bg-white">
    <!-- Page Header -->
    <div class="p-8 border-b border-slate-200">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-3xl font-bold text-slate-800 mb-2">すべてのTodo</h1>
          <p class="text-slate-600">全タスクの一覧表示</p>
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

    <!-- Filters and Search -->
    <div class="p-8 border-b border-slate-200 bg-slate-50">
      <div class="flex flex-col lg:flex-row lg:items-center space-y-4 lg:space-y-0 lg:space-x-4">
        <!-- Search -->
        <div class="flex-1">
          <div class="relative">
            <Icon name="fluent:search-20-filled" class="absolute left-3 top-1/2 transform -translate-y-1/2 text-slate-400" size="1.2em" />
            <input
              type="text"
              placeholder="タスクを検索..."
              class="w-full pl-10 pr-4 py-3 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            />
          </div>
        </div>
        
        <!-- Filters -->
        <div class="flex space-x-3">
          <select class="border border-slate-300 rounded-lg px-3 py-3 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500">
            <option>すべて</option>
            <option>未完了</option>
            <option>完了済み</option>
          </select>
          <select class="border border-slate-300 rounded-lg px-3 py-3 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500">
            <option>すべてのカテゴリ</option>
            <option>実装</option>
            <option>デザイン</option>
            <option>会議</option>
          </select>
          <select class="border border-slate-300 rounded-lg px-3 py-3 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500">
            <option>期日順</option>
            <option>作成日順</option>
            <option>優先度順</option>
            <option>タイトル順</option>
          </select>
        </div>
      </div>
    </div>

    <!-- Task List -->
    <div class="p-8">
      <div class="mb-4 text-sm text-slate-500">
        12件のタスク（8件未完了、4件完了済み）
      </div>

      <!-- Tasks Grid -->
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <!-- Sample Tasks -->
        <div class="bg-white border-2 border-slate-200 rounded-xl p-6 transition-all hover:shadow-lg hover:border-slate-300">
          <div class="flex items-start space-x-4">
            <div class="mt-1">
              <input type="checkbox" class="w-5 h-5 text-blue-500 rounded border-2 border-slate-300 focus:ring-blue-500" />
            </div>
            <div class="flex-1">
              <div class="flex items-center space-x-3 mb-2">
                <h3 class="text-lg font-semibold text-slate-800">緊急バグ修正</h3>
                <span class="bg-red-100 text-red-800 text-xs px-2 py-1 rounded-full font-medium">高優先度</span>
                <div class="w-3 h-3 rounded-full bg-red-400"></div>
              </div>
              <p class="text-slate-600 mb-3">ユーザーログイン機能の重要なバグを修正する必要があります</p>
              <div class="flex items-center justify-between">
                <div class="flex items-center space-x-3 text-sm text-slate-500">
                  <span>期限: 今日 18:00</span>
                  <span>実装</span>
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
                <h3 class="text-lg font-semibold text-slate-800">UIデザイン見直し</h3>
                <span class="bg-blue-100 text-blue-800 text-xs px-2 py-1 rounded-full font-medium">通常</span>
                <div class="w-3 h-3 rounded-full bg-green-400"></div>
              </div>
              <p class="text-slate-600 mb-3">メインページのUIを現代的なデザインに更新</p>
              <div class="flex items-center justify-between">
                <div class="flex items-center space-x-3 text-sm text-slate-500">
                  <span>期限: 今日 17:00</span>
                  <span>デザイン</span>
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
            </div>
          </div>
        </div>

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
              <div class="flex items-center justify-between">
                <div class="flex items-center space-x-3 text-sm text-slate-500">
                  <span>完了: 今日 14:30</span>
                  <span>会議</span>
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
                <h3 class="text-lg font-semibold text-slate-800">データベース最適化</h3>
                <span class="bg-blue-100 text-blue-800 text-xs px-2 py-1 rounded-full font-medium">通常</span>
                <div class="w-3 h-3 rounded-full bg-red-400"></div>
              </div>
              <p class="text-slate-600 mb-3">クエリのパフォーマンスを改善してレスポンス時間を短縮</p>
              <div class="flex items-center justify-between">
                <div class="flex items-center space-x-3 text-sm text-slate-500">
                  <span>期限: 今日 19:00</span>
                  <span>実装</span>
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
              <div class="flex items-center justify-between">
                <div class="flex items-center space-x-3 text-sm text-slate-500">
                  <span>期限: 明日 16:00</span>
                  <span>デザイン</span>
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
                <h3 class="text-lg font-semibold text-slate-800">プレゼンテーション準備</h3>
                <span class="bg-orange-100 text-orange-800 text-xs px-2 py-1 rounded-full font-medium">高優先度</span>
                <div class="w-3 h-3 rounded-full bg-blue-400"></div>
              </div>
              <p class="text-slate-600 mb-3">来週のプロジェクト会議用のプレゼンテーション資料作成</p>
              <div class="flex items-center justify-between">
                <div class="flex items-center space-x-3 text-sm text-slate-500">
                  <span>期限: 明日 15:00</span>
                  <span>会議</span>
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
            </div>
          </div>
        </div>
      </div>

      <!-- Pagination -->
      <div class="mt-8 flex items-center justify-between">
        <div class="text-sm text-slate-500">
          1-6 / 12件を表示
        </div>
        <div class="flex space-x-2">
          <button class="px-3 py-2 border border-slate-300 rounded-lg text-slate-500 hover:bg-slate-50 transition-colors">
            前へ
          </button>
          <button class="px-3 py-2 bg-blue-500 text-white rounded-lg">1</button>
          <button class="px-3 py-2 border border-slate-300 rounded-lg text-slate-700 hover:bg-slate-50 transition-colors">2</button>
          <button class="px-3 py-2 border border-slate-300 rounded-lg text-slate-700 hover:bg-slate-50 transition-colors">
            次へ
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  layout: "todo",
});
</script>