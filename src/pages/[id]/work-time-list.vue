<template>
  <Loading v-if="isLoading" />
  <div v-else>
    <Header />
  </div>
</template>

<script lang="ts">
import { formatDate } from "date-fns";
import { defineComponent } from "vue";
import { useRoute } from "vue-router";
import Header from "~/components/Header.vue";
import Loading from "~/components/Loading.vue";
import { UserData } from "~/models/user";
import { UserRepository } from "~/repositories/tauri-commands/user";
import { WorkTimeRepository } from "~/repositories/tauri-commands/workTime";

export default defineComponent({
  components: {
    Header,
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
    WorkTimeRepository.
    this.isLoading = false;
  },
  methods: {
    
  },
});
</script>
