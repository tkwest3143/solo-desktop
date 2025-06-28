<template>
  <div>
    <div v-if="isLoading" class="h-screen flex items-center justify-center">
      <Loading loading-text="データ取り込み中" />
    </div>
    <div v-else>
      <div style="position: fixed; top: 0; width: 100%; z-index: 10">
        <Header :userData="userData" />
      </div>
      <div style="padding-top: 60px; overflow-y: auto">
        <slot />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, onMounted, ref, watch } from "vue";
import { useRoute } from "vue-router";
import Header from "~/components/Header.vue";
import Loading from "~/components/Loading.vue";
import { UserData } from "~/models/user";
import { JapaneseHolidayRepository } from "~/repositories/tauri-commands/japaneseHoliday";
import { UserRepository } from "~/repositories/tauri-commands/user";

export default defineComponent({
  components: {
    Header,
    Loading,
  },
  setup() {
    const route = useRoute();
    const isLoading = ref(true);
    const userData = ref<UserData | undefined>(undefined);

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

    onMounted(async () => {
      const holiday = await JapaneseHolidayRepository.get(
        new Date().getFullYear().toString()
      );
      if (holiday.length === 0) {
        await JapaneseHolidayRepository.import();
      }
      isLoading.value = false;
      await fetchUserData();
    });

    watch(
      () => route.fullPath,
      async () => {
        await fetchUserData();
      }
    );

    return {
      isLoading,
      userData,
    };
  },
});
</script>
