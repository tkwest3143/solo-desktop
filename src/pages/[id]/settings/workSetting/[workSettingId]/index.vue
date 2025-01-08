<template>
  <div class="container mx-auto p-4">
    <div class="bg-white shadow-md rounded-lg p-6">
      <h2 class="text-2xl font-bold mb-4">ユーザ設定</h2>
    </div>
  </div>
</template>

<script lang="ts">
import { UserData } from "~/models/user";
import type { workSettingData } from "~/models/workSetting";
import { UserRepository } from "~/repositories/tauri-commands/user";

export default {
  data() {
    return {
      user: undefined as UserData | undefined,
      workSettings: [] as workSettingData[],
      isLoading: true,
    };
  },
  async mounted() {
    const userId = this.$route.params.id as string;
    this.user = new UserData(await UserRepository.getById(parseInt(userId)));
  },
  methods: {},
};
</script>

<style scoped>
.container {
  max-width: 600px;
}
</style>
