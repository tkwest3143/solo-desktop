<template>
  <Loading v-if="isLoading" />
  <div v-else>
    <div class="overflow-x-auto m-2 mb-20">
      <div class="flex justify-between items-center mb-4">
        <div class="flex space-x-2">
          <button
            @click="previousMonth"
            class="px-4 py-2 rounded hover:bg-basic-200 flex items-center"
          >
            <Icon name="material-symbols:arrow-back-ios" size="1em" />
          </button>
          <button
            @click="nextMonth"
            class="px-4 py-2 rounded hover:bg-basic-200 flex items-center"
          >
            <Icon name="material-symbols:arrow-forward-ios" size="1em" />
          </button>
          <h2 class="text-xl font-semibold flex items-center">
            {{ selectedMonth.yearText }}年{{
              selectedMonth.monthText
            }}月の勤務時間
          </h2>
          <button
            @click="thisMonth"
            class="px-4 py-1 rounded hover:bg-basic-200 flex items-center border border-basic-200 rounded-full"
          >
            今月
          </button>
        </div>
      </div>
      <div class="mb-4">
        <div class="flex items-center space-x-2">
          <CommonSelect
            class="w-1/3"
            id="workSetting"
            label="勤務設定"
            v-model="selectedWorkSettingId"
            :options="
              workSettings.map((setting) => ({
                value: setting.prop.id,
                text: setting.prop.title,
              }))
            "
          />
          <button
            v-if="selectedWorkSettingId"
            class="px-4 py-2 bg-primary-400 text-basic-0 rounded hover:bg-primary-500"
          >
            設定を編集する
          </button>
          <NuxtLink
            v-else
            class="px-4 py-2 bg-primary-400 text-basic-0 rounded hover:bg-primary-500"
            :to="{
              name: 'id-settings-workSetting-add',
              params: { id: user?.prop.id },
            }"
          >
            設定を登録する
          </NuxtLink>
        </div>

        <div class="mb-4 divide-x divide-solid space-x-1">
          <label class="text-basic-700 text-sm font-bold">
            開始時間：{{ getSelectedWorkSetting()?.startByText ?? " - " }}
          </label>
          <label class="text-basic-700 text-sm font-bold">
            終了時間：{{ getSelectedWorkSetting()?.endByText ?? " - " }}
          </label>
          <label class="text-basic-700 text-sm font-bold">
            休憩開始時間：{{
              getSelectedWorkSetting()?.restStartByText ?? " - "
            }}
          </label>
          <label class="text-basic-700 text-sm font-bold">
            休憩終了時間：{{ getSelectedWorkSetting()?.restEndByText ?? " - " }}
          </label>
          <label class="text-basic-700 text-sm font-bold">
            勤務時間単位：{{
              getSelectedWorkSetting()?.prop.working_unit ?? " - "
            }}分
          </label>
        </div>
      </div>
      <div class="flex justify-end mb-2">
        <div class="flex items-center space-x-2">
          <span class="text-md font-semibold text-basic-700">
            今月の勤務時間:
          </span>
          <span class="text-md font-semibold text-basic-800">
            {{ totalWorkTime() }}
          </span>
        </div>
      </div>
      <table class="min-w-full border-collapse mb-4">
        <thead class="bg-basic-100">
          <tr>
            <th
              scope="col"
              class="w-1/12 py-3 text-center text-xs font-medium text-basic-500 border border-table-border"
            ></th>
            <th
              scope="col"
              class="w-2/12 py-3 text-center text-xs font-medium text-basic-500 border border-table-border"
            >
              日付
            </th>
            <th
              scope="col"
              class="w-1/12 py-3 text-center text-xs font-medium text-basic-500 border border-table-border"
            >
              稼働状況
            </th>
            <th
              scope="col"
              class="w-1/12 py-3 text-center text-xs font-medium text-basic-500 border border-table-border"
            >
              開始時間
            </th>
            <th
              scope="col"
              class="w-1/12 py-3 text-center text-xs font-medium text-basic-500 border border-table-border"
            >
              終了時間
            </th>
            <th
              scope="col"
              class="w-1/12 py-3 text-center text-xs font-medium text-basic-500 border border-table-border"
            >
              休憩開始
            </th>
            <th
              scope="col"
              class="w-1/12 py-3 text-center text-xs font-medium text-basic-500 border border-table-border"
            >
              休憩終了
            </th>
            <th
              scope="col"
              class="w-1/12 py-3 text-center text-xs font-medium text-basic-500 border border-table-border"
            >
              休憩時間
            </th>
            <th
              scope="col"
              class="w-1/12 py-3 text-center text-xs font-medium text-basic-500 border border-table-border"
            >
              業務時間
            </th>
            <th
              scope="col"
              class="w-1/2 py-3 text-center text-xs font-medium text-basic-500 border border-table-border w-auto"
            >
              備考
            </th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="(workTime, index) in selectedMonth.monthWorkTimes"
            :key="index"
            :class="{
              'bg-primary-100':
                editingWorkTime?.prop.target_day === workTime.prop.target_day,
            }"
          >
            <td
              class="py-2 text-center text-sm text-basic-900 border border-table-border"
            >
              <button
                :disabled="selectedWorkSettingId === 0"
                @click="setDefaultWorkTime(workTime)"
                class="disabled:opacity-25 px-2 py-1 text-white bg-primary-400 enabled:hover:bg-primary-500 text-xs rounded-lg"
              >
                デフォルト
              </button>
            </td>
            <td
              class="py-2 text-center text-sm text-basic-900 border border-table-border"
            >
              {{ workTime.getDayTextWithWeek(japaneseHolidays) }}
            </td>
            <td
              class="py-2 text-center text-sm text-basic-900 border border-table-border"
            >
              <div
                v-if="
                  workTimes.some(
                    (wt) => wt.prop.target_day === workTime.prop.target_day
                  )
                "
              >
                稼働
              </div>
              <div v-else>非稼働</div>
            </td>
            <td
              @dblclick="editWorkTime(workTime)"
              class="py-2 text-center text-sm text-basic-900 border border-table-border"
            >
              <div
                v-if="
                  editingWorkTime?.prop.target_day !== workTime.prop.target_day
                "
              >
                {{ workTime.startByText }}
              </div>
              <div v-else>
                <input
                  type="time"
                  :value="workTime.startByText"
                  @change="
                    changeWorkTime(workTime, {
                      start: ($event.target as HTMLInputElement).value,
                    })
                  "
                  class="border border-basic-300 rounded"
                />
              </div>
            </td>
            <td
              @dblclick="editWorkTime(workTime)"
              class="py-2 text-center text-sm text-basic-900 border border-table-border"
            >
              <div
                v-if="
                  editingWorkTime?.prop.target_day !== workTime.prop.target_day
                "
              >
                {{ workTime.endByText }}
              </div>
              <div v-else>
                <input
                  type="time"
                  :value="workTime.endByText"
                  @change="
                    changeWorkTime(workTime, {
                      end: ($event.target as HTMLInputElement).value,
                    })
                  "
                  class="border border-basic-300 rounded"
                />
              </div>
            </td>
            <td
              @dblclick="editWorkTime(workTime)"
              class="py-2 text-center text-sm text-basic-900 border border-table-border"
            >
              <div
                v-if="
                  editingWorkTime?.prop.target_day !== workTime.prop.target_day
                "
              >
                {{ workTime.restStartByText }}
              </div>
              <div v-else>
                <input
                  type="time"
                  :value="workTime.restStartByText"
                  @change="
                    changeWorkTime(workTime, {
                      restStart: ($event.target as HTMLInputElement).value,
                    })
                  "
                  class="border border-basic-300 rounded"
                />
              </div>
            </td>
            <td
              @dblclick="editWorkTime(workTime)"
              class="py-2 text-center text-sm text-basic-900 border border-table-border"
            >
              <div
                v-if="
                  editingWorkTime?.prop.target_day !== workTime.prop.target_day
                "
              >
                {{ workTime.restEndByText }}
              </div>
              <div v-else>
                <input
                  type="time"
                  :value="workTime.restEndByText"
                  @change="
                    changeWorkTime(workTime, {
                      restEnd: ($event.target as HTMLInputElement).value,
                    })
                  "
                  class="border border-basic-300 rounded"
                />
              </div>
            </td>
            <td
              class="py-2 text-center text-sm text-basic-900 border border-table-border"
            >
              {{ workTime.restDurationByText ?? "00:00" }}
            </td>
            <td
              class="py-2 text-center text-sm text-basic-900 border border-table-border"
            >
              {{ workTime.workDurationByText ?? "00:00" }}
            </td>
            <td
              @dblclick="editWorkTime(workTime)"
              class="py-2 text-center text-sm text-basic-900 border border-table-border"
            >
              <div
                v-if="
                  editingWorkTime?.prop.target_day !== workTime.prop.target_day
                "
              >
                {{ workTime.memo }}
              </div>
              <div v-else>
                <input
                  type="text"
                  :value="workTime.memo"
                  @change="
                    changeWorkTime(workTime, {
                      memo: ($event.target as HTMLInputElement).value,
                    })
                  "
                  class="border border-basic-300 rounded"
                />
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
    <div
      v-if="changedWorkTimes.length > 0"
      class="fixed bottom-0 left-0 w-full shadow-lg p-4 flex justify-end items-center bg-black bg-opacity-50"
    >
      <button
        @click="saveChanges"
        class="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-700 transition mr-4"
      >
        保存
      </button>
      <button
        @click="resetChanges"
        class="bg-red-500 text-white px-4 py-2 rounded hover:bg-red-700 transition"
      >
        編集をリセット
      </button>
    </div>
  </div>
</template>

<script lang="ts">
import { format } from "date-fns";
import { defineComponent } from "vue";
import CommonSelect from "~/components/CommonSelect.vue";
import Loading from "~/components/Loading.vue";
import { JapaneseHolidayData } from "~/models/japaneseHoliday";
import { MonthForWork } from "~/models/monthForWork";
import { UserData } from "~/models/user";
import { workSettingData } from "~/models/workSetting";
import { workTimeData } from "~/models/workTime";
import { JapaneseHolidayRepository } from "~/repositories/tauri-commands/japaneseHoliday";
import { UserRepository } from "~/repositories/tauri-commands/user";
import { WorkTimeRepository } from "~/repositories/tauri-commands/workTime";
import { WorkSettingRepository } from "~/repositories/tauri-commands/workTimeSetting";

export default defineComponent({
  components: {
    Loading,
    CommonSelect,
  },
  data() {
    return {
      user: undefined as UserData | undefined,
      isLoading: true,
      selectedMonth: new MonthForWork(new Date(), []),
      workTimes: [] as workTimeData[],
      editingWorkTime: null as workTimeData | null,
      changedWorkTimes: [] as workTimeData[],
      workSettings: [] as workSettingData[],
      selectedWorkSettingId: 0,
      japaneseHolidays: [] as JapaneseHolidayData[],
    };
  },
  async mounted() {
    await this.init();
  },
  methods: {
    async init() {
      const userId = this.$route.params.id as string;
      this.user = new UserData(await UserRepository.getById(parseInt(userId)));
      const workTimeRes = await WorkTimeRepository.getWorkTimeByMonth(
        userId,
        this.selectedMonth.byText
      );
      this.workTimes = workTimeRes.map((wt) => new workTimeData(wt));
      this.selectedMonth = new MonthForWork(new Date(), this.workTimes);
      const workSettingRes = await WorkSettingRepository.getByUserId(
        parseInt(userId)
      );
      this.workSettings = workSettingRes.map((ws) => new workSettingData(ws));
      if (this.user.prop.default_work_setting_id) {
        this.selectedWorkSettingId = this.user.prop.default_work_setting_id;
      }
      const holiday = await JapaneseHolidayRepository.get(
        new Date().getFullYear().toString()
      );
      if (holiday.length === 0) {
        await JapaneseHolidayRepository.import();
        const importHoliday = await JapaneseHolidayRepository.get(
          new Date().getFullYear().toString()
        );
        this.japaneseHolidays = importHoliday.map(
          (h) => new JapaneseHolidayData(h)
        );
      } else {
        this.japaneseHolidays = holiday.map((h) => new JapaneseHolidayData(h));
      }
      this.isLoading = false;
    },
    getWorkTime(date: string): workTimeData | undefined {
      const workTime = this.workTimes.find(
        (workTime) => workTime.prop.target_day === date
      );
      if (workTime) {
        return workTime;
      }
    },
    totalWorkTime(): string {
      const total = this.workTimes.reduce((acc, workTime) => {
        return acc + workTime.workDurationByMinute;
      }, 0);
      return `${Math.floor(total / 60)}時間${total % 60}分`;
    },
    async previousMonth() {
      this.selectedMonth = this.selectedMonth.previousMonth;
      await this.fetchWorkTimeByMonth();
    },
    async nextMonth() {
      this.selectedMonth = this.selectedMonth.nextMonth;
      await this.fetchWorkTimeByMonth();
    },
    async fetchWorkTimeByMonth() {
      this.isLoading = true;
      if (this.user) {
        await WorkTimeRepository.getWorkTimeByMonth(
          this.user.prop.id.toString(),
          this.selectedMonth.byText
        )
          .then(
            (val) => (this.workTimes = val.map((wt) => new workTimeData(wt)))
          )
          .finally(() => (this.isLoading = false));
      }
    },
    async thisMonth() {
      this.selectedMonth = this.selectedMonth.thisMonth;
      await this.fetchWorkTimeByMonth();
    },
    editWorkTime(workTime: workTimeData) {
      this.editingWorkTime = workTime;
    },
    changeWorkTime(
      workTime: workTimeData,
      updateProp: {
        start?: string;
        end?: string;
        restStart?: string;
        restEnd?: string;
        memo?: string;
      }
    ) {
      if (updateProp.start) {
        const start = new Date(
          `${workTime.prop.target_day}T${updateProp.start}`
        );
        workTime.prop.start = start;
      }
      if (updateProp.end) {
        const end = new Date(`${workTime.prop.target_day}T${updateProp.end}`);
        workTime.prop.end = end;
      }
      if (updateProp.restStart) {
        const restStart = new Date(
          `${workTime.prop.target_day}T${updateProp.restStart}`
        );
        workTime.prop.rest_start = restStart;
      }
      if (updateProp.restEnd) {
        const restEnd = new Date(
          `${workTime.prop.target_day}T${updateProp.restEnd}`
        );
        workTime.prop.rest_end = restEnd;
      }
      if (updateProp.memo) {
        workTime.prop.memo = updateProp.memo;
      }

      if (
        this.workTimes.some(
          (wt) => wt.prop.target_day === workTime.prop.target_day
        )
      ) {
        this.workTimes = this.workTimes.map((wt) => {
          if (wt.prop.target_day === workTime.prop.target_day) {
            return workTime;
          }
          return wt;
        });
      } else {
        this.workTimes.push(workTime);
      }
      this.selectedMonth = new MonthForWork(
        this.selectedMonth.month,
        this.workTimes
      );
      if (
        this.changedWorkTimes.some(
          (wt) => wt.prop.target_day === workTime.prop.target_day
        )
      ) {
        this.changedWorkTimes = this.changedWorkTimes.map((wt) => {
          if (wt.prop.target_day === workTime.prop.target_day) {
            return workTime;
          }
          return wt;
        });
      } else {
        this.changedWorkTimes.push(workTime);
      }
      this.editingWorkTime = null;
    },
    async saveChanges() {
      this.changedWorkTimes.forEach((workTime) => {
        if (workTime.prop.id === -1) {
          WorkTimeRepository.create({
            start: workTime.prop.start,
            end: workTime.prop.end,
            restStart: workTime.prop.rest_start,
            restEnd: workTime.prop.rest_end,
            memo: workTime.prop.memo,
            userId: this.user!.prop.id,
            targetDay: workTime.prop.target_day,
          });
        } else {
          WorkTimeRepository.update({
            start: workTime.prop.start,
            end: workTime.prop.end,
            restStart: workTime.prop.rest_start,
            restEnd: workTime.prop.rest_end,
            memo: workTime.prop.memo,
            userId: this.user!.prop.id,
            targetDay: workTime.prop.target_day,
          });
        }
      });
      this.changedWorkTimes = [];
      this.editingWorkTime = null;
      await this.init();
    },
    async resetChanges() {
      this.changedWorkTimes = [];
      this.editingWorkTime = null;
      await this.init();
    },
    getSelectedWorkSetting() {
      return this.workSettings.find(
        (ws) => ws.prop.id === this.selectedWorkSettingId
      );
    },
    setDefaultWorkTime(workTime: workTimeData) {
      const workSetting = this.getSelectedWorkSetting();
      if (!workSetting) return;
      this.changeWorkTime(workTime, {
        start: format(workSetting.prop.start, "HH:mm"),
        end: format(workSetting.prop.end, "HH:mm"),
        restStart: format(workSetting.prop.rest_start, "HH:mm"),
        restEnd: format(workSetting.prop.rest_end, "HH:mm"),
        memo: workSetting.prop.memo,
      });
    },
  },
});
</script>
