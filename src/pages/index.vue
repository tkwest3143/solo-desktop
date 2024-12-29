<template>
  <Header />
  <main>
    <div v-if="users.length">
      <h2>ユーザ情報選択</h2>
      <div class="user-list">
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
    <div v-else>
      <p>ユーザ情報がありません。</p>
    </div>
  </main>
</template>

<script lang="ts">
import Header from "~/components/Header.vue";
import { UserData, type user } from "~/models/user";
import { UserRepository } from "~/repositories/tauri-commands/user";

export default {
  components: {
    Header,
  },
  data() {
    return {
      users: [] as UserData[],
    };
  },
  async mounted() {
    await this.loadUsers();
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
