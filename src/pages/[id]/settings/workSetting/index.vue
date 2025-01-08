<template>
  <main>
    <h1 class="text-2xl font-semibold">設定を選択してください</h1>
    <div class="flex justify-end mt-4">
      <NuxtLink
        :to="{
          name: 'id-settings-workSetting-add',
          params: { id: user?.prop.id },
        }"
        class="bg-indigo-500 hover:bg-indigo-700 text-white font-bold py-2 px-4 rounded"
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
import Header from "~/components/Header.vue";
import { UserData } from "~/models/user";
import { workSettingData, type workSetting } from "~/models/workSetting";
import { UserRepository } from "~/repositories/tauri-commands/user";
import { WorkSettingRepository } from "~/repositories/tauri-commands/workTimeSetting";

export default {
  components: {
    Header,
  },
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

<style scoped>
body {
  font-family: "Arial", sans-serif;
  background-color: #f4f4f9;
  margin: 0;
  padding: 0;
}

header {
  background-color: #343a40;
  padding: 10px 0;
}

nav ul {
  display: flex;
  list-style: none;
  padding: 0;
  margin: 0;
  justify-content: center;
}

nav ul li {
  margin-right: 20px;
}

nav ul li a,
nav ul li button {
  color: white;
  text-decoration: none;
  background: none;
  border: none;
  cursor: pointer;
  font-size: 16px;
}

nav ul li button:hover,
nav ul li a:hover {
  text-decoration: underline;
}

main {
  padding: 20px;
  text-align: center;
}

.user-list {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
  justify-content: center;
}

.user-button {
  padding: 10px 20px;
  border: none;
  background-color: #007bff;
  color: white;
  cursor: pointer;
  border-radius: 5px;
  transition: background-color 0.3s;
}

.user-button:hover {
  background-color: #0056b3;
}

h2 {
  color: #343a40;
}

p {
  color: #6c757d;
}
</style>
