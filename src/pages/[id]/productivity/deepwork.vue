<template>
  <div class="h-full bg-white">
    <!-- Page Header -->
    <div class="p-8 border-b border-slate-200">
      <div class="flex items-center justify-between">
        <div class="flex items-center space-x-4">
          <NuxtLink
            :to="{ name: 'id-productivity', params: { id: $route.params.id } }"
            class="p-2 hover:bg-slate-100 rounded-lg transition-colors"
          >
            <Icon name="fluent:arrow-left-20-filled" size="1.5em" class="text-slate-600" />
          </NuxtLink>
          <div>
            <h1 class="text-3xl font-bold text-slate-800 mb-2">ディープワーク</h1>
            <p class="text-slate-600">深い集中状態で価値の高い仕事に取り組みましょう</p>
          </div>
        </div>
        <div class="text-right">
          <div class="text-sm text-slate-500">現在のモード</div>
          <div class="text-lg font-semibold text-slate-700">{{ currentMode.name }}</div>
        </div>
      </div>
    </div>

    <!-- Main Content -->
    <div class="p-8">
      <div class="max-w-6xl mx-auto">
        <!-- Deep Work Mode Selection -->
        <div class="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-4 gap-6 mb-8">
          <div
            v-for="mode in deepWorkModes"
            :key="mode.id"
            @click="selectMode(mode)"
            class="bg-white rounded-xl shadow-lg border-2 cursor-pointer transition-all duration-200 hover:shadow-xl hover:-translate-y-1"
            :class="{
              'border-purple-500 bg-purple-50': currentMode.id === mode.id,
              'border-slate-200': currentMode.id !== mode.id
            }"
          >
            <div class="p-6">
              <div class="flex items-center justify-between mb-4">
                <Icon 
                  :name="mode.icon" 
                  size="3em" 
                  :class="{
                    'text-purple-600': currentMode.id === mode.id,
                    'text-slate-400': currentMode.id !== mode.id
                  }"
                />
                <div 
                  v-if="currentMode.id === mode.id"
                  class="w-6 h-6 bg-purple-500 text-white rounded-full flex items-center justify-center"
                >
                  <Icon name="fluent:checkmark-20-filled" size="1em" />
                </div>
              </div>
              <h3 class="text-lg font-semibold text-slate-800 mb-2">{{ mode.name }}</h3>
              <p class="text-sm text-slate-600 mb-3">{{ mode.description }}</p>
              <div class="text-xs text-slate-500">
                <strong>適用場面:</strong> {{ mode.usageCase }}
              </div>
            </div>
          </div>
        </div>

        <!-- Current Session Panel -->
        <div class="bg-white rounded-xl shadow-lg border border-slate-200 p-6 mb-6">
          <div class="flex items-center justify-between mb-6">
            <h2 class="text-xl font-semibold text-slate-800">ディープワークセッション</h2>
            <div class="flex items-center space-x-2">
              <span 
                class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium"
                :class="{
                  'bg-green-100 text-green-800': sessionStatus === 'active',
                  'bg-yellow-100 text-yellow-800': sessionStatus === 'paused',
                  'bg-gray-100 text-gray-800': sessionStatus === 'idle'
                }"
              >
                {{ getSessionStatusLabel(sessionStatus) }}
              </span>
            </div>
          </div>

          <div v-if="sessionStatus === 'idle'" class="text-center py-8">
            <Icon name="fluent:brain-circuit-20-regular" size="4em" class="text-slate-300 mb-4" />
            <h3 class="text-lg font-medium text-slate-800 mb-2">ディープワークセッションを開始</h3>
            <p class="text-slate-600 mb-6">{{ currentMode.name }}モードで集中して作業しましょう</p>
            
            <div class="max-w-md mx-auto space-y-4 mb-6">
              <div>
                <label class="block text-sm font-medium text-slate-700 mb-2">セッション時間 (分)</label>
                <input
                  v-model.number="sessionSettings.duration"
                  type="number"
                  min="15"
                  max="240"
                  class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-purple-500"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-slate-700 mb-2">取り組むタスク</label>
                <input
                  v-model="sessionSettings.task"
                  type="text"
                  placeholder="このセッションで取り組むタスクを入力..."
                  class="w-full px-3 py-2 border border-slate-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-purple-500"
                />
              </div>
            </div>

            <button
              @click="startSession"
              class="px-8 py-3 bg-purple-500 hover:bg-purple-600 text-white rounded-lg font-semibold shadow-lg hover:shadow-xl transform hover:-translate-y-1 transition-all duration-200"
            >
              <Icon name="fluent:play-20-filled" class="mr-2" />
              セッション開始
            </button>
          </div>

          <div v-else class="grid grid-cols-1 lg:grid-cols-2 gap-6">
            <!-- Timer Display -->
            <div class="text-center">
              <div class="inline-block relative mb-6">
                <svg class="w-48 h-48 transform -rotate-90" viewBox="0 0 192 192">
                  <circle
                    cx="96"
                    cy="96"
                    r="80"
                    stroke="currentColor"
                    stroke-width="12"
                    fill="transparent"
                    class="text-slate-200"
                  />
                  <circle
                    cx="96"
                    cy="96"
                    r="80"
                    stroke="currentColor"
                    stroke-width="12"
                    fill="transparent"
                    :stroke-dasharray="circumference"
                    :stroke-dashoffset="dashOffset"
                    class="text-purple-500 transition-all duration-1000"
                  />
                </svg>
                <div class="absolute inset-0 flex items-center justify-center">
                  <div class="text-center">
                    <div class="text-3xl font-bold text-slate-800">
                      {{ formatTime(timeRemaining) }}
                    </div>
                    <div class="text-sm text-slate-600">残り時間</div>
                  </div>
                </div>
              </div>
              
              <div class="space-y-2">
                <button
                  @click="toggleSession"
                  :class="{
                    'bg-red-500 hover:bg-red-600': sessionStatus === 'active',
                    'bg-green-500 hover:bg-green-600': sessionStatus === 'paused'
                  }"
                  class="px-6 py-2 text-white rounded-lg font-medium transition-colors"
                >
                  <Icon 
                    :name="sessionStatus === 'active' ? 'fluent:pause-20-filled' : 'fluent:play-20-filled'" 
                    class="mr-2" 
                  />
                  {{ sessionStatus === 'active' ? '一時停止' : '再開' }}
                </button>
                <button
                  @click="endSession"
                  class="ml-2 px-6 py-2 bg-slate-500 hover:bg-slate-600 text-white rounded-lg font-medium transition-colors"
                >
                  <Icon name="fluent:stop-20-filled" class="mr-2" />
                  終了
                </button>
              </div>
            </div>

            <!-- Session Info -->
            <div class="space-y-4">
              <div class="bg-purple-50 rounded-lg p-4">
                <h4 class="font-medium text-purple-800 mb-2">現在のタスク</h4>
                <p class="text-purple-700">{{ currentSession.task || '設定されていません' }}</p>
              </div>
              
              <div class="bg-blue-50 rounded-lg p-4">
                <h4 class="font-medium text-blue-800 mb-2">進捗</h4>
                <div class="space-y-2">
                  <div class="flex justify-between text-sm">
                    <span class="text-blue-700">経過時間</span>
                    <span class="text-blue-700 font-medium">{{ formatTime(currentSession.duration * 60 - timeRemaining) }}</span>
                  </div>
                  <div class="flex justify-between text-sm">
                    <span class="text-blue-700">総時間</span>
                    <span class="text-blue-700 font-medium">{{ currentSession.duration }}分</span>
                  </div>
                  <div class="w-full bg-blue-200 rounded-full h-2">
                    <div 
                      class="bg-blue-500 h-2 rounded-full transition-all duration-1000"
                      :style="{ width: `${progressPercentage}%` }"
                    ></div>
                  </div>
                </div>
              </div>

              <div class="bg-green-50 rounded-lg p-4">
                <h4 class="font-medium text-green-800 mb-2">フォーカス支援</h4>
                <div class="space-y-2 text-sm text-green-700">
                  <div class="flex items-center space-x-2">
                    <Icon name="fluent:phone-dismiss-20-filled" class="text-green-600" />
                    <span>通知をオフにする</span>
                  </div>
                  <div class="flex items-center space-x-2">
                    <Icon name="fluent:eye-off-20-filled" class="text-green-600" />
                    <span>SNSやメールを避ける</span>
                  </div>
                  <div class="flex items-center space-x-2">
                    <Icon name="fluent:library-20-filled" class="text-green-600" />
                    <span>静かな環境を確保</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Statistics and History -->
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
          <!-- Today's Statistics -->
          <div class="bg-white rounded-xl shadow-lg border border-slate-200 p-6">
            <h3 class="text-lg font-semibold text-slate-800 mb-4 flex items-center">
              <Icon name="fluent:data-bar-horizontal-20-filled" class="mr-2 text-blue-500" />
              本日の統計
            </h3>
            <div class="space-y-4">
              <div class="flex justify-between items-center p-3 bg-purple-50 rounded-lg">
                <span class="text-sm font-medium text-slate-700">完了セッション</span>
                <span class="text-lg font-bold text-purple-600">{{ todayStats.completedSessions }}</span>
              </div>
              <div class="flex justify-between items-center p-3 bg-blue-50 rounded-lg">
                <span class="text-sm font-medium text-slate-700">総集中時間</span>
                <span class="text-lg font-bold text-blue-600">{{ todayStats.totalFocusTime }}分</span>
              </div>
              <div class="flex justify-between items-center p-3 bg-green-50 rounded-lg">
                <span class="text-sm font-medium text-slate-700">平均セッション時間</span>
                <span class="text-lg font-bold text-green-600">{{ todayStats.averageSessionTime }}分</span>
              </div>
            </div>
          </div>

          <!-- Recent Sessions -->
          <div class="bg-white rounded-xl shadow-lg border border-slate-200 p-6">
            <h3 class="text-lg font-semibold text-slate-800 mb-4 flex items-center">
              <Icon name="fluent:history-20-filled" class="mr-2 text-green-500" />
              最近のセッション
            </h3>
            <div v-if="recentSessions.length === 0" class="text-center py-4">
              <Icon name="fluent:brain-circuit-20-regular" size="2em" class="text-slate-300 mb-2" />
              <p class="text-slate-500 text-sm">まだセッション履歴がありません</p>
            </div>
            <div v-else class="space-y-3 max-h-64 overflow-y-auto">
              <div
                v-for="session in recentSessions"
                :key="session.id"
                class="flex items-center justify-between p-3 bg-slate-50 rounded-lg"
              >
                <div>
                  <p class="font-medium text-slate-800 text-sm">{{ session.task }}</p>
                  <p class="text-xs text-slate-600">
                    {{ session.mode }} · {{ session.duration }}分 · {{ formatDate(session.startTime) }}
                  </p>
                </div>
                <span 
                  class="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium"
                  :class="{
                    'bg-green-100 text-green-800': session.completed,
                    'bg-yellow-100 text-yellow-800': !session.completed
                  }"
                >
                  {{ session.completed ? '完了' : '中断' }}
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue'

interface DeepWorkMode {
  id: string
  name: string
  description: string
  usageCase: string
  icon: string
}

interface DeepWorkSession {
  id: string
  task: string
  duration: number
  mode: string
  startTime: Date
  endTime?: Date
  completed: boolean
}

interface SessionSettings {
  duration: number
  task: string
}

type SessionStatus = 'idle' | 'active' | 'paused'

export default defineComponent({
  name: 'DeepWork',
  data() {
    return {
      currentMode: {
        id: 'regular',
        name: '定期的ディープワーク',
        description: '毎日決まった時間にディープワークの時間を作る',
        usageCase: '継続的な習慣作り',
        icon: 'fluent:calendar-clock-20-filled'
      } as DeepWorkMode,
      deepWorkModes: [
        {
          id: 'regular',
          name: '定期的ディープワーク',
          description: '毎日決まった時間にディープワークの時間を作る',
          usageCase: '継続的な習慣作り',
          icon: 'fluent:calendar-clock-20-filled'
        },
        {
          id: 'intermittent',
          name: '断続的ディープワーク',
          description: '短い時間でも良いので、集中的な作業を行う機会を作る',
          usageCase: '忙しいスケジュールの中での集中',
          icon: 'fluent:timer-20-filled'
        },
        {
          id: 'journalist',
          name: 'ジャーナリスト的ディープワーク',
          description: '隙間時間を見つけて集中する',
          usageCase: '不規則なスケジュール',
          icon: 'fluent:news-20-filled'
        },
        {
          id: 'monastic',
          name: '修道院的ディープワーク',
          description: '長期間、完全に外界から遮断して集中する',
          usageCase: '大きなプロジェクトの完遂',
          icon: 'fluent:building-lighthouse-20-filled'
        }
      ] as DeepWorkMode[],
      sessionStatus: 'idle' as SessionStatus,
      sessionSettings: {
        duration: 90,
        task: ''
      } as SessionSettings,
      currentSession: {
        duration: 0,
        task: '',
        startTime: null as Date | null
      },
      timeRemaining: 0,
      timer: null as ReturnType<typeof setInterval> | null,
      recentSessions: [] as DeepWorkSession[]
    }
  },
  computed: {
    circumference(): number {
      return 2 * Math.PI * 80
    },
    dashOffset(): number {
      if (this.currentSession.duration === 0) return this.circumference
      const totalTime = this.currentSession.duration * 60
      const progress = (totalTime - this.timeRemaining) / totalTime
      return this.circumference * (1 - progress)
    },
    progressPercentage(): number {
      if (this.currentSession.duration === 0) return 0
      const totalTime = this.currentSession.duration * 60
      return ((totalTime - this.timeRemaining) / totalTime) * 100
    },
    todayStats() {
      const today = new Date().toDateString()
      const todaySessions = this.recentSessions.filter(
        session => session.startTime.toDateString() === today && session.completed
      )
      
      const totalTime = todaySessions.reduce((sum, session) => sum + session.duration, 0)
      const averageTime = todaySessions.length > 0 ? Math.round(totalTime / todaySessions.length) : 0
      
      return {
        completedSessions: todaySessions.length,
        totalFocusTime: totalTime,
        averageSessionTime: averageTime
      }
    }
  },
  mounted() {
    this.loadSessions()
  },
  beforeUnmount() {
    this.clearTimer()
  },
  methods: {
    selectMode(mode: DeepWorkMode) {
      this.currentMode = mode
      this.saveSettings()
    },
    startSession() {
      this.currentSession = {
        duration: this.sessionSettings.duration,
        task: this.sessionSettings.task,
        startTime: new Date()
      }
      this.timeRemaining = this.sessionSettings.duration * 60
      this.sessionStatus = 'active'
      this.startTimer()
    },
    toggleSession() {
      if (this.sessionStatus === 'active') {
        this.sessionStatus = 'paused'
        this.clearTimer()
      } else {
        this.sessionStatus = 'active'
        this.startTimer()
      }
    },
    endSession() {
      const completed = this.timeRemaining === 0
      
      // Save session to history
      const session: DeepWorkSession = {
        id: Date.now().toString(),
        task: this.currentSession.task,
        duration: this.currentSession.duration,
        mode: this.currentMode.name,
        startTime: this.currentSession.startTime!,
        endTime: new Date(),
        completed
      }
      
      this.recentSessions.unshift(session)
      if (this.recentSessions.length > 10) {
        this.recentSessions = this.recentSessions.slice(0, 10)
      }
      
      this.saveSessions()
      this.resetSession()
    },
    resetSession() {
      this.sessionStatus = 'idle'
      this.currentSession = {
        duration: 0,
        task: '',
        startTime: null
      }
      this.timeRemaining = 0
      this.clearTimer()
    },
    startTimer() {
      this.timer = setInterval(() => {
        if (this.timeRemaining > 0) {
          this.timeRemaining--
        } else {
          this.completeSession()
        }
      }, 1000)
    },
    clearTimer() {
      if (this.timer) {
        clearInterval(this.timer)
        this.timer = null
      }
    },
    completeSession() {
      this.clearTimer()
      this.endSession()
      this.showCompletionNotification()
    },
    formatTime(seconds: number): string {
      const minutes = Math.floor(seconds / 60)
      const remainingSeconds = seconds % 60
      return `${minutes.toString().padStart(2, '0')}:${remainingSeconds.toString().padStart(2, '0')}`
    },
    formatDate(date: Date): string {
      return date.toLocaleDateString('ja-JP', {
        month: 'short',
        day: 'numeric',
        hour: '2-digit',
        minute: '2-digit'
      })
    },
    getSessionStatusLabel(status: SessionStatus): string {
      switch (status) {
        case 'active':
          return '実行中'
        case 'paused':
          return '一時停止中'
        case 'idle':
          return '待機中'
        default:
          return status
      }
    },
    showCompletionNotification() {
      if ('Notification' in window && Notification.permission === 'granted') {
        new Notification('ディープワーク完了！', {
          body: `${this.currentSession.duration}分のセッションが完了しました。お疲れ様でした！`,
          icon: '/favicon.ico'
        })
      }
    },
    loadSessions() {
      const saved = localStorage.getItem('deepWorkSessions')
      if (saved) {
        this.recentSessions = JSON.parse(saved).map((session: any) => ({
          ...session,
          startTime: new Date(session.startTime),
          endTime: session.endTime ? new Date(session.endTime) : undefined
        }))
      }
      
      const savedSettings = localStorage.getItem('deepWorkSettings')
      if (savedSettings) {
        const settings = JSON.parse(savedSettings)
        if (settings.currentMode) {
          this.currentMode = settings.currentMode
        }
      }
    },
    saveSessions() {
      localStorage.setItem('deepWorkSessions', JSON.stringify(this.recentSessions))
    },
    saveSettings() {
      localStorage.setItem('deepWorkSettings', JSON.stringify({
        currentMode: this.currentMode
      }))
    }
  }
})
</script>