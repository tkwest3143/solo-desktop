<template>
  <main class="min-h-screen bg-gradient-to-br from-slate-50 to-blue-50">
    <div class="container mx-auto px-6 py-12">
      <!-- Page Header -->
      <div class="text-center mb-12">
        <div class="flex items-center justify-center mb-4">
          <Icon 
            name="fluent:organization-20-filled" 
            class="text-blue-600 mr-3" 
            size="3em" 
          />
          <h1 class="text-4xl font-bold text-slate-800">Solo Desktop</h1>
        </div>
        <p class="text-xl text-slate-600">ユーザーを選択してログインしてください</p>
      </div>

      <!-- User Grid -->
      <div class="max-w-6xl mx-auto">
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
          <!-- User Cards -->
          <div
            v-for="user in users"
            :key="user.prop.id"
            @click="onClickUser(user)"
            class="bg-white rounded-xl shadow-lg border border-slate-200 hover:shadow-xl hover:border-blue-300 cursor-pointer transition-all duration-300 transform hover:-translate-y-1 p-6"
          >
            <div class="text-center">
              <!-- User Avatar -->
              <div class="mb-4 flex justify-center">
                <div class="w-20 h-20 bg-gradient-to-br from-blue-500 to-blue-600 rounded-full flex items-center justify-center shadow-lg">
                  <Icon
                    name="fluent:person-20-filled"
                    class="text-white"
                    size="2.5em"
                  />
                </div>
              </div>
              
              <!-- User Info -->
              <h3 class="text-lg font-semibold text-slate-800 mb-1">{{ user.prop.name }}</h3>
              <p class="text-sm text-slate-600 mb-3">{{ user.prop.email }}</p>
              
              <!-- Last Login -->
              <div class="text-xs text-slate-500 bg-slate-50 rounded-lg p-2">
                <Icon name="fluent:clock-20-filled" class="inline mr-1" size="0.9em" />
                最終アクセス：{{ user.lastLoginTimeText }}
              </div>
            </div>
          </div>

          <!-- Add User Card -->
          <NuxtLink
            to="/addUser"
            class="bg-white rounded-xl shadow-lg border-2 border-dashed border-slate-300 hover:border-blue-400 hover:shadow-xl cursor-pointer transition-all duration-300 transform hover:-translate-y-1 p-6 flex flex-col items-center justify-center text-center group"
          >
            <div class="w-20 h-20 bg-gradient-to-br from-gray-400 to-gray-500 rounded-full flex items-center justify-center shadow-lg mb-4 group-hover:scale-110 transition-transform duration-200">
              <Icon
                name="fluent:person-add-20-filled"
                class="text-white"
                size="2.5em"
              />
            </div>
            <h3 class="text-lg font-semibold text-slate-700 group-hover:text-slate-900">新しいユーザー</h3>
            <p class="text-sm text-slate-500">ユーザーを追加</p>
          </NuxtLink>
        </div>

        <!-- Empty State -->
        <div v-if="users.length === 0" class="text-center py-12">
          <Icon name="fluent:people-20-filled" class="text-slate-300 mb-4" size="4em" />
          <h3 class="text-xl font-semibold text-slate-600 mb-2">ユーザーがいません</h3>
          <p class="text-slate-500 mb-6">最初のユーザーを作成してください</p>
          <NuxtLink
            to="/addUser"
            class="inline-flex items-center px-6 py-3 bg-gradient-to-r from-blue-500 to-blue-600 hover:from-blue-600 hover:to-blue-700 text-white rounded-lg font-medium transition-all duration-200 shadow-lg hover:shadow-xl transform hover:-translate-y-0.5"
          >
            <Icon name="fluent:person-add-20-filled" class="mr-2" size="1.2em" />
            ユーザーを追加
          </NuxtLink>
        </div>
      </div>
    </div>
  </main>
</template>

<script lang="ts">
import { UserData, type user } from "~/models/user";
import { UserRepository } from "~/repositories/tauri-commands/user";

export default {
  data() {
    return {
      users: [] as UserData[],
    };
  },
  async mounted() {
    await this.loadUsers();
  },
  methods: {
    async onClickUser(user: UserData) {
      await UserRepository.update({
        id: user.prop.id,
        lastLoginTime: new Date(),
      });
      this.$router.push({ name: "id", params: { id: user.prop.id } });
    },
    async loadUsers() {
      const users = await UserRepository.getAll();
      this.users = users.map((user: user) => new UserData(user));
    },
  },
};
</script>
