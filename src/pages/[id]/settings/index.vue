<template>
  <Loading v-if="isLoading" />
  <div v-else>
    <div class="text-center text-2xl mt-5">
      {{ user?.prop.name ?? "" }} さん
    </div>
    <div class="mt-3">
      <div class="flex flex-col items-center">
        <div class="text-xl font-semibold">{{ user?.prop.email ?? "" }}</div>
      </div>
      <div class="mt-8 space-y-4">
        <NuxtLink
          :to="{
            name: 'id-settings-userSetting',
            params: { id: user?.prop.id },
          }"
          class="w-full py-2 px-4 bg-green-500 text-white rounded-lg shadow-md hover:bg-green-600"
        >
          ユーザ設定
        </NuxtLink>
        <NuxtLink
          :to="{
            name: 'id-settings-workSetting',
            params: { id: user?.prop.id },
          }"
          class="w-full py-2 px-4 bg-green-500 text-white rounded-lg shadow-md hover:bg-green-600"
        >
          勤務設定
        </NuxtLink>
        <NuxtLink
          :to="{
            name: 'id-settings-userSetting',
            params: { id: user?.prop.id },
          }"
          class="w-full py-2 px-4 bg-gray-500 text-white rounded-lg shadow-md hover:bg-gray-600"
        >
          このアプリについて
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
