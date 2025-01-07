<template>
  <Loading v-if="isLoading" />
  <div v-else>
    <Header />
    <div class="overflow-x-auto m-2 mb-20">
      <div class="flex justify-between items-center mb-4">
        <h2 class="text-xl font-semibold text-gray-700">
          {{ selectedMonth.yearText }}年{{
            selectedMonth.monthText
          }}月の勤務時間
        </h2>
        <div class="flex space-x-2">
          <button
            @click="previousMonth"
            class="px-4 py-2 bg-sky-500 text-white rounded hover:bg-sky-600"
          >
            前の月
          </button>
          <button
            @click="nextMonth"
            class="px-4 py-2 bg-rose-500 text-white rounded hover:bg-rose-600"
          >
            次の月
          </button>
        </div>
      </div>
      <div class="flex justify-end mb-4">
        <div class="flex items-center space-x-2">
          <span class="text-lg font-semibold text-gray-700"
            >今月の勤務時間:</span
          >
          <span class="text-lg font-semibold text-gray-900">
            {{ totalWorkTime() }}
          </span>
        </div>
      </div>
      <table class="min-w-full border-collapse border border-slate-520">
        <thead class="bg-gray-50">
          <tr>
            <th
              scope="col"
              class="w-2/12 py-3 text-center text-xs font-medium text-gray-500 border border-slate-300"
            >
              日付
            </th>
            <th
              scope="col"
              class="w-1/12 py-3 text-center text-xs font-medium text-gray-500 border border-slate-300"
            >
              開始時間
            </th>
            <th
              scope="col"
              class="w-1/12 py-3 text-center text-xs font-medium text-gray-500 border border-slate-300"
            >
              終了時間
            </th>
            <th
              scope="col"
              class="w-1/12 py-3 text-center text-xs font-medium text-gray-500 border border-slate-300"
            >
              休憩開始
            </th>
            <th
              scope="col"
              class="w-1/12 py-3 text-center text-xs font-medium text-gray-500 border border-slate-300"
            >
              休憩終了
            </th>
            <th
              scope="col"
              class="w-1/12 py-3 text-center text-xs font-medium text-gray-500 border border-slate-300"
            >
              休憩時間
            </th>
            <th
              scope="col"
              class="w-1/12 py-3 text-center text-xs font-medium text-gray-500 border border-slate-300"
            >
              業務時間
            </th>
            <th
              scope="col"
              class="w-1/2 py-3 text-center text-xs font-medium text-gray-500 border border-slate-300 w-auto"
            >
              備考
            </th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-200">
          <tr
            v-for="(workTime, index) in selectedMonth.monthWorkTimes"
            :key="index"
            :class="{
              'bg-lime-200':
                editingWorkTime?.prop.target_day === workTime.prop.target_day,
            }"
          >
            <td
              class="py-2 text-center text-sm text-gray-900 border border-slate-300"
            >
              {{ workTime.prop.target_day }}
            </td>
            <td
              @dblclick="editWorkTime(workTime)"
              class="py-2 text-center text-sm text-gray-900 border border-slate-300"
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
                  class="border border-gray-300 rounded"
                />
              </div>
            </td>
            <td
              @dblclick="editWorkTime(workTime)"
              class="py-2 text-center text-sm text-gray-900 border border-slate-300"
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
                  class="border border-gray-300 rounded"
                />
              </div>
            </td>
            <td
              @dblclick="editWorkTime(workTime)"
              class="py-2 text-center text-sm text-gray-900 border border-slate-300"
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
                  class="border border-gray-300 rounded"
                />
              </div>
            </td>
            <td
              @dblclick="editWorkTime(workTime)"
              class="py-2 text-center text-sm text-gray-900 border border-slate-300"
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
                  class="border border-gray-300 rounded"
                />
              </div>
            </td>
            <td
              class="py-2 text-center text-sm text-gray-900 border border-slate-300"
            >
              {{ workTime.restDurationByText ?? "00:00" }}
            </td>
            <td
              class="py-2 text-center text-sm text-gray-900 border border-slate-300"
            >
              {{ workTime.workDurationByText ?? "00:00" }}
            </td>
            <td
              @dblclick="editWorkTime(workTime)"
              class="py-2 text-center text-sm text-gray-900 border border-slate-300"
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
                  class="border border-gray-300 rounded"
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
import { defineComponent } from "vue";
import Header from "~/components/Header.vue";
import Loading from "~/components/Loading.vue";
import { MonthForWork } from "~/models/monthForWork";
import { UserData } from "~/models/user";
import { workTimeData } from "~/models/workTime";
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
      selectedMonth: new MonthForWork(new Date(), []),
      workTimes: [] as workTimeData[],
      editingWorkTime: null as workTimeData | null,
      changedWorkTimes: [] as workTimeData[],
    };
  },
  async mounted() {
    await this.init();
  },
  methods: {
    async init() {
      const userId = this.$route.params.id as string;
      this.user = new UserData(await UserRepository.getById(parseInt(userId)));
      await WorkTimeRepository.getWorkTimeByMonth(
        userId,
        this.selectedMonth.byText
      )
        .then((val) => {
          this.workTimes = val.map((wt) => new workTimeData(wt));
          this.selectedMonth = new MonthForWork(new Date(), this.workTimes);
        })
        .finally(() => (this.isLoading = false));
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
      this.isLoading = true;
      this.selectedMonth = this.selectedMonth.previousMonth;
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
    async nextMonth() {
      this.isLoading = true;
      this.selectedMonth = this.selectedMonth.nextMonth;
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
      console.log(this.changedWorkTimes);
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
            id: workTime.prop.id,
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
      console.log("saved", this.selectedMonth);
    },
    async resetChanges() {
      this.changedWorkTimes = [];
      this.editingWorkTime = null;
      await this.init();
    },
  },
});
</script>
