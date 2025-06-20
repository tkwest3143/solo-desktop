<template>
  <div class="min-h-screen bg-gradient-to-br from-slate-50 to-blue-50 p-6">
    <div class="max-w-2xl mx-auto">
      <!-- Header Section -->
      <div class="bg-white/70 backdrop-blur-sm rounded-2xl shadow-xl border border-white/20 p-8 mb-8">
        <div class="flex items-center space-x-4">
          <div class="p-3 bg-gradient-to-r from-blue-500 to-blue-600 rounded-xl">
            <Icon name="fluent:person-20-filled" class="text-white" size="1.5em" />
          </div>
          <div>
            <h1 class="text-3xl font-bold text-slate-800">ユーザー設定</h1>
            <p class="text-slate-600 mt-1">アカウント情報を管理します</p>
          </div>
        </div>
      </div>

      <!-- Settings Form -->
      <div class="bg-white/70 backdrop-blur-sm rounded-2xl shadow-xl border border-white/20 p-8">
        <form @submit.prevent="saveSettings" class="space-y-6">
          <!-- Username Field -->
          <div class="space-y-2">
            <label class="flex items-center text-sm font-semibold text-slate-700" for="username">
              <Icon name="fluent:person-20-filled" class="mr-2 text-slate-500" size="1em" />
              ユーザー名
            </label>
            <input
              id="username"
              type="text"
              placeholder="ユーザー名を入力してください"
              v-model="user.prop.name"
              class="w-full px-4 py-3 bg-white/50 border border-slate-200 rounded-xl text-slate-900 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200 shadow-sm hover:shadow-md"
            />
          </div>

          <!-- Email Field -->
          <div class="space-y-2">
            <label class="flex items-center text-sm font-semibold text-slate-700" for="email">
              <Icon name="fluent:mail-20-filled" class="mr-2 text-slate-500" size="1em" />
              メールアドレス
            </label>
            <input
              id="email"
              type="email"
              placeholder="メールアドレスを入力してください"
              v-model="user.prop.email"
              class="w-full px-4 py-3 bg-white/50 border border-slate-200 rounded-xl text-slate-900 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200 shadow-sm hover:shadow-md"
            />
          </div>

          <!-- Save Button -->
          <div class="flex justify-end pt-6">
            <button
              type="submit"
              class="inline-flex items-center px-8 py-3 bg-gradient-to-r from-blue-500 to-blue-600 hover:from-blue-600 hover:to-blue-700 text-white font-semibold rounded-xl shadow-lg hover:shadow-xl transition-all duration-200 transform hover:scale-105 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
            >
              <Icon name="fluent:save-20-filled" class="mr-2" size="1.2em" />
              設定を保存
            </button>
          </div>
        </form>
      </div>

      <!-- Additional Info Card -->
      <div class="mt-8 bg-gradient-to-r from-blue-50 to-slate-50 rounded-2xl border border-blue-100 p-6">
        <div class="flex items-start space-x-3">
          <div class="p-2 bg-blue-100 rounded-lg">
            <Icon name="fluent:info-20-filled" class="text-blue-600" size="1.2em" />
          </div>
          <div>
            <h3 class="text-sm font-semibold text-slate-800 mb-1">設定について</h3>
            <p class="text-sm text-slate-600">
              ユーザー名とメールアドレスはアプリケーション内での識別とレポート生成に使用されます。
            </p>
          </div>
        </div>
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
