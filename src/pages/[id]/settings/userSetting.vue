<template>
  <div class="container mx-auto p-4">
    <div class="bg-white shadow-md rounded-lg p-6">
      <h2 class="text-2xl font-bold mb-4">ユーザ設定</h2>
      <CommonInput
        id="username"
        label="ユーザ名"
        type="text"
        placeholder="ユーザ名を入力"
        v-model="user.prop.name"
      />
      <CommonInput
        id="email"
        label="メールアドレス"
        type="email"
        placeholder="メールアドレスを入力"
        v-model="user.prop.email"
      />
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
import CommonInput from "~/components/CommonInput.vue";
import { UserData } from "~/models/user";
import { UserRepository } from "~/repositories/tauri-commands/user";

export default {
  components: {
    CommonInput,
  },
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
