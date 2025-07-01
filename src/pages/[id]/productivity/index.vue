<template>
  <div class="h-full bg-white">
    <!-- Page Header -->
    <div class="p-8 border-b border-slate-200">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-3xl font-bold text-slate-800 mb-2">集中力向上ツール</h1>
          <p class="text-slate-600">ポモドーロ・タイムブロッキング・ディープワークで生産性を最大化</p>
        </div>
        <div class="text-right">
          <div class="text-sm text-slate-500">{{ currentDate }}</div>
          <div class="text-lg font-semibold text-slate-700">{{ currentDay }}</div>
        </div>
      </div>
    </div>

    <!-- Main Content -->
    <div class="p-8">
      <!-- Feature Cards -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-8">
        <!-- Pomodoro Timer Card -->
        <div class="bg-gradient-to-br from-red-500 to-red-600 rounded-xl p-6 text-white shadow-lg">
          <div class="flex items-center justify-between mb-4">
            <div>
              <p class="text-red-100 text-sm">ポモドーロタイマー</p>
              <p class="text-2xl font-bold">25分 集中</p>
              <p class="text-red-200 text-sm">短期集中で効率アップ</p>
            </div>
            <Icon name="fluent:timer-20-filled" size="3em" class="text-red-200" />
          </div>
          <NuxtLink
            :to="{ name: 'id-productivity-pomodoro', params: { id: $route.params.id } }"
            class="inline-flex items-center px-4 py-2 bg-white bg-opacity-20 hover:bg-opacity-30 rounded-lg text-white font-medium transition-all duration-200"
          >
            タイマー開始
            <Icon name="fluent:arrow-right-20-filled" class="ml-2" size="1em" />
          </NuxtLink>
        </div>

        <!-- Time Blocking Card -->
        <div class="bg-gradient-to-br from-blue-500 to-blue-600 rounded-xl p-6 text-white shadow-lg">
          <div class="flex items-center justify-between mb-4">
            <div>
              <p class="text-blue-100 text-sm">タイムブロッキング</p>
              <p class="text-2xl font-bold">時間割当</p>
              <p class="text-blue-200 text-sm">計画的なスケジュール管理</p>
            </div>
            <Icon name="fluent:calendar-agenda-20-filled" size="3em" class="text-blue-200" />
          </div>
          <NuxtLink
            :to="{ name: 'id-productivity-timeblocking', params: { id: $route.params.id } }"
            class="inline-flex items-center px-4 py-2 bg-white bg-opacity-20 hover:bg-opacity-30 rounded-lg text-white font-medium transition-all duration-200"
          >
            スケジュール設定
            <Icon name="fluent:arrow-right-20-filled" class="ml-2" size="1em" />
          </NuxtLink>
        </div>

        <!-- Deep Work Card -->
        <div class="bg-gradient-to-br from-purple-500 to-purple-600 rounded-xl p-6 text-white shadow-lg">
          <div class="flex items-center justify-between mb-4">
            <div>
              <p class="text-purple-100 text-sm">ディープワーク</p>
              <p class="text-2xl font-bold">深い集中</p>
              <p class="text-purple-200 text-sm">質の高い作業時間を確保</p>
            </div>
            <Icon name="fluent:brain-circuit-20-filled" size="3em" class="text-purple-200" />
          </div>
          <NuxtLink
            :to="{ name: 'id-productivity-deepwork', params: { id: $route.params.id } }"
            class="inline-flex items-center px-4 py-2 bg-white bg-opacity-20 hover:bg-opacity-30 rounded-lg text-white font-medium transition-all duration-200"
          >
            モード設定
            <Icon name="fluent:arrow-right-20-filled" class="ml-2" size="1em" />
          </NuxtLink>
        </div>
      </div>

      <!-- Today's Focus Sessions -->
      <div class="bg-white rounded-xl shadow-lg border border-slate-200 p-6 mb-6">
        <div class="flex items-center justify-between mb-4">
          <h2 class="text-xl font-semibold text-slate-800">本日の集中セッション</h2>
          <div class="flex items-center space-x-2 text-sm text-slate-600">
            <Icon name="fluent:calendar-today-20-filled" class="text-blue-500" />
            <span>{{ todaySessionsCount }}セッション完了</span>
          </div>
        </div>
        
        <div v-if="todaySessions.length === 0" class="text-center py-8">
          <Icon name="fluent:brain-circuit-20-regular" size="3em" class="text-slate-300 mb-3" />
          <p class="text-slate-500">まだ今日の集中セッションはありません</p>
          <p class="text-sm text-slate-400">上のツールを使って集中時間を始めましょう</p>
        </div>

        <div v-else class="space-y-3">
          <div
            v-for="session in todaySessions"
            :key="session.prop.id"
            class="flex items-center justify-between p-4 bg-slate-50 rounded-lg"
          >
            <div class="flex items-center space-x-3">
              <div 
                class="w-3 h-3 rounded-full"
                :class="{
                  'bg-red-500': session.prop.type === 'pomodoro',
                  'bg-blue-500': session.prop.type === 'timeblock',
                  'bg-purple-500': session.prop.type === 'deepwork'
                }"
              ></div>
              <div>
                <p class="font-medium text-slate-800">{{ session.prop.title }}</p>
                <p class="text-sm text-slate-600">{{ session.durationText }} · {{ session.startTimeText }}</p>
              </div>
            </div>
            <div class="text-right">
              <span
                class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium"
                :class="{
                  'bg-green-100 text-green-800': session.prop.completed,
                  'bg-yellow-100 text-yellow-800': !session.prop.completed
                }"
              >
                {{ getStatusLabel(session.prop.completed) }}
              </span>
            </div>
          </div>
        </div>
      </div>

      <!-- Productivity Tips -->
      <div class="bg-gradient-to-br from-slate-50 to-blue-50 rounded-xl p-6">
        <h3 class="text-lg font-semibold text-slate-800 mb-4 flex items-center">
          <Icon name="fluent:lightbulb-20-filled" class="mr-2 text-yellow-500" />
          生産性向上のヒント
        </h3>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          <div class="bg-white rounded-lg p-4 shadow-sm">
            <h4 class="font-medium text-slate-800 mb-2">ポモドーロテクニック</h4>
            <p class="text-sm text-slate-600">25分の集中と5分の休憩を繰り返すことで、長時間の集中力を維持できます。</p>
          </div>
          <div class="bg-white rounded-lg p-4 shadow-sm">
            <h4 class="font-medium text-slate-800 mb-2">タイムブロッキング</h4>
            <p class="text-sm text-slate-600">特定の時間をタスクに割り当てることで、計画的に作業を進められます。</p>
          </div>
          <div class="bg-white rounded-lg p-4 shadow-sm">
            <h4 class="font-medium text-slate-800 mb-2">ディープワーク</h4>
            <p class="text-sm text-slate-600">気を散らすものを排除して、質の高い集中状態で価値の高い仕事に取り組みます。</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'
import { ProductivitySessionData, type ProductivitySession } from '~/models/productivitySession'

export default defineComponent({
  name: 'ProductivityIndex',
  data() {
    return {
      currentDate: new Date().toLocaleDateString('ja-JP'),
      currentDay: new Date().toLocaleDateString('ja-JP', { weekday: 'long' }),
      todaySessions: [] as ProductivitySessionData[]
    }
  },
  mounted() {
    this.loadTodaySessions()
  },
  computed: {
    todaySessionsCount(): number {
      return this.todaySessions.filter(session => session.prop.completed).length
    }
  },
  methods: {
    getStatusLabel(completed: boolean): string {
      return completed ? '完了' : '中断'
    },
    loadTodaySessions() {
      // Load sessions for today from localStorage or API
      const today = new Date().toISOString().split('T')[0]
      const allSessions = this.getAllSessions()
      
      this.todaySessions = allSessions
        .filter(session => session.prop.targetDay === today)
        .sort((a, b) => b.prop.startTime.getTime() - a.prop.startTime.getTime())
    },
    getAllSessions(): ProductivitySessionData[] {
      const sessions: ProductivitySessionData[] = []
      
      // Load from localStorage (in a real app, this would be from a database)
      const pomodoro = localStorage.getItem('pomodoroSessions')
      const deepWork = localStorage.getItem('deepWorkSessions') 
      const timeBlocks = localStorage.getItem('timeBlocks')
      
      if (pomodoro) {
        const pomodoroSessions = JSON.parse(pomodoro)
        sessions.push(...pomodoroSessions.map((s: any) => 
          new ProductivitySessionData({
            ...s,
            type: 'pomodoro' as const,
            startTime: new Date(s.startTime),
            endTime: s.endTime ? new Date(s.endTime) : undefined
          })
        ))
      }
      
      if (deepWork) {
        const deepWorkSessions = JSON.parse(deepWork)
        sessions.push(...deepWorkSessions.map((s: any) => 
          new ProductivitySessionData({
            ...s,
            type: 'deepwork' as const,
            startTime: new Date(s.startTime),
            endTime: s.endTime ? new Date(s.endTime) : undefined
          })
        ))
      }
      
      return sessions
    }
  }
})
</script>