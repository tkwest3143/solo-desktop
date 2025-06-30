<template>
  <Loading v-if="isLoading" />
  <div v-else class="min-h-screen bg-gradient-to-br from-slate-50 to-blue-50">
    <!-- Page Header -->
    <div class="bg-white shadow-sm border-b border-slate-200">
      <div class="container mx-auto px-6 py-8">
        <div class="flex items-center justify-between">
          <div>
            <h1 class="text-3xl font-bold text-slate-800 mb-2">設定</h1>
            <p class="text-slate-600">
              {{ user?.prop.name }}さんのアカウント設定
            </p>
          </div>
          <div class="flex items-center space-x-2">
            <Icon
              name="fluent:person-20-filled"
              class="text-slate-400"
              size="1.5em"
            />
            <span class="text-lg font-medium text-slate-700">{{
              user?.prop.email
            }}</span>
          </div>
        </div>
      </div>
    </div>

    <div class="container mx-auto px-6 py-8">
      <!-- Settings Cards -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <!-- User Settings Card -->
        <NuxtLink
          :to="{
            name: 'id-settings-userSetting',
            params: { id: user?.prop.id },
          }"
          class="bg-white rounded-xl shadow-lg border border-slate-200 hover:shadow-xl hover:border-blue-300 transition-all duration-300 transform hover:-translate-y-1 p-6 group"
        >
          <div class="text-center">
            <div
              class="w-16 h-16 mx-auto mb-4 bg-gradient-to-br from-blue-500 to-blue-600 rounded-full flex items-center justify-center group-hover:scale-110 transition-transform duration-200"
            >
              <Icon
                name="fluent:person-settings-20-filled"
                class="text-white"
                size="2em"
              />
            </div>
            <h3 class="text-xl font-semibold text-slate-800 mb-2">
              ユーザー設定
            </h3>
            <p class="text-slate-600 mb-4">プロフィール情報や個人設定を管理</p>
            <div
              class="flex items-center justify-center text-blue-600 font-medium group-hover:text-blue-700"
            >
              設定を開く
              <Icon
                name="fluent:arrow-right-20-filled"
                class="ml-2"
                size="1.1em"
              />
            </div>
          </div>
        </NuxtLink>

        <!-- Work Settings Card -->
        <NuxtLink
          :to="{
            name: 'id-settings-workSetting',
            params: { id: user?.prop.id },
          }"
          class="bg-white rounded-xl shadow-lg border border-slate-200 hover:shadow-xl hover:border-green-300 transition-all duration-300 transform hover:-translate-y-1 p-6 group"
        >
          <div class="text-center">
            <div
              class="w-16 h-16 mx-auto mb-4 bg-gradient-to-br from-green-500 to-green-600 rounded-full flex items-center justify-center group-hover:scale-110 transition-transform duration-200"
            >
              <Icon
                name="fluent:settings-20-filled"
                class="text-white"
                size="2em"
              />
            </div>
            <h3 class="text-xl font-semibold text-slate-800 mb-2">勤務設定</h3>
            <p class="text-slate-600 mb-4">勤務時間や休憩時間の設定を管理</p>
            <div
              class="flex items-center justify-center text-green-600 font-medium group-hover:text-green-700"
            >
              設定を開く
              <Icon
                name="fluent:arrow-right-20-filled"
                class="ml-2"
                size="1.1em"
              />
            </div>
          </div>
        </NuxtLink>

        <!-- App Information Card -->
        <NuxtLink
          :to="{
            name: 'id-settings-about',
            params: { id: user?.prop.id },
          }"
          class="bg-white rounded-xl shadow-lg border border-slate-200 hover:shadow-xl hover:border-slate-300 transition-all duration-300 transform hover:-translate-y-1 p-6 group"
        >
          <div class="text-center">
            <div
              class="w-16 h-16 mx-auto mb-4 bg-gradient-to-br from-slate-500 to-slate-600 rounded-full flex items-center justify-center group-hover:scale-110 transition-transform duration-200"
            >
              <Icon
                name="fluent:info-20-filled"
                class="text-white"
                size="2em"
              />
            </div>
            <h3 class="text-xl font-semibold text-slate-800 mb-2">
              アプリについて
            </h3>
            <p class="text-slate-600 mb-4">バージョン情報やライセンス情報</p>
            <div
              class="flex items-center justify-center text-slate-600 font-medium group-hover:text-slate-700"
            >
              詳細を見る
              <Icon
                name="fluent:arrow-right-20-filled"
                class="ml-2"
                size="1.1em"
              />
            </div>
          </div>
        </NuxtLink>
      </div>

      <!-- Quick Actions -->
      <div class="mt-12">
        <h2 class="text-xl font-semibold text-slate-800 mb-6">
          クイックアクション
        </h2>
        <div class="bg-white rounded-xl shadow-lg border border-slate-200 p-6">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <!-- Export Data -->
            <button
              class="flex items-center p-4 rounded-lg bg-slate-50 hover:bg-slate-100 transition-colors duration-200 group"
            >
              <div
                class="w-12 h-12 bg-blue-100 rounded-lg flex items-center justify-center mr-4 group-hover:bg-blue-200 transition-colors duration-200"
              >
                <Icon
                  name="fluent:document-arrow-down-20-filled"
                  class="text-blue-600"
                  size="1.5em"
                />
              </div>
              <div class="text-left">
                <h4 class="font-semibold text-slate-800">
                  データをエクスポート
                </h4>
                <p class="text-sm text-slate-600">勤務データをファイルに出力</p>
              </div>
            </button>

            <!-- Import Data -->
            <button
              class="flex items-center p-4 rounded-lg bg-slate-50 hover:bg-slate-100 transition-colors duration-200 group"
            >
              <div
                class="w-12 h-12 bg-green-100 rounded-lg flex items-center justify-center mr-4 group-hover:bg-green-200 transition-colors duration-200"
              >
                <Icon
                  name="fluent:document-arrow-up-20-filled"
                  class="text-green-600"
                  size="1.5em"
                />
              </div>
              <div class="text-left">
                <h4 class="font-semibold text-slate-800">データをインポート</h4>
                <p class="text-sm text-slate-600">バックアップから復元</p>
              </div>
            </button>
          </div>
        </div>
      </div>

      <!-- Back to Home -->
      <div class="mt-8 text-center">
        <NuxtLink
          :to="{ name: 'id', params: { id: user?.prop.id } }"
          class="inline-flex items-center px-6 py-3 bg-gradient-to-r from-slate-600 to-slate-700 hover:from-slate-700 hover:to-slate-800 text-white rounded-lg font-medium transition-all duration-200 shadow-lg hover:shadow-xl transform hover:-translate-y-0.5"
        >
          <Icon name="fluent:arrow-left-20-filled" class="mr-2" size="1.2em" />
          ホームに戻る
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { useRoute } from "vue-router";
import Loading from "~/components/Loading.vue";
import { UserData } from "~/models/user";
import { UserRepository } from "~/repositories/tauri-commands/user";

export default defineComponent({
  components: {
    Loading,
  },
  data() {
    return {
      user: undefined as UserData | undefined,
      isLoading: true,
    };
  },
  async mounted() {
    const route = useRoute();
    const userId = route.params.id as string;
    this.user = new UserData(await UserRepository.getById(parseInt(userId)));

    this.isLoading = false;
  },
  methods: {},
});
</script>
