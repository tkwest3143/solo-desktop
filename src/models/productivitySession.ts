export interface ProductivitySession {
  id: string
  userId: number
  type: 'pomodoro' | 'timeblock' | 'deepwork'
  title: string
  description?: string
  duration: number // in minutes
  startTime: Date
  endTime?: Date
  completed: boolean
  targetDay: string // YYYY-MM-DD format
  createdAt: Date
  updatedAt: Date
  metadata?: {
    pomodoroSettings?: {
      focusTime: number
      shortBreakTime: number
      longBreakTime: number
      longBreakInterval: number
    }
    deepWorkMode?: string
    timeBlockCategory?: string
  }
}

export class ProductivitySessionData {
  constructor(public prop: ProductivitySession) {}

  get durationText(): string {
    const hours = Math.floor(this.prop.duration / 60)
    const minutes = this.prop.duration % 60
    if (hours > 0) {
      return `${hours}時間${minutes}分`
    }
    return `${minutes}分`
  }

  get statusText(): string {
    return this.prop.completed ? '完了' : '中断'
  }

  get typeText(): string {
    switch (this.prop.type) {
      case 'pomodoro':
        return 'ポモドーロ'
      case 'timeblock':
        return 'タイムブロック'
      case 'deepwork':
        return 'ディープワーク'
      default:
        return this.prop.type
    }
  }

  get startTimeText(): string {
    return this.prop.startTime.toLocaleTimeString('ja-JP', {
      hour: '2-digit',
      minute: '2-digit'
    })
  }

  get endTimeText(): string {
    if (!this.prop.endTime) return ''
    return this.prop.endTime.toLocaleTimeString('ja-JP', {
      hour: '2-digit',
      minute: '2-digit'
    })
  }

  get dateText(): string {
    return this.prop.startTime.toLocaleDateString('ja-JP', {
      year: 'numeric',
      month: 'short',
      day: 'numeric'
    })
  }

  toWorkTimeCompatible() {
    // This could be used to integrate with existing work time tracking
    // if the team decides to link productivity sessions with work hours
    return {
      targetDay: this.prop.targetDay,
      start: this.prop.startTime,
      end: this.prop.endTime,
      memo: `${this.typeText}: ${this.prop.title}`,
      duration: this.prop.duration
    }
  }
}