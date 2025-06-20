<template>
  <div class="min-h-screen bg-gradient-to-br from-slate-50 to-blue-50 p-6">
    <div class="max-w-4xl mx-auto">
      <!-- Header Section -->
      <div class="bg-white/70 backdrop-blur-sm rounded-2xl shadow-xl border border-white/20 p-8 mb-8">
        <div class="flex items-center justify-between">
          <div class="flex items-center space-x-4">
            <div class="p-3 bg-gradient-to-r from-blue-500 to-blue-600 rounded-xl">
              <Icon name="fluent:settings-20-filled" class="text-white" size="1.5em" />
            </div>
            <div>
              <h1 class="text-3xl font-bold text-slate-800">勤務設定</h1>
              <p class="text-slate-600 mt-1">勤務時間の設定を管理します</p>
            </div>
          </div>
          <NuxtLink
            :to="{
              name: 'id-settings-workSetting-add',
              params: { id: user?.prop.id },
            }"
            class="inline-flex items-center px-6 py-3 bg-gradient-to-r from-blue-500 to-blue-600 hover:from-blue-600 hover:to-blue-700 text-white font-semibold rounded-xl shadow-lg hover:shadow-xl transition-all duration-200 transform hover:scale-105"
          >
            <Icon name="fluent:add-20-filled" class="mr-2" size="1.2em" />
            設定を追加する
          </NuxtLink>
        </div>
      </div>

      <!-- Settings List -->
      <div v-if="workSettings.length" class="grid gap-6">
        <div
          v-for="workSetting in workSettings"
          :key="workSetting.prop.id"
          class="group"
        >
          <NuxtLink
            :to="{
              name: 'id-settings-workSetting-workSettingId',
              params: { id: user?.prop.id, workSettingId: workSetting.prop.id },
            }"
            class="block bg-white/70 backdrop-blur-sm rounded-2xl shadow-lg border border-white/20 p-6 hover:shadow-xl hover:bg-white/80 transition-all duration-300 transform hover:scale-[1.02]"
          >
            <div class="flex items-center justify-between">
              <div class="flex items-center space-x-4">
                <div class="p-3 bg-gradient-to-r from-slate-100 to-slate-200 group-hover:from-blue-100 group-hover:to-blue-200 rounded-xl transition-all duration-300">
                  <Icon name="fluent:clock-20-filled" class="text-slate-600 group-hover:text-blue-600" size="1.5em" />
                </div>
                <div>
                  <h3 class="text-xl font-semibold text-slate-800 group-hover:text-blue-600 transition-colors duration-200">
                    {{ workSetting.prop.title }}
                  </h3>
                  <p class="text-slate-500 mt-1">
                    <Icon name="fluent:calendar-20-filled" class="inline mr-1" size="0.9em" />
                    登録日: {{ workSetting.createdAtText }}
                  </p>
                </div>
              </div>
              <div class="flex items-center space-x-2 text-slate-400 group-hover:text-blue-500 transition-colors duration-200">
                <span class="text-sm font-medium">詳細を見る</span>
                <Icon name="fluent:chevron-right-20-filled" size="1.2em" />
              </div>
            </div>
          </NuxtLink>
        </div>
      </div>

      <!-- Empty State -->
      <div v-else class="text-center py-16">
        <div class="bg-white/70 backdrop-blur-sm rounded-2xl shadow-lg border border-white/20 p-12">
          <div class="p-4 bg-gradient-to-r from-slate-100 to-slate-200 rounded-2xl inline-block mb-6">
            <Icon name="fluent:settings-20-regular" class="text-slate-400" size="3em" />
          </div>
          <h3 class="text-xl font-semibold text-slate-600 mb-2">設定情報がありません</h3>
          <p class="text-slate-500 mb-8">新しい勤務設定を追加して開始しましょう</p>
          <NuxtLink
            :to="{
              name: 'id-settings-workSetting-add',
              params: { id: user?.prop.id },
            }"
            class="inline-flex items-center px-6 py-3 bg-gradient-to-r from-blue-500 to-blue-600 hover:from-blue-600 hover:to-blue-700 text-white font-semibold rounded-xl shadow-lg hover:shadow-xl transition-all duration-200 transform hover:scale-105"
          >
            <Icon name="fluent:add-20-filled" class="mr-2" size="1.2em" />
            最初の設定を追加する
          </NuxtLink>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { UserData } from "~/models/user";
import { workSettingData, type workSetting } from "~/models/workSetting";
import { UserRepository } from "~/repositories/tauri-commands/user";
import { WorkSettingRepository } from "~/repositories/tauri-commands/workTimeSetting";

export default {
  data() {
    return {
      user: undefined as UserData | undefined,
      workSettings: [] as workSettingData[],
    };
  },
  async mounted() {
    await this.init();
  },
  methods: {
    async init() {
      const userId = this.$route.params.id as string;
      this.user = new UserData(await UserRepository.getById(parseInt(userId)));
      await WorkSettingRepository.getByUserId(parseInt(userId)).then(
        (workSettings) => {
          this.workSettings = workSettings.map(
            (workSetting: workSetting) => new workSettingData(workSetting)
          );
        }
      );
    },
  },
};
</script>
