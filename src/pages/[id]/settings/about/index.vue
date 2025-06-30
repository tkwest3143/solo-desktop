<template>
  <section class="max-w-2xl mx-auto p-8">
    <div class="flex items-center mb-8">
      <div
        class="bg-gradient-to-br from-blue-400 to-blue-600 rounded-full w-16 h-16 flex items-center justify-center shadow-lg mr-5"
      >
        <Icon name="fluent:apps-list-20-filled" size="2em" class="text-white" />
      </div>
      <div>
        <h1 class="text-3xl font-extrabold text-slate-800 tracking-tight mb-1">
          このアプリについて
        </h1>
        <p class="text-blue-600 font-medium">
          タスク・勤務管理デスクトップアプリ
        </p>
      </div>
    </div>
    <div
      class="bg-white/80 rounded-2xl shadow-lg p-8 space-y-6 border border-slate-100"
    >
      <div class="flex items-center mb-2">
        <Icon
          name="fluent:star-20-filled"
          class="text-yellow-400 mr-2"
          size="1.2em"
        />
        <h2 class="text-xl font-bold text-slate-800">アプリの特徴</h2>
      </div>
      <ul class="list-disc pl-7 text-slate-700 space-y-1">
        <li>タスクの追加・編集・完了・削除など直感的な操作</li>
        <li>カテゴリや優先度によるタスクの整理</li>
        <li>Googleカレンダー連携やICSファイルエクスポート</li>
        <li>勤務時間の記録・レポート表示</li>
        <li>ユーザーごとの管理と柔軟な設定</li>
        <li>すべてのダイアログ・通知が統一デザインで快適</li>
      </ul>
      <div class="flex items-center mt-6 mb-2">
        <Icon
          name="fluent:people-community-20-filled"
          class="text-green-500 mr-2"
          size="1.2em"
        />
        <h2 class="text-xl font-bold text-slate-800">利用シーン例</h2>
      </div>
      <ul class="list-disc pl-7 text-slate-700 space-y-1">
        <li>日々のToDo管理やスケジュール調整</li>
        <li>リモートワークやフレックスタイム制の勤務記録</li>
        <li>チームでのタスク共有や進捗管理</li>
        <li>業務レポート作成や自己管理の振り返り</li>
      </ul>
      <div class="flex items-center mt-6 mb-2">
        <Icon
          name="fluent:lightbulb-20-filled"
          class="text-purple-500 mr-2"
          size="1.2em"
        />
        <h2 class="text-xl font-bold text-slate-800">開発背景</h2>
      </div>
      <p class="text-slate-700">
        「もっとシンプルに、でもフリーランスの時間を管理したい」という私の感情から生まれました。<br />
        デスクトップアプリとして動作し、オフラインでも快適に利用できます。
      </p>
      <div class="flex items-center mt-6 mb-2">
        <Icon
          name="fluent:chat-help-20-filled"
          class="text-blue-400 mr-2"
          size="1.2em"
        />
        <h2 class="text-xl font-bold text-slate-800">フィードバック</h2>
      </div>
      <p class="text-slate-700">
        ご意見・ご要望はぜひフィードバックください。今後も機能改善・拡張を続けていきます。
      </p>
    </div>
  </section>
</template>
<script setup lang="ts"></script>
