<template>
  <main>
    <div v-if="users.length">
      <h1 class="text-2xl font-semibold">ユーザを選択してください</h1>
      <div class="flex justify-end mt-4">
        <NuxtLink
          to="/addUser"
          class="bg-indigo-500 hover:bg-indigo-700 text-white font-bold py-2 px-4 rounded"
        >
          ユーザを追加する
        </NuxtLink>
      </div>
      <div class="user-list mt-4">
        <NuxtLink
          v-for="user in users"
          :key="user.prop.id"
          class="user-button w-full"
          :to="{ name: 'id', params: { id: user.prop.id } }"
        >
          {{ user.prop.name }} ({{ user.prop.email }})
          <div>最終アクセス日：{{ user.lastLoginTimeText }}</div>
        </NuxtLink>
      </div>
    </div>
    <div
      v-else
      class="flex items-center justify-center h-[40rem] w-full flex-col"
    >
      <div class="w-full mb-6 font-bold text-gray-500 text-xl">
        ユーザ情報が未登録です。
      </div>
      <div class="w-full mb-6 font-bold text-gray-500 text-xl">
        ユーザを追加するボタンからユーザを追加してください。
      </div>
      <div class="w-full">
        <NuxtLink
          to="/addUser"
          class="bg-indigo-500 hover:bg-indigo-700 text-white font-semibold py-2 px-4 rounded shadow hover:shadow-lg text-xl"
        >
          ユーザを追加する
        </NuxtLink>
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

    console.log(this.users);
  },
  methods: {
    async loadUsers() {
      const users = await UserRepository.getAll();
      this.users = users.map((user: user) => new UserData(user));
    },
  },
};
</script>

<style scoped>
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
