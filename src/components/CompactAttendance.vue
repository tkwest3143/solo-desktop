<template>
  <div class="h-full flex flex-col p-4 space-y-4">
    <!-- Status Display -->
    <div class="bg-slate-50 rounded-lg p-3 text-center">
      <div v-if="!isWorking() && !isResting()" class="flex items-center justify-center space-x-2">
        <Icon name="fluent:sleep-20-filled" class="text-slate-400" size="1.5em" />
        <span class="text-sm font-medium text-slate-600">未出勤</span>
      </div>
      <div v-else-if="isWorking()" class="flex items-center justify-center space-x-2">
        <Icon name="fluent:people-team-20-filled" class="text-blue-500" size="1.5em" />
        <span class="text-sm font-medium text-blue-600">勤務中</span>
      </div>
      <div v-else-if="isResting()" class="flex items-center justify-center space-x-2">
        <Icon name="fluent:drink-coffee-20-filled" class="text-orange-500" size="1.5em" />
        <span class="text-sm font-medium text-orange-600">休憩中</span>
      </div>
    </div>

    <!-- Action Buttons Grid -->
    <div class="grid grid-cols-2 gap-3 flex-1">
      <!-- Start Work Button -->
      <button
        @click="startWork"
        :disabled="isWorking() || isResting()"
        class="bg-gradient-to-br from-blue-500 to-blue-600 hover:from-blue-600 hover:to-blue-700 disabled:from-slate-300 disabled:to-slate-400 text-white rounded-lg p-3 transition-all duration-200 shadow-md hover:shadow-lg disabled:cursor-not-allowed flex flex-col items-center justify-center space-y-1"
      >
        <Icon name="fluent:clock-arrow-download-20-filled" size="1.2em" />
        <span class="text-xs font-medium">出勤</span>
      </button>

      <!-- End Work Button -->
      <button
        @click="endWork"
        :disabled="!isWorking()"
        class="bg-gradient-to-br from-red-500 to-red-600 hover:from-red-600 hover:to-red-700 disabled:from-slate-300 disabled:to-slate-400 text-white rounded-lg p-3 transition-all duration-200 shadow-md hover:shadow-lg disabled:cursor-not-allowed flex flex-col items-center justify-center space-y-1"
      >
        <Icon name="fluent:arrow-right-20-filled" size="1.2em" />
        <span class="text-xs font-medium">退勤</span>
      </button>

      <!-- Start Rest Button -->
      <button
        @click="startRest"
        :disabled="isResting() || isNone()"
        class="bg-gradient-to-br from-orange-500 to-orange-600 hover:from-orange-600 hover:to-orange-700 disabled:from-slate-300 disabled:to-slate-400 text-white rounded-lg p-3 transition-all duration-200 shadow-md hover:shadow-lg disabled:cursor-not-allowed flex flex-col items-center justify-center space-y-1"
      >
        <Icon name="fluent:drink-coffee-20-filled" size="1.2em" />
        <span class="text-xs font-medium">休憩開始</span>
      </button>

      <!-- End Rest Button -->
      <button
        @click="endRest"
        :disabled="!isResting()"
        class="bg-gradient-to-br from-green-500 to-green-600 hover:from-green-600 hover:to-green-700 disabled:from-slate-300 disabled:to-slate-400 text-white rounded-lg p-3 transition-all duration-200 shadow-md hover:shadow-lg disabled:cursor-not-allowed flex flex-col items-center justify-center space-y-1"
      >
        <Icon name="fluent:briefcase-20-filled" size="1.2em" />
        <span class="text-xs font-medium">休憩終了</span>
      </button>
    </div>

    <!-- Current Time -->
    <div class="text-center">
      <div class="text-lg font-bold text-slate-800">{{ currentTime }}</div>
      <div class="text-xs text-slate-600">{{ currentDate }}</div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { formatDate } from "date-fns";
import { useRoute } from "vue-router";
import { UserData } from "~/models/user";
import { workSettingData } from "~/models/workSetting";
import { UserRepository } from "~/repositories/tauri-commands/user";
import { WorkSettingRepository } from "~/repositories/tauri-commands/workTimeSetting";
import { WorkTimeRepository } from "~/repositories/tauri-commands/workTime";

enum Working {
  none,
  working,
  resting,
}

export default defineComponent({
  data() {
    return {
      working: Working.none,
      currentTime: new Date().toLocaleTimeString(),
      currentDate: new Date().toLocaleDateString("ja-JP", {
        year: "numeric",
        month: "long",
        day: "numeric",
        weekday: "long",
      }),
      user: undefined as UserData | undefined,
      workSetting: undefined as workSettingData | undefined,
    };
  },
  async mounted() {
    const route = useRoute();
    const userId = route.params.id as string;
    
    if (userId) {
      this.user = new UserData(await UserRepository.getById(parseInt(userId)));
      const workSettingRes = await WorkSettingRepository.getByUserId(parseInt(userId));
      const userWorkSetting = workSettingRes.find(
        (ws) => ws.id === this.user?.prop.default_work_setting_id
      );
      if (this.user.prop.default_work_setting_id && userWorkSetting) {
        this.workSetting = new workSettingData(userWorkSetting);
      }
    }
    
    setInterval(() => {
      this.currentTime = new Date().toLocaleTimeString();
    }, 1000);
  },
  methods: {
    isWorking() {
      return this.working === Working.working;
    },
    isResting() {
      return this.working === Working.resting;
    },
    isNone() {
      return this.working === Working.none;
    },
    async startWork() {
      this.working = Working.working;
      if (!this.user) return;

      if (this.workSetting) {
        const startTime = new Date();
        startTime.setMinutes(
          Math.ceil(startTime.getMinutes() / this.workSetting.prop.working_unit) *
            this.workSetting.prop.working_unit
        );
        await WorkTimeRepository.create({
          userId: this.user.prop.id,
          targetDay: formatDate(new Date(), "yyyy-MM-dd"),
          start: startTime,
        });
        return;
      }
      await WorkTimeRepository.create({
        userId: this.user.prop.id,
        targetDay: formatDate(new Date(), "yyyy-MM-dd"),
        start: new Date(),
      });
    },
    async endWork() {
      this.working = Working.none;
      if (!this.user) return;

      if (this.workSetting) {
        const endTime = new Date();
        endTime.setMinutes(
          Math.ceil(endTime.getMinutes() / this.workSetting.prop.working_unit) *
            this.workSetting.prop.working_unit
        );
        await WorkTimeRepository.update({
          userId: this.user.prop.id,
          targetDay: formatDate(new Date(), "yyyy-MM-dd"),
          end: endTime,
        });
        return;
      }
      await WorkTimeRepository.update({
        userId: this.user.prop.id,
        targetDay: formatDate(new Date(), "yyyy-MM-dd"),
        end: new Date(),
      });
    },
    async startRest() {
      this.working = Working.resting;
      if (!this.user) return;

      if (this.workSetting) {
        const restStartTime = new Date();
        restStartTime.setMinutes(
          Math.ceil(restStartTime.getMinutes() / this.workSetting.prop.working_unit) *
            this.workSetting.prop.working_unit
        );
        await WorkTimeRepository.update({
          userId: this.user.prop.id,
          targetDay: formatDate(new Date(), "yyyy-MM-dd"),
          restStart: restStartTime,
        });
        return;
      }
      await WorkTimeRepository.update({
        userId: this.user.prop.id,
        targetDay: formatDate(new Date(), "yyyy-MM-dd"),
        restStart: new Date(),
      });
    },
    async endRest() {
      this.working = Working.working;
      if (!this.user) return;

      if (this.workSetting) {
        const restEndTime = new Date();
        restEndTime.setMinutes(
          Math.ceil(restEndTime.getMinutes() / this.workSetting.prop.working_unit) *
            this.workSetting.prop.working_unit
        );
        await WorkTimeRepository.update({
          userId: this.user.prop.id,
          targetDay: formatDate(new Date(), "yyyy-MM-dd"),
          restEnd: restEndTime,
        });
        return;
      }
      await WorkTimeRepository.update({
        userId: this.user.prop.id,
        targetDay: formatDate(new Date(), "yyyy-MM-dd"),
        restEnd: new Date(),
      });
    },
  },
});
</script>