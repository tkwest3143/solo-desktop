<template>
  <Loading v-if="isLoading" />
  <div v-else>
    <div class="text-center text-4xl mt-5">{{ user?.prop.name }} さん</div>
    <div class="w-full mt-4 grid justify-items-center">
      <div class="flex">
        <div v-if="!isWorking() && !isResting()">
          <Icon name="fluent:sleep-20-filled" style="color: gray" size="2em" />
        </div>
        <div v-else-if="isWorking()">
          <Icon
            name="fluent:people-team-20-filled"
            style="color: aqua"
            size="2em"
          />
        </div>
        <div v-else-if="isResting()">
          <Icon
            name="fluent:drink-coffee-20-filled"
            style="color: aqua"
            size="2em"
          />
        </div>
        <div class="text-2xl">
          {{ isWorking() ? "勤務中" : isResting() ? "休憩中" : "未出勤" }}
        </div>
      </div>
    </div>
    <div class="user-detail-container divide-x divide-solid">
      <div class="left-panel flex justify-center items-center">
        <div class="text-center">
          <div class="text-4xl">{{ currentDate }} {{ currentDay }}</div>
          <div class="text-7xl">{{ currentTime }}</div>
        </div>
      </div>
      <div class="right-panel">
        <div class="button-group ml-4">
          <button
            class="bg-sky-300 enabled:hover:bg-sky-500 disabled:opacity-25"
            @click="startWork"
            :disabled="isWorking() || isResting()"
          >
            <div class="mr-2 w-1/3">
              <img src="~/assets/icons/attendance.png" width="48" />
            </div>
            <div class="w-2/3 text-xl">出勤</div>
          </button>
          <button
            class="bg-rose-300 enabled:hover:bg-rose-500 disabled:opacity-25"
            @click="endWork"
            :disabled="!isWorking()"
          >
            <div class="mr-2 w-1/3">
              <img src="~/assets/icons/go-home.png" width="48" />
            </div>
            <div class="w-2/3 text-xl">退勤</div>
          </button>
          <button
            class="bg-lime-300 enabled:hover:bg-lime-500 disabled:opacity-25"
            @click="startRest"
            :disabled="isResting() || isNone()"
          >
            <div class="mr-2 w-1/3">
              <img src="~/assets/icons/break-time.png" width="48" />
            </div>
            <div class="w-2/3 text-xl">休憩開始</div>
          </button>
          <button
            class="bg-amber-300 enabled:hover:bg-amber-500 disabled:opacity-25"
            @click="endRest"
            :disabled="!isResting()"
          >
            <div class="mr-2 w-1/3">
              <img
                src="~/assets/icons/back-office.png"
                class="mr-2"
                width="48"
              />
            </div>
            <div class="w-2/3 text-xl">休憩終了</div>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.user-detail-container {
  display: flex;
  justify-content: space-between;
  max-width: 800px;
  margin: 50px auto;
  padding: 20px;
  background-color: #f8f9fa;
  border-radius: 10px;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
}

.left-panel {
  flex: 1;
  text-align: left;
}

.right-panel {
  flex: 1;
  display: flex;
  justify-content: center;
  align-items: center;
}

h1 {
  margin-bottom: 20px;
  color: #343a40;
  font-size: 24px;
  font-weight: bold;
}

p {
  color: #495057;
  font-size: 16px;
}

.button-group {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 10px;
}

button {
  padding: 10px 20px;
  border: none;
  cursor: pointer;
  border-radius: 5px;
  transition: background-color 0.3s;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 16px;
}

button .nuxt-icon {
  margin-right: 5px;
}
</style>

<script lang="ts">
import { formatDate } from "date-fns";
import { defineComponent } from "vue";
import { useRoute } from "vue-router";
import Loading from "~/components/Loading.vue";
import { UserData } from "~/models/user";
import { UserRepository } from "~/repositories/tauri-commands/user";
import { WorkTimeRepository } from "~/repositories/tauri-commands/workTime";

enum Working {
  working = "working",
  resting = "resting",
  none = "none",
}
export default defineComponent({
  components: {
    Loading,
  },
  data() {
    return {
      user: undefined as UserData | undefined,
      currentDate: new Date().toLocaleDateString(),
      currentDay: new Date().toLocaleDateString("ja-JP", { weekday: "long" }),
      currentTime: new Date().toLocaleTimeString(),
      working: Working.none,
      isLoading: true,
    };
  },
  async mounted() {
    const route = useRoute();
    const userId = route.params.id as string;
    this.user = new UserData(await UserRepository.getById(parseInt(userId)));
    setInterval(() => {
      this.currentTime = new Date().toLocaleTimeString();
    }, 1000);
    this.isLoading = false;
  },
  methods: {
    isWorking() {
      return this.working === Working.working;
    },
    isResting() {
      return this.working === Working.resting;
    },
    isNone() {
      return this.working === Working.none;
    },
    async startWork() {
      this.working = Working.working;
      await WorkTimeRepository.create({
        userId: this.user!.prop.id,
        targetDay: formatDate(new Date(), "yyyy-MM-dd"),
        start: new Date(),
      });
    },
    async endWork() {
      this.working = Working.none;
      await WorkTimeRepository.update({
        userId: this.user!.prop.id,
        targetDay: formatDate(new Date(), "yyyy-MM-dd"),
        end: new Date(),
      });
    },
    async startRest() {
      this.working = Working.resting;
      await WorkTimeRepository.update({
        userId: this.user!.prop.id,
        targetDay: formatDate(new Date(), "yyyy-MM-dd"),
        restStart: new Date(),
      });
    },
    async endRest() {
      this.working = Working.working;
      await WorkTimeRepository.update({
        userId: this.user!.prop.id,
        targetDay: formatDate(new Date(), "yyyy-MM-dd"),
        restEnd: new Date(),
      });
    },
  },
});
</script>
