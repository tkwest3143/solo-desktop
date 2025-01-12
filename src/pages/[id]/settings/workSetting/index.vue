<template>
  <main class="flex flex-col items-center">
    <h1 class="text-2xl font-semibold">設定を選択してください</h1>
    <div class="flex justify-end mt-4">
      <NuxtLink
        :to="{
          name: 'id-settings-workSetting-add',
          params: { id: user?.prop.id },
        }"
        class="bg-primary-500 hover:bg-primary-700 text-white font-bold py-2 px-4 rounded"
      >
        設定を追加する
      </NuxtLink>
    </div>
    <div v-if="workSettings.length">
      <div class="user-list mt-4">
        <NuxtLink
          v-for="workSetting in workSettings"
          :key="workSetting.prop.id"
          class="user-button w-full"
          :to="{
            name: 'id-settings-workSetting-workSettingId',
            params: { id: user?.prop.id, workSettingId: workSetting.prop.id },
          }"
        >
          {{ workSetting.prop.title }}
          <div>登録日{{ workSetting.createdAtText }}</div>
        </NuxtLink>
      </div>
    </div>
    <div v-else>
      <p>設定情報がありません</p>
    </div>
  </main>
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
