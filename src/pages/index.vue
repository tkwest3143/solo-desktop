<template>
  <main class="text-basic-500 p-4 text-center">
    <div>
      <h1 class="text-2xl font-semibold">ユーザを選択してください</h1>
      <div class="flex justify-end mt-4"></div>
      <div
        class="mt-4 grid grid-cols-4 gap-4 w-full border bg-basic-50 p-4 rounded"
      >
        <div
          v-for="user in users"
          :key="user.prop.id"
          class="cursor-pointer items-center justify-center flex bg-primary-200 w-full h-48 rounded shadow hover:shadow-lg hover:bg-primary-300"
          @click="onClickUser(user)"
        >
          <div>
            <div class="items-center justify-center flex">
              <div class="w-24 h-24 border-2 border-primary-700 rounded-full">
                <Icon
                  name="material-symbols:person-edit-rounded"
                  class="bg-primary-700 w-20 h-20"
                />
              </div>
            </div>

            {{ user.prop.name }} ({{ user.prop.email }})
            <div class="text-xs">
              最終アクセス日：{{ user.lastLoginTimeText }}
            </div>
          </div>
        </div>
        <NuxtLink
          to="/addUser"
          class="items-center justify-center flex text-basic-500 hover:bg-basic-100 py-2 px-4 border-2 rounded shadow hover:shadow-lg text-xl"
        >
          <div>
            <div
              class="w-full h-24 border-2 border-dashed border-basic-300 items-center justify-center flex rounded-lg"
            >
              <Icon
                name="fluent:add-20-filled"
                class="bg-basic-400"
                size="1.5em"
              />
            </div>
            ユーザを追加する
          </div>
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
