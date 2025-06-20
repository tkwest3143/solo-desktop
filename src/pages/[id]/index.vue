<template>
  <Loading v-if="isLoading" />
  <div v-else class="min-h-screen bg-gradient-to-br from-slate-50 to-blue-50">
    <!-- Page Header -->
    <div class="bg-white shadow-sm border-b border-slate-200">
      <div class="container mx-auto px-6 py-8">
        <div class="flex items-center justify-between">
          <div>
            <h1 class="text-3xl font-bold text-slate-800 mb-2">
              おかえりなさい、{{ user?.prop.name }}さん
            </h1>
            <p class="text-slate-600">今日も1日お疲れ様です</p>
          </div>
          <div class="text-right">
            <div class="text-2xl font-bold text-slate-800">{{ currentTime }}</div>
            <div class="text-sm text-slate-600">{{ currentDate }} {{ currentDay }}</div>
          </div>
        </div>
      </div>
    </div>

    <div class="container mx-auto px-6 py-8">
      <!-- Status Card -->
      <div class="mb-8">
        <div class="bg-white rounded-xl shadow-lg border border-slate-200 p-6">
          <div class="flex items-center justify-center mb-4">
            <div class="flex items-center space-x-3">
              <div v-if="!isWorking() && !isResting()" class="flex items-center space-x-2">
                <Icon name="fluent:sleep-20-filled" class="text-slate-400" size="2.5em" />
                <span class="text-2xl font-semibold text-slate-600">未出勤</span>
              </div>
              <div v-else-if="isWorking()" class="flex items-center space-x-2">
                <Icon name="fluent:people-team-20-filled" class="text-blue-500" size="2.5em" />
                <span class="text-2xl font-semibold text-blue-600">勤務中</span>
              </div>
              <div v-else-if="isResting()" class="flex items-center space-x-2">
                <Icon name="fluent:drink-coffee-20-filled" class="text-orange-500" size="2.5em" />
                <span class="text-2xl font-semibold text-orange-600">休憩中</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Action Buttons -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <!-- Start Work Button -->
        <button
          @click="startWork"
          :disabled="isWorking() || isResting()"
          class="bg-gradient-to-br from-blue-500 to-blue-600 hover:from-blue-600 hover:to-blue-700 disabled:from-slate-300 disabled:to-slate-400 text-white rounded-xl p-6 transition-all duration-200 shadow-lg hover:shadow-xl transform hover:-translate-y-1 disabled:transform-none disabled:cursor-not-allowed"
        >
          <div class="flex flex-col items-center space-y-3">
            <div class="w-12 h-12 bg-white bg-opacity-20 rounded-full flex items-center justify-center">
              <Icon name="fluent:clock-arrow-download-20-filled" size="1.5em" />
            </div>
            <span class="font-semibold">出勤</span>
          </div>
        </button>

        <!-- End Work Button -->
        <button
          @click="endWork"
          :disabled="!isWorking()"
          class="bg-gradient-to-br from-red-500 to-red-600 hover:from-red-600 hover:to-red-700 disabled:from-slate-300 disabled:to-slate-400 text-white rounded-xl p-6 transition-all duration-200 shadow-lg hover:shadow-xl transform hover:-translate-y-1 disabled:transform-none disabled:cursor-not-allowed"
        >
          <div class="flex flex-col items-center space-y-3">
            <div class="w-12 h-12 bg-white bg-opacity-20 rounded-full flex items-center justify-center">
              <Icon name="fluent:clock-arrow-upload-20-filled" size="1.5em" />
            </div>
            <span class="font-semibold">退勤</span>
          </div>
        </button>

        <!-- Start Rest Button -->
        <button
          @click="startRest"
          :disabled="isResting() || isNone()"
          class="bg-gradient-to-br from-orange-500 to-orange-600 hover:from-orange-600 hover:to-orange-700 disabled:from-slate-300 disabled:to-slate-400 text-white rounded-xl p-6 transition-all duration-200 shadow-lg hover:shadow-xl transform hover:-translate-y-1 disabled:transform-none disabled:cursor-not-allowed"
        >
          <div class="flex flex-col items-center space-y-3">
            <div class="w-12 h-12 bg-white bg-opacity-20 rounded-full flex items-center justify-center">
              <Icon name="fluent:drink-coffee-20-filled" size="1.5em" />
            </div>
            <span class="font-semibold">休憩開始</span>
          </div>
        </button>

        <!-- End Rest Button -->
        <button
          @click="endRest"
          :disabled="!isResting()"
          class="bg-gradient-to-br from-green-500 to-green-600 hover:from-green-600 hover:to-green-700 disabled:from-slate-300 disabled:to-slate-400 text-white rounded-xl p-6 transition-all duration-200 shadow-lg hover:shadow-xl transform hover:-translate-y-1 disabled:transform-none disabled:cursor-not-allowed"
        >
          <div class="flex flex-col items-center space-y-3">
            <div class="w-12 h-12 bg-white bg-opacity-20 rounded-full flex items-center justify-center">
              <Icon name="fluent:briefcase-20-filled" size="1.5em" />
            </div>
            <span class="font-semibold">休憩終了</span>
          </div>
        </button>
      </div>

      <!-- Quick Navigation -->
      <div class="mt-8">
        <h2 class="text-xl font-semibold text-slate-800 mb-4">クイックアクセス</h2>
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
          <NuxtLink
            :to="{ name: 'id-workTimeList', params: { id: user?.prop.id } }"
            class="bg-white border-2 border-slate-200 hover:border-blue-300 rounded-lg p-4 text-center transition-all duration-200 hover:shadow-lg group"
          >
            <Icon name="fluent:clock-20-filled" size="2em" class="text-blue-500 mb-2 group-hover:scale-110 transition-transform" />
            <h3 class="font-semibold text-slate-800">勤怠一覧</h3>
            <p class="text-sm text-slate-600">勤務記録を確認</p>
          </NuxtLink>

          <NuxtLink
            :to="{ name: 'id-report', params: { id: user?.prop.id } }"
            class="bg-white border-2 border-slate-200 hover:border-green-300 rounded-lg p-4 text-center transition-all duration-200 hover:shadow-lg group"
          >
            <Icon name="fluent:document-data-20-filled" size="2em" class="text-green-500 mb-2 group-hover:scale-110 transition-transform" />
            <h3 class="font-semibold text-slate-800">レポート</h3>
            <p class="text-sm text-slate-600">勤務データ分析</p>
          </NuxtLink>

          <NuxtLink
            :to="{ name: 'id-todo', params: { id: user?.prop.id } }"
            class="bg-white border-2 border-slate-200 hover:border-purple-300 rounded-lg p-4 text-center transition-all duration-200 hover:shadow-lg group"
          >
            <Icon name="fluent:task-list-square-20-filled" size="2em" class="text-purple-500 mb-2 group-hover:scale-110 transition-transform" />
            <h3 class="font-semibold text-slate-800">Todo管理</h3>
            <p class="text-sm text-slate-600">タスクを管理</p>
          </NuxtLink>

          <NuxtLink
            :to="{ name: 'id-settings', params: { id: user?.prop.id } }"
            class="bg-white border-2 border-slate-200 hover:border-slate-300 rounded-lg p-4 text-center transition-all duration-200 hover:shadow-lg group"
          >
            <Icon name="fluent:settings-20-filled" size="2em" class="text-slate-500 mb-2 group-hover:scale-110 transition-transform" />
            <h3 class="font-semibold text-slate-800">設定</h3>
            <p class="text-sm text-slate-600">アプリ設定</p>
          </NuxtLink>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { formatDate } from "date-fns";
import { defineComponent } from "vue";
import { useRoute } from "vue-router";
import { UserData } from "~/models/user";
import { workSettingData } from "~/models/workSetting";
import { UserRepository } from "~/repositories/tauri-commands/user";
import { WorkTimeRepository } from "~/repositories/tauri-commands/workTime";
import { WorkSettingRepository } from "~/repositories/tauri-commands/workTimeSetting";

enum Working {
  working = "working",
  resting = "resting",
  none = "none",
}
export default defineComponent({
  data() {
    return {
      user: undefined as UserData | undefined,
      currentDate: new Date().toLocaleDateString(),
      currentDay: new Date().toLocaleDateString("ja-JP", { weekday: "long" }),
      currentTime: new Date().toLocaleTimeString(),
      working: Working.none,
      isLoading: true,
      workSetting: undefined as workSettingData | undefined,
    };
  },
  async mounted() {
    const route = useRoute();
    const userId = route.params.id as string;
    this.user = new UserData(await UserRepository.getById(parseInt(userId)));
    const workSettingRes = await WorkSettingRepository.getByUserId(
      parseInt(userId)
    );
    const userWorkSetting = workSettingRes.find(
      (ws) => ws.id === this.user?.prop.default_work_setting_id
    );
    if (this.user.prop.default_work_setting_id && userWorkSetting) {
      this.workSetting = new workSettingData(userWorkSetting);
    }
    setInterval(() => {
      this.currentTime = new Date().toLocaleTimeString();
    }, 1000);
    this.isLoading = false;
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
      if (this.workSetting) {
        const startTime = new Date();
        startTime.setMinutes(
          Math.ceil(
            startTime.getMinutes() / this.workSetting.prop.working_unit
          ) * this.workSetting.prop.working_unit
        );
        await WorkTimeRepository.create({
          userId: this.user!.prop.id,
          targetDay: formatDate(new Date(), "yyyy-MM-dd"),
          start: startTime,
        });
        return;
      }
      await WorkTimeRepository.create({
        userId: this.user!.prop.id,
        targetDay: formatDate(new Date(), "yyyy-MM-dd"),
        start: new Date(),
      });
    },
    async endWork() {
      this.working = Working.none;
      if (this.workSetting) {
        const endTime = new Date();
        endTime.setMinutes(
          Math.ceil(endTime.getMinutes() / this.workSetting.prop.working_unit) *
            this.workSetting.prop.working_unit
        );
        await WorkTimeRepository.update({
          userId: this.user!.prop.id,
          targetDay: formatDate(new Date(), "yyyy-MM-dd"),
          end: endTime,
        });
        return;
      }
      await WorkTimeRepository.update({
        userId: this.user!.prop.id,
        targetDay: formatDate(new Date(), "yyyy-MM-dd"),
        end: new Date(),
      });
    },
    async startRest() {
      this.working = Working.resting;
      if (this.workSetting) {
        const restStartTime = new Date();
        restStartTime.setMinutes(
          Math.ceil(
            restStartTime.getMinutes() / this.workSetting.prop.working_unit
          ) * this.workSetting.prop.working_unit
        );
        await WorkTimeRepository.update({
          userId: this.user!.prop.id,
          targetDay: formatDate(new Date(), "yyyy-MM-dd"),
          restStart: restStartTime,
        });
        return;
      }
      await WorkTimeRepository.update({
        userId: this.user!.prop.id,
        targetDay: formatDate(new Date(), "yyyy-MM-dd"),
        restStart: new Date(),
      });
    },
    async endRest() {
      this.working = Working.working;
      if (this.workSetting) {
        const restEndTime = new Date();
        restEndTime.setMinutes(
          Math.ceil(
            restEndTime.getMinutes() / this.workSetting.prop.working_unit
          ) * this.workSetting.prop.working_unit
        );
        await WorkTimeRepository.update({
          userId: this.user!.prop.id,
          targetDay: formatDate(new Date(), "yyyy-MM-dd"),
          restEnd: restEndTime,
        });
        return;
      }
      await WorkTimeRepository.update({
        userId: this.user!.prop.id,
        targetDay: formatDate(new Date(), "yyyy-MM-dd"),
        restEnd: new Date(),
      });
    },
  },
});
</script>
