<template>
  <div class="container mx-auto p-4">
    <div class="bg-white shadow-md rounded-lg p-6">
      <h2 class="text-2xl font-bold mb-4">ユーザ設定</h2>
      <div class="mb-4">
        <label
          class="block text-gray-700 text-sm font-bold mb-2"
          for="username"
        >
          ユーザ名
        </label>
        <input
          class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
          id="username"
          type="text"
          placeholder="ユーザ名を入力"
          v-model="user.prop.name"
        />
      </div>
      <div class="mb-4">
        <label class="block text-gray-700 text-sm font-bold mb-2" for="email">
          メールアドレス
        </label>
        <input
          class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
          id="email"
          type="email"
          placeholder="メールアドレスを入力"
          v-model="user.prop.email"
        />
      </div>
      <div class="flex items-center justify-between">
        <button
          class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
          type="button"
          @click="saveSettings"
        >
          保存
        </button>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { UserData } from "~/models/user";
import { UserRepository } from "~/repositories/tauri-commands/user";

export default {
  async setup() {
    const route = useRoute();
    const userId = route.params.id as string;
    const user = ref(
      new UserData(await UserRepository.getById(parseInt(userId)))
    );
    return {
      user,
    };
  },
  data() {
    return {
      isLoading: true,
    };
  },
  async mounted() {
    const route = useRoute();
    const userId = route.params.id as string;
    this.user = new UserData(await UserRepository.getById(parseInt(userId)));
  },
  methods: {
    async saveSettings() {
      await UserRepository.update({
        id: this.user!.prop.id,
        name: this.user!.prop.name,
        email: this.user!.prop.email,
      });
    },
  },
};
</script>

<style scoped>
.container {
  max-width: 600px;
}
</style>
