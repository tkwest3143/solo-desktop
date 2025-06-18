<template>
  <div
    v-if="isLoading"
    class="w-screen h-screen bg-gray-800 bg-opacity-75 flex items-center justify-center z-50"
  >
    <Loading />
  </div>
  <div>
    <div class="max-w-2xl mx-auto p-6 bg-white rounded-lg shadow-md mt-6">
      <h2 class="text-2xl font-bold mb-4">勤務設定を追加</h2>
      <form @submit.prevent="submitForm">
        <CommonInput
          id="title"
          label="タイトル"
          v-model="form.title"
          placeholder="タイトル"
        />
        <div class="mb-4 grid grid-cols-5 gap-2">
          <CommonTimeInput
            id="startTime"
            label="開始時間"
            v-model="form.startTime"
          />
          <CommonTimeInput
            id="endTime"
            label="終了時間"
            v-model="form.endTime"
          />
          <CommonTimeInput
            id="breakStartTime"
            label="休憩開始時間"
            v-model="form.breakStartTime"
          />
          <CommonTimeInput
            id="breakEndTime"
            label="休憩終了時間"
            v-model="form.breakEndTime"
          />
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
        <CommonTextarea
          id="memo"
          label="メモ"
          v-model="form.memo"
          placeholder="メモ"
        />
        <div class="mb-4">
          <div
            :class="{
              'bg-primary-200 text-gray-500': form.isDefaultWorkSetting,
              'bg-gray-200 text-gray-700': !form.isDefaultWorkSetting,
            }"
            class="py-2 px-4 rounded shadow-outline"
          >
            <CommonCheckbox
              v-model="form.isDefaultWorkSetting"
              label="デフォルトの勤務時間として登録する"
              icon-name="fluent:checkmark-20-filled"
            />
          </div>
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
import CommonCheckbox from "~/components/CommonCheckbox.vue";
import CommonInput from "~/components/CommonInput.vue";
import CommonTextarea from "~/components/CommonTextarea.vue";
import CommonTimeInput from "~/components/CommonTimeInput.vue";
import Loading from "~/components/Loading.vue";
import { UserData } from "~/models/user";
import { workSettingData } from "~/models/workSetting";
import { UserRepository } from "~/repositories/tauri-commands/user";
import { WorkSettingRepository } from "~/repositories/tauri-commands/workTimeSetting";

export default defineComponent({
  components: {
    Loading,
    CommonCheckbox,
    CommonInput,
    CommonTextarea,
    CommonTimeInput,
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
      isLoading: false,
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
      this.isLoading = true;
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
      this.isLoading = false;
    },
  },
});
</script>
