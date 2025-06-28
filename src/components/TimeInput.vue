<template>
  <div class="time-input-container w-full">
    <div
      class="flex items-center space-x-2 bg-white border border-slate-300 rounded-lg focus-within:ring-2 focus-within:ring-blue-500 focus-within:border-transparent transition-all duration-200"
    >
      <!-- Hour input -->
      <div class="flex items-center">
        <button
          type="button"
          @click="adjustHour(-1)"
          class="p-1 text-slate-500 hover:text-slate-700 hover:bg-slate-100 rounded transition-colors"
          :disabled="disabled"
        >
          <Icon
            name="fluent:chevron-up-20-filled"
            size="0.9em"
            class="rotate-180"
          />
        </button>
        <input
          ref="hourInput"
          v-model="localHour"
          @blur="validateAndFormatHour"
          @input="onHourInput"
          @keydown="onHourKeydown"
          type="text"
          inputmode="numeric"
          class="w-8 text-center border-0 focus:outline-none text-sm font-medium text-slate-900 bg-transparent"
          :disabled="disabled"
          maxlength="2"
          placeholder="00"
        />
        <button
          type="button"
          @click="adjustHour(1)"
          class="p-1 text-slate-500 hover:text-slate-700 hover:bg-slate-100 rounded transition-colors"
          :disabled="disabled"
        >
          <Icon name="fluent:chevron-up-20-filled" size="0.9em" />
        </button>
      </div>

      <span class="text-slate-400 font-medium">:</span>

      <!-- Minute input -->
      <div class="flex items-center">
        <button
          type="button"
          @click="adjustMinute(-15)"
          class="p-1 text-slate-500 hover:text-slate-700 hover:bg-slate-100 rounded transition-colors"
          :disabled="disabled"
        >
          <Icon
            name="fluent:chevron-up-20-filled"
            size="0.9em"
            class="rotate-180"
          />
        </button>
        <input
          ref="minuteInput"
          v-model="localMinute"
          @blur="validateAndFormatMinute"
          @input="onMinuteInput"
          @keydown="onMinuteKeydown"
          type="text"
          inputmode="numeric"
          class="w-8 text-center border-0 focus:outline-none text-sm font-medium text-slate-900 bg-transparent"
          :disabled="disabled"
          maxlength="2"
          placeholder="00"
        />
        <button
          type="button"
          @click="adjustMinute(15)"
          class="p-1 text-slate-500 hover:text-slate-700 hover:bg-slate-100 rounded transition-colors"
          :disabled="disabled"
        >
          <Icon name="fluent:chevron-up-20-filled" size="0.9em" />
        </button>
      </div>

      <!-- Quick time picker button -->
      <div class="relative">
        <button
          type="button"
          @click="toggleQuickPicker"
          class="p-2 text-slate-500 hover:text-slate-700 hover:bg-slate-100 rounded transition-colors"
          :disabled="disabled"
          v-click-outside="closeQuickPicker"
          title="クイック選択"
        >
          <Icon name="fluent:clock-20-filled" size="1em" />
        </button>

        <!-- Quick picker dropdown -->
        <Transition
          enter-active-class="transition ease-out duration-200"
          enter-from-class="opacity-0 scale-95"
          enter-to-class="opacity-100 scale-100"
          leave-active-class="transition ease-in duration-150"
          leave-from-class="opacity-100 scale-100"
          leave-to-class="opacity-0 scale-95"
        >
          <div
            v-if="showQuickPicker"
            class="absolute top-full right-0 mt-1 bg-white border border-slate-200 rounded-lg shadow-lg z-50 w-48 max-h-64 overflow-y-auto"
          >
            <div class="p-2">
              <div class="text-xs font-medium text-slate-500 mb-2 px-2">
                よく使用する時間
              </div>
              <div class="grid grid-cols-2 gap-1">
                <button
                  v-for="time in quickTimes"
                  :key="time"
                  type="button"
                  @click="selectQuickTime(time)"
                  class="px-2 py-1 text-xs text-slate-700 hover:bg-blue-50 hover:text-blue-700 rounded transition-colors text-center"
                >
                  {{ time }}
                </button>
              </div>
            </div>
          </div>
        </Transition>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "TimeInput",
  props: {
    modelValue: {
      type: String,
      default: "",
    },
    disabled: {
      type: Boolean,
      default: false,
    },
    placeholder: {
      type: String,
      default: "00:00",
    },
  },
  emits: ["update:modelValue"],
  data() {
    return {
      localHour: "",
      localMinute: "",
      showQuickPicker: false,
      quickTimes: [
        "08:00",
        "08:30",
        "09:00",
        "09:30",
        "10:00",
        "10:30",
        "11:00",
        "11:30",
        "12:00",
        "12:30",
        "13:00",
        "13:30",
        "14:00",
        "14:30",
        "15:00",
        "15:30",
        "16:00",
        "16:30",
        "17:00",
        "17:30",
        "18:00",
        "18:30",
        "19:00",
        "19:30",
        "20:00",
        "20:30",
        "21:00",
        "21:30",
      ],
    };
  },
  watch: {
    modelValue: {
      immediate: true,
      handler(newValue: string) {
        this.parseTime(newValue);
      },
    },
  },
  methods: {
    parseTime(timeString: string) {
      if (!timeString) {
        this.localHour = "";
        this.localMinute = "";
        return;
      }

      const [hour, minute] = timeString.split(":");
      this.localHour = hour || "";
      this.localMinute = minute || "";
    },

    formatTime(): string {
      const hour = this.localHour.padStart(2, "0");
      const minute = this.localMinute.padStart(2, "0");
      return `${hour}:${minute}`;
    },

    emitValue() {
      if (this.localHour && this.localMinute) {
        const formattedTime = this.formatTime();
        this.$emit("update:modelValue", formattedTime);
      } else if (!this.localHour && !this.localMinute) {
        this.$emit("update:modelValue", "");
      }
    },

    adjustHour(delta: number) {
      const currentHour = parseInt(this.localHour) || 0;
      const newHour = Math.max(0, Math.min(23, currentHour + delta));
      this.localHour = newHour.toString();
      this.validateAndFormatHour();
    },

    adjustMinute(delta: number) {
      const currentMinute = parseInt(this.localMinute) || 0;
      let newMinute = currentMinute + delta;
      let hourAdjustment = 0;

      if (newMinute >= 60) {
        hourAdjustment = Math.floor(newMinute / 60);
        newMinute = newMinute % 60;
      } else if (newMinute < 0) {
        hourAdjustment = Math.ceil(newMinute / 60) - 1;
        newMinute = newMinute + Math.abs(hourAdjustment) * 60;
      }

      this.localMinute = newMinute.toString();

      if (hourAdjustment !== 0) {
        this.adjustHour(hourAdjustment);
      }

      this.validateAndFormatMinute();
    },

    validateAndFormatHour() {
      let hour = parseInt(this.localHour);
      if (isNaN(hour)) {
        this.localHour = "";
      } else {
        hour = Math.max(0, Math.min(23, hour));
        this.localHour = hour.toString().padStart(2, "0");
      }
      this.emitValue();
    },

    validateAndFormatMinute() {
      let minute = parseInt(this.localMinute);
      if (isNaN(minute)) {
        this.localMinute = "";
      } else {
        minute = Math.max(0, Math.min(59, minute));
        this.localMinute = minute.toString().padStart(2, "0");
      }
      this.emitValue();
    },

    onHourInput(event: Event) {
      const input = event.target as HTMLInputElement;
      let value = input.value.replace(/[^0-9]/g, "");
      if (value.length > 2) value = value.slice(0, 2);
      if (parseInt(value) > 23) value = "23";
      this.localHour = value;
      input.value = value;
    },

    onMinuteInput(event: Event) {
      const input = event.target as HTMLInputElement;
      let value = input.value.replace(/[^0-9]/g, "");
      if (value.length > 2) value = value.slice(0, 2);
      if (parseInt(value) > 59) value = "59";
      this.localMinute = value;
      input.value = value;
    },

    onHourKeydown(event: KeyboardEvent) {
      this.handleKeydown(event, "hour");
    },

    onMinuteKeydown(event: KeyboardEvent) {
      this.handleKeydown(event, "minute");
    },

    handleKeydown(event: KeyboardEvent, type: "hour" | "minute") {
      if (event.key === "ArrowUp") {
        event.preventDefault();
        if (type === "hour") {
          this.adjustHour(1);
        } else {
          this.adjustMinute(15);
        }
      } else if (event.key === "ArrowDown") {
        event.preventDefault();
        if (type === "hour") {
          this.adjustHour(-1);
        } else {
          this.adjustMinute(-15);
        }
      } else if (event.key === "Tab") {
        if (type === "hour" && !event.shiftKey) {
          event.preventDefault();
          (this.$refs.minuteInput as HTMLInputElement)?.focus();
        }
      }
    },

    toggleQuickPicker() {
      this.showQuickPicker = !this.showQuickPicker;
    },

    closeQuickPicker() {
      this.showQuickPicker = false;
    },

    selectQuickTime(time: string) {
      this.closeQuickPicker();
      const [hour, minute] = time.split(":");
      this.localHour = hour;
      this.localMinute = minute;
      this.emitValue();
    },
  },

  directives: {
    "click-outside": {
      beforeMount(el: HTMLElement, binding: any) {
        (el as any)._clickOutside = (event: Event) => {
          if (!(el === event.target || el.contains(event.target as Node))) {
            binding.value();
          }
        };
        document.addEventListener("click", (el as any)._clickOutside);
      },
      unmounted(el: HTMLElement) {
        document.removeEventListener("click", (el as any)._clickOutside);
        delete (el as any)._clickOutside;
      },
    },
  },
});
</script>

<style scoped>
/* Custom scrollbar for quick picker */
.overflow-y-auto::-webkit-scrollbar {
  width: 4px;
}

.overflow-y-auto::-webkit-scrollbar-track {
  background: #f1f5f9;
}

.overflow-y-auto::-webkit-scrollbar-thumb {
  background: #cbd5e1;
  border-radius: 2px;
}

.overflow-y-auto::-webkit-scrollbar-thumb:hover {
  background: #94a3b8;
}
</style>
