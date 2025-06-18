<template>
  <div class="h-full bg-white">
    <!-- Page Header -->
    <div class="p-8 border-b border-slate-200">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-3xl font-bold text-slate-800 mb-2">期日が近いTodo</h1>
          <p class="text-slate-600">優先的に対応が必要なタスク</p>
        </div>
        <NuxtLink
          :to="{ name: 'id-todo-add', params: { id: $route.params.id } }"
          class="bg-orange-500 hover:bg-orange-600 text-white px-6 py-3 rounded-lg font-medium transition-colors shadow-lg"
        >
          <Icon name="fluent:add-20-filled" class="mr-2" />
          新規作成
        </NuxtLink>
      </div>
    </div>

    <!-- Task List -->
    <div class="p-8">
      <!-- Filter Options -->
      <div class="flex items-center justify-between mb-6">
        <div class="flex items-center space-x-4">
          <select class="border border-slate-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-orange-500">
            <option>すべて</option>
            <option>未完了</option>
            <option>完了済み</option>
          </select>
          <select class="border border-slate-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-orange-500">
            <option>すべてのカテゴリ</option>
            <option>実装</option>
            <option>デザイン</option>
            <option>会議</option>
          </select>
        </div>
        <div class="text-sm text-slate-500">
          3件の期日が近いタスク
        </div>
      </div>

      <!-- Urgent Tasks (Today) -->
      <div class="mb-8">
        <h2 class="text-xl font-semibold text-red-600 mb-4 flex items-center">
          <Icon name="fluent:warning-20-filled" class="mr-2" />
          今日期限（緊急）
        </h2>
        <div class="space-y-4">
          <div class="bg-red-50 border-2 border-red-300 rounded-xl p-6 transition-all hover:shadow-lg">
            <div class="flex items-start space-x-4">
              <div class="mt-1">
                <input type="checkbox" class="w-5 h-5 text-red-500 rounded border-2 border-red-300 focus:ring-red-500" />
              </div>
              <div class="flex-1">
                <div class="flex items-center space-x-3 mb-2">
                  <h3 class="text-lg font-semibold text-slate-800">緊急バグ修正</h3>
                  <span class="bg-red-200 text-red-800 text-xs px-2 py-1 rounded-full font-medium">今日期限</span>
                  <div class="w-3 h-3 rounded-full bg-red-400"></div>
                </div>
                <p class="text-slate-600 mb-3">ユーザーログイン機能の重要なバグを修正する必要があります</p>
                <div class="flex items-center space-x-4 text-sm text-slate-500">
                  <span class="text-red-600 font-medium">期限: 今日 18:00（残り3時間）</span>
                  <span>カテゴリ: 実装</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Tomorrow's Tasks -->
      <div class="mb-8">
        <h2 class="text-xl font-semibold text-orange-600 mb-4 flex items-center">
          <Icon name="fluent:calendar-20-filled" class="mr-2" />
          明日期限
        </h2>
        <div class="space-y-4">
          <div class="bg-orange-50 border-2 border-orange-200 rounded-xl p-6 transition-all hover:shadow-lg">
            <div class="flex items-start space-x-4">
              <div class="mt-1">
                <input type="checkbox" class="w-5 h-5 text-orange-500 rounded border-2 border-orange-300 focus:ring-orange-500" />
              </div>
              <div class="flex-1">
                <div class="flex items-center space-x-3 mb-2">
                  <h3 class="text-lg font-semibold text-slate-800">プレゼンテーション準備</h3>
                  <span class="bg-orange-200 text-orange-800 text-xs px-2 py-1 rounded-full font-medium">明日期限</span>
                  <div class="w-3 h-3 rounded-full bg-blue-400"></div>
                </div>
                <p class="text-slate-600 mb-3">来週のプロジェクト会議用のプレゼンテーション資料作成</p>
                <div class="flex items-center space-x-4 text-sm text-slate-500">
                  <span class="text-orange-600 font-medium">期限: 明日 15:00</span>
                  <span>カテゴリ: 会議</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- This Week's Tasks -->
      <div class="mb-8">
        <h2 class="text-xl font-semibold text-yellow-600 mb-4 flex items-center">
          <Icon name="fluent:calendar-week-start-20-filled" class="mr-2" />
          今週期限
        </h2>
        <div class="space-y-4">
          <div class="bg-yellow-50 border-2 border-yellow-200 rounded-xl p-6 transition-all hover:shadow-lg">
            <div class="flex items-start space-x-4">
              <div class="mt-1">
                <input type="checkbox" class="w-5 h-5 text-yellow-500 rounded border-2 border-yellow-300 focus:ring-yellow-500" />
              </div>
              <div class="flex-1">
                <div class="flex items-center space-x-3 mb-2">
                  <h3 class="text-lg font-semibold text-slate-800">新機能テスト</h3>
                  <span class="bg-yellow-200 text-yellow-800 text-xs px-2 py-1 rounded-full font-medium">今週期限</span>
                  <div class="w-3 h-3 rounded-full bg-red-400"></div>
                </div>
                <p class="text-slate-600 mb-3">新しく実装した機能の統合テストとバグ修正</p>
                <div class="flex items-center space-x-4 text-sm text-slate-500">
                  <span class="text-yellow-600 font-medium">期限: 6月21日（金）</span>
                  <span>カテゴリ: 実装</span>
                </div>
              </div>
            </div>
          </div>

          <div class="bg-yellow-50 border-2 border-yellow-200 rounded-xl p-6 transition-all hover:shadow-lg">
            <div class="flex items-start space-x-4">
              <div class="mt-1">
                <input type="checkbox" class="w-5 h-5 text-yellow-500 rounded border-2 border-yellow-300 focus:ring-yellow-500" />
              </div>
              <div class="flex-1">
                <div class="flex items-center space-x-3 mb-2">
                  <h3 class="text-lg font-semibold text-slate-800">UIデザインレビュー</h3>
                  <span class="bg-yellow-200 text-yellow-800 text-xs px-2 py-1 rounded-full font-medium">今週期限</span>
                  <div class="w-3 h-3 rounded-full bg-green-400"></div>
                </div>
                <p class="text-slate-600 mb-3">デザインチームとの最終レビューミーティング</p>
                <div class="flex items-center space-x-4 text-sm text-slate-500">
                  <span class="text-yellow-600 font-medium">期限: 6月22日（土）</span>
                  <span>カテゴリ: デザイン</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Tips Section -->
      <div class="bg-blue-50 border border-blue-200 rounded-xl p-6">
        <div class="flex items-start space-x-3">
          <Icon name="fluent:lightbulb-20-filled" class="text-blue-500 mt-1" size="1.5em" />
          <div>
            <h3 class="font-semibold text-slate-800 mb-2">効果的なタスク管理のヒント</h3>
            <ul class="text-sm text-slate-600 space-y-1">
              <li>• 緊急度と重要度を区別して優先順位を付ける</li>
              <li>• 大きなタスクは小さなタスクに分割する</li>
              <li>• 期限の前日にリマインダーを設定する</li>
              <li>• 定期的にタスクの進捗状況を確認する</li>
            </ul>
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
});
</script>