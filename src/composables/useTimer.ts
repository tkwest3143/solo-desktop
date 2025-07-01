import { ref, computed } from 'vue'

export type TimerMode = 'pomodoro' | 'countup'
export type PomodoroPhase = 'focus' | 'shortBreak' | 'longBreak'

interface GlobalTimerState {
  isRunning: boolean
  mode: TimerMode
  timeElapsed: number // For count-up timer
  timeLeft: number // For pomodoro timer
  pomodoroPhase: PomodoroPhase
  currentSessionCount: number
  completedCycles: number
  completedFocusSessions: number
  completedBreaks: number
  currentTask: string
  pomodoroSettings: {
    focusTime: number
    shortBreakTime: number
    longBreakTime: number
    longBreakInterval: number
  }
}

const globalTimerState = ref<GlobalTimerState>({
  isRunning: false,
  mode: 'pomodoro',
  timeElapsed: 0,
  timeLeft: 25 * 60, // 25 minutes default
  pomodoroPhase: 'focus',
  currentSessionCount: 0,
  completedCycles: 0,
  completedFocusSessions: 0,
  completedBreaks: 0,
  currentTask: '',
  pomodoroSettings: {
    focusTime: 25,
    shortBreakTime: 5,
    longBreakTime: 15,
    longBreakInterval: 4
  }
})

let timer: ReturnType<typeof setInterval> | null = null

export const useTimer = () => {
  const loadState = () => {
    const saved = localStorage.getItem('globalTimerState')
    if (saved) {
      const parsedState = JSON.parse(saved)
      // Restore state but don't auto-start the timer
      globalTimerState.value = { ...globalTimerState.value, ...parsedState, isRunning: false }
    }
  }

  const saveState = () => {
    localStorage.setItem('globalTimerState', JSON.stringify(globalTimerState.value))
  }

  const startTimer = () => {
    if (timer) return // Prevent multiple timers
    
    globalTimerState.value.isRunning = true
    timer = setInterval(() => {
      if (globalTimerState.value.mode === 'countup') {
        globalTimerState.value.timeElapsed++
      } else {
        // Pomodoro mode
        if (globalTimerState.value.timeLeft > 0) {
          globalTimerState.value.timeLeft--
        } else {
          completePomodoroSession()
        }
      }
      saveState()
    }, 1000)
    saveState()
  }

  const pauseTimer = () => {
    globalTimerState.value.isRunning = false
    if (timer) {
      clearInterval(timer)
      timer = null
    }
    saveState()
  }

  const resetTimer = () => {
    pauseTimer()
    if (globalTimerState.value.mode === 'countup') {
      globalTimerState.value.timeElapsed = 0
    } else {
      globalTimerState.value.timeLeft = getCurrentPomodoroTime() * 60
    }
    saveState()
  }

  const switchMode = (mode: TimerMode) => {
    pauseTimer()
    globalTimerState.value.mode = mode
    if (mode === 'countup') {
      globalTimerState.value.timeElapsed = 0
    } else {
      globalTimerState.value.timeLeft = getCurrentPomodoroTime() * 60
    }
    saveState()
  }

  const getCurrentPomodoroTime = (): number => {
    const settings = globalTimerState.value.pomodoroSettings
    switch (globalTimerState.value.pomodoroPhase) {
      case 'focus':
        return settings.focusTime
      case 'shortBreak':
        return settings.shortBreakTime
      case 'longBreak':
        return settings.longBreakTime
      default:
        return settings.focusTime
    }
  }

  const completePomodoroSession = () => {
    pauseTimer()
    
    if (globalTimerState.value.pomodoroPhase === 'focus') {
      globalTimerState.value.completedFocusSessions++
      globalTimerState.value.currentSessionCount++
      
      // Determine next break type
      if (globalTimerState.value.currentSessionCount % globalTimerState.value.pomodoroSettings.longBreakInterval === 0) {
        globalTimerState.value.pomodoroPhase = 'longBreak'
        globalTimerState.value.completedCycles++
      } else {
        globalTimerState.value.pomodoroPhase = 'shortBreak'
      }
    } else {
      globalTimerState.value.completedBreaks++
      globalTimerState.value.pomodoroPhase = 'focus'
    }

    globalTimerState.value.timeLeft = getCurrentPomodoroTime() * 60
    showNotification()
    saveState()
  }

  const showNotification = () => {
    const message = globalTimerState.value.mode === 'countup' 
      ? 'カウントアップタイマーが動作中です'
      : globalTimerState.value.pomodoroPhase === 'focus' 
        ? '休憩時間です！少し休憩しましょう。'
        : '集中時間です！作業を再開しましょう。'
    
    // Request notification permission if not granted
    if ('Notification' in window) {
      if (Notification.permission === 'default') {
        Notification.requestPermission()
      }
      
      if (Notification.permission === 'granted') {
        new Notification('タイマー通知', {
          body: message,
          icon: '/favicon.ico'
        })
      }
    }
  }

  const formatTime = (seconds: number): string => {
    const hours = Math.floor(seconds / 3600)
    const minutes = Math.floor((seconds % 3600) / 60)
    const remainingSeconds = seconds % 60
    
    if (hours > 0) {
      return `${hours}:${minutes.toString().padStart(2, '0')}:${remainingSeconds.toString().padStart(2, '0')}`
    }
    return `${minutes.toString().padStart(2, '0')}:${remainingSeconds.toString().padStart(2, '0')}`
  }

  const updatePomodoroSettings = (settings: Partial<typeof globalTimerState.value.pomodoroSettings>) => {
    globalTimerState.value.pomodoroSettings = { ...globalTimerState.value.pomodoroSettings, ...settings }
    if (!globalTimerState.value.isRunning && globalTimerState.value.mode === 'pomodoro') {
      globalTimerState.value.timeLeft = getCurrentPomodoroTime() * 60
    }
    saveState()
  }

  // Initialize state on first use
  if (typeof window !== 'undefined') {
    loadState()
  }

  return {
    // State
    state: globalTimerState,
    
    // Computed
    isRunning: computed(() => globalTimerState.value.isRunning),
    mode: computed(() => globalTimerState.value.mode),
    timeElapsed: computed(() => globalTimerState.value.timeElapsed),
    timeLeft: computed(() => globalTimerState.value.timeLeft),
    pomodoroPhase: computed(() => globalTimerState.value.pomodoroPhase),
    currentTask: computed({
      get: () => globalTimerState.value.currentTask,
      set: (value: string) => {
        globalTimerState.value.currentTask = value
        saveState()
      }
    }),
    
    // Methods
    startTimer,
    pauseTimer,
    resetTimer,
    switchMode,
    updatePomodoroSettings,
    formatTime,
    showNotification,
    loadState,
    saveState
  }
}