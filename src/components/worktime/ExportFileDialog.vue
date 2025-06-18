<template>
  <button
    @click="isShow = true"
    class="px-4 py-2 bg-primary-400 text-basic-0 rounded-lg hover:bg-primary-500"
  >
    列を指定してダウンロード
  </button>
  <div
    class="fixed inset-0 flex items-center justify-center bg-gray-800 bg-opacity-50 z-50 overflow-auto"
    v-if="isShow"
  >
    <div class="bg-white p-6 rounded shadow-lg w-4/5 mt-10">
      <div class="form-group mb-4 border rounded p-4">
        <label for="columns" class="block mb-2"
          >エクスポートする列を選択してください</label
        >
        <CommonCheckbox
          v-model="isHeader"
          label="ヘッダも含める"
          container-class="p-2 mx-2 border border-basic-300 rounded-lg mb-1 w-1/2 bg-basic-50"
        />
        <div id="columns" class="w-full p-2 grid grid-cols-2 gap-2">
          <div
            v-for="(column, index) in selectableColumns"
            :key="index"
            class="flex items-center p-2 cursor-pointer border border-basic-300 justify-between"
            :class="{ 'bg-primary-100': selectedColumns.includes(column) }"
            @click="oncheckColumn(column)"
          >
            <label for="columns" class="cursor-pointer">{{ column }}</label>
            <Switch
              :class="
                selectedColumns.includes(column)
                  ? 'bg-primary-600'
                  : 'bg-basic-200'
              "
              class="relative inline-flex h-6 w-11 items-center rounded-full"
            >
              <span class="sr-only">Enable notifications</span>
              <span
                :class="
                  selectedColumns.includes(column)
                    ? 'translate-x-6'
                    : 'translate-x-1'
                "
                class="inline-block h-4 w-4 transform rounded-full bg-white transition"
              />
            </Switch>
          </div>
        </div>
      </div>
      <div class="form-group mb-4 border rounded p-4">
        <label for="separator" class="block mb-2"
          >区切り文字を選択してください</label
        >
        <div id="separator" class="w-full p-2 grid grid-cols-3 gap-2">
          <div
            v-for="(separator, index) in selectableSeparator"
            :key="index"
            class="flex items-center p-2 cursor-pointer border border-basic-300"
            :class="{ 'bg-primary-100': selectedSeparatorIndex === index }"
            @click="selectedSeparatorIndex = index"
          >
            <CommonCheckbox
              :model-value="selectedSeparatorIndex === index"
              @update:model-value="() => {}"
              :label="separator.text"
            />
          </div>
        </div>
      </div>
      <div class="flex justify-end">
        <button
          @click="clipCopy"
          class="px-4 py-2 bg-primary-500 text-white rounded mr-2 hover:bg-primary-600 flex items-center"
        >
          <Icon name="material-symbols:content-copy" class="h-5 w-5 mr-2" />
          クリップボードにコピー
        </button>
        <transition name="fade">
          <div
            v-if="message"
            class="fixed bottom-0 left-0 right-0 mb-4 mx-auto w-1/2 bg-basic-700 text-white text-center py-2 rounded shadow-lg opacity-70"
          >
            {{ message }}
          </div>
        </transition>
        <button
          @click="download"
          class="px-4 py-2 bg-primary-500 text-white rounded mr-2 hover:bg-primary-600 flex items-center"
        >
          <Icon name="material-symbols:download-rounded" class="h-5 w-5 mr-2" />
          ダウンロード
        </button>
        <button
          @click="isShow = false"
          class="px-4 py-2 bg-basic-300 text-black rounded hover:bg-gray-400 flex items-center"
        >
          <Icon name="material-symbols:close-rounded" class="h-5 w-5 mr-2" />
          閉じる
        </button>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { Switch } from "@headlessui/vue";
import { defineComponent } from "vue";
import CommonCheckbox from "~/components/CommonCheckbox.vue";
import { SeparatorArray } from "~/constants/separetor";

export default defineComponent({
  components: {
    Switch,
    CommonCheckbox,
  },
  props: {
    selectableColumns: {
      type: Array<string>,
      required: true,
    },
  },
  data() {
    return {
      selectedColumns: [] as string[],
      selectedSeparatorIndex: 0,
      selectableSeparator: SeparatorArray,
      isShow: false,
      message: "",
      isHeader: false,
    };
  },
  methods: {
    oncheckSeparetor(separatorIndex: number) {
      this.selectedSeparatorIndex = separatorIndex;
    },
    oncheckColumn(column: string) {
      if (this.selectedColumns.includes(column)) {
        this.selectedColumns = this.selectedColumns.filter(
          (selectedColumn) => selectedColumn !== column
        );
      } else {
        this.selectedColumns.push(column);
      }
    },
    download() {
      const separator = this.selectableSeparator[this.selectedSeparatorIndex];
      this.$emit(
        "download",
        this.selectedColumns,
        separator.value,
        this.isHeader
      );
      this.message = "ダウンロードしました";
      setTimeout(() => {
        this.message = "";
      }, 3000);
    },
    clipCopy() {
      const separator = this.selectableSeparator[this.selectedSeparatorIndex];
      this.$emit(
        "clipCopy",
        this.selectedColumns,
        separator.value,
        this.isHeader
      );
      this.message = "クリップボードにコピーしました";
      setTimeout(() => {
        this.message = "";
      }, 3000);
    },
  },
});
</script>
