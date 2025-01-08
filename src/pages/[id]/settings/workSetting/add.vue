<template>
  <div>
    <Header />
    <div class="max-w-2xl mx-auto p-6 bg-white rounded-lg shadow-md mt-6">
      <h2 class="text-2xl font-bold mb-4">勤務設定を追加</h2>
      <form @submit.prevent="submitForm">
        <div class="mb-4">
          <label class="block text-gray-700 text-sm font-bold mb-2" for="title"
            >タイトル</label
          >
          <input
            v-model="form.title"
            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
            id="title"
            type="text"
            placeholder="タイトル"
          />
        </div>
        <div class="mb-4 grid grid-cols-5 gap-2">
          <div>
            <label
              class="block text-gray-700 text-sm font-bold mb-2"
              for="startTime"
              >開始時間</label
            >
            <input
              v-model="form.startTime"
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              id="startTime"
              type="time"
            />
          </div>
          <div>
            <label
              class="block text-gray-700 text-sm font-bold mb-2"
              for="endTime"
              >終了時間</label
            >
            <input
              v-model="form.endTime"
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              id="endTime"
              type="time"
            />
          </div>
          <div>
            <label
              class="block text-gray-700 text-sm font-bold mb-2"
              for="breakStartTime"
              >休憩開始時間</label
            >
            <input
              v-model="form.breakStartTime"
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              id="breakStartTime"
              type="time"
            />
          </div>
          <div>
            <label
              class="block text-gray-700 text-sm font-bold mb-2"
              for="breakEndTime"
              >休憩終了時間</label
            >
            <input
              v-model="form.breakEndTime"
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              id="breakEndTime"
              type="time"
            />
          </div>
          <div>
            <label
              class="block text-gray-700 text-sm font-bold mb-2"
              for="workUnit"
              >勤務時間単位</label
            >
            <input
              v-model="form.workUnit"
              class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
              id="workUnit"
              type="number"
              min="1"
              max="99"
              placeholder="勤務時間単位"
            />
          </div>
        </div>
        <div class="mb-4">
          <label class="block text-gray-700 text-sm font-bold mb-2" for="memo"
            >メモ</label
          >
          <textarea
            v-model="form.memo"
            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
            id="memo"
            placeholder="メモ"
          ></textarea>
        </div>
        <div class="mb-4">
          <button
            @click="form.isDefaultWorkSetting = !form.isDefaultWorkSetting"
            :class="{
              'bg-sky-500 text-white': form.isDefaultWorkSetting,
              'bg-gray-200 text-gray-700': !form.isDefaultWorkSetting,
            }"
            class="flex items-left justify-start py-2 px-4 rounded shadow-outline"
            type="button"
          >
            <div
              class="flex items-center justify-center w-6 h-6 border-2 border-gray-400 rounded mr-2"
              :class="{
                'bg-sky-500 border-blue-500': form.isDefaultWorkSetting,
                'bg-white': !form.isDefaultWorkSetting,
              }"
            >
              <Icon
                name="fluent:checkmark-20-filled"
                v-if="form.isDefaultWorkSetting"
                style="color: greenyellow"
                size="2em"
              />
            </div>
            <span>デフォルトの勤務時間として登録する</span>
          </button>
        </div>
        <div class="flex items-center justify-between">
          <button
            class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
            type="submit"
          >
            保存
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { useRoute } from "vue-router";
import Header from "~/components/Header.vue";
import { UserData } from "~/models/user";
import { workSettingData } from "~/models/workSetting";
import { UserRepository } from "~/repositories/tauri-commands/user";
import { WorkSettingRepository } from "~/repositories/tauri-commands/workTimeSetting";

export default defineComponent({
  components: {
    Header,
  },
  data() {
    return {
      user: undefined as UserData | undefined,
      workSettings: [] as workSettingData[],
      form: {
        title: "",
        startTime: "",
        endTime: "",
        breakStartTime: "",
        breakEndTime: "",
        workUnit: 0,
        memo: "",
        isDefaultWorkSetting: false,
      },
    };
  },
  async mounted() {
    await this.init();
  },
  methods: {
    async init() {
      const route = useRoute();
      const userId = route.params.id as string;
      this.user = new UserData(await UserRepository.getById(parseInt(userId)));
    },
    async submitForm() {
      // フォーム送信処理を実装
      if (!this.user) return;
      await WorkSettingRepository.create({
        userId: this.user.prop.id,
        title: this.form.title,
        start: new Date(`2024-01-01T${this.form.startTime}`),
        end: new Date(`2024-01-01T${this.form.endTime}`),
        restStart: new Date(`2024-01-01T${this.form.breakStartTime}`),
        restEnd: new Date(`2024-01-01T${this.form.breakEndTime}`),
        workingUnit: this.form.workUnit,
        memo: this.form.memo,
        isDefault: this.form.isDefaultWorkSetting,
      });
      // フォームをリセット
      this.form = {
        title: "",
        startTime: "",
        endTime: "",
        breakStartTime: "",
        breakEndTime: "",
        workUnit: 0,
        memo: "",
        isDefaultWorkSetting: false,
      };
    },
  },
});
</script>

<style scoped>
/* 追加のスタイルが必要な場合はここに記述 */
</style>
