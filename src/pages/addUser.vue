<template>
  <div class="flex flex-col items-center">
    <h1 class="font-bold text-2xl">ユーザ追加</h1>
    <form @submit.prevent="addUser" class="w-96 mt-4">
      <CommonInput id="name" label="名前" v-model.trim="name" required />
      <CommonInput
        id="email"
        label="メール"
        type="email"
        v-model.trim="email"
        required
      />
      <div class="flex justify-end w-full mt-4 space-x-1">
        <button type="button" class="bg-basic-200 w-32 p-1 rounded-lg">
          キャンセル
        </button>
        <button type="submit" class="bg-primary-200 w-32 p-1 rounded-lg">
          追加
        </button>
      </div>
    </form>
  </div>
</template>

<script lang="ts">
import CommonInput from "~/components/CommonInput.vue";
import { UserRepository } from "~/repositories/tauri-commands/user";

export default {
  components: {
    CommonInput,
  },
  data() {
    return {
      name: "",
      email: "",
    };
  },
  methods: {
    async addUser() {
      await UserRepository.create({
        name: this.name,
        email: this.email,
      });
      this.name = "";
      this.email = "";
      this.$router.push("/");
    },
  },
};
</script>
