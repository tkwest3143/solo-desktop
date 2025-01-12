<template>
  <div>
    <NuxtLayout>
      <div v-if="isLoading">
        <Loading loading-text="データ取り込み中" />
      </div>
      <div v-else class="h-screen">
        <div style="position: fixed; top: 0; width: 100%; z-index: 1000">
          <Header :userData="userData" />
        </div>
        <div style="padding-top: 60px">
          <!-- Adjust padding-top based on Header height -->
          <NuxtPage />
        </div>
      </div>
    </NuxtLayout>
  </div>
</template>

<script lang="ts">
import { defineComponent, onMounted, ref, watch } from "vue";
import { useRoute } from "vue-router";
import Header from "./components/Header.vue";
import { UserData } from "./models/user";
import { JapaneseHolidayRepository } from "./repositories/tauri-commands/japaneseHoliday";
import { UserRepository } from "./repositories/tauri-commands/user";

export default defineComponent({
  components: {
    Header,
  },
  setup() {
    const route = useRoute(); // ルート情報を取得
    const isLoading = ref(true); // ローディング状態
    const userData = ref<UserData | undefined>(undefined); // ユーザーデータ

    // ユーザーデータを取得する関数
    const fetchUserData = async () => {
      const userId = route.params.id as string;
      if (userId) {
        userData.value = new UserData(
          await UserRepository.getById(parseInt(userId))
        );
      } else {
        userData.value = undefined;
      }
    };

    // 初期ロード時にデータ取得
    onMounted(async () => {
      const holiday = await JapaneseHolidayRepository.get(
        new Date().getFullYear().toString()
      );
      if (holiday.length === 0) {
        await JapaneseHolidayRepository.import();
      }
      isLoading.value = false;

      // 初回データ取得
      await fetchUserData();
    });

    // ルートの変更を監視してfetchUserDataを再実行
    watch(
      () => route.fullPath,
      async () => {
        await fetchUserData();
      }
    );

    // テンプレートに返す
    return {
      isLoading,
      userData,
    };
  },
});
</script>
