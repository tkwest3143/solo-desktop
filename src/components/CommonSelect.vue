<template>
  <div class="mb-4">
    <label :for="id" class="block text-sm font-medium text-gray-700">
      {{ label }}
    </label>
    <Listbox :modelValue="selectedValue" @update:modelValue="updateValue">
      <div class="relative mt-1">
        <ListboxButton
          class="border relative w-full cursor-default rounded-lg bg-white py-2 pl-3 pr-10 text-left shadow-md focus:outline-none focus-visible:ring-2 focus-visible:ring-opacity-75 focus-visible:ring-white focus-visible:ring-offset-2 focus-visible:ring-offset-orange-300 sm:text-sm"
          style="min-height: 2rem"
        >
          <span class="block truncate">{{ selectedOption.text }}</span>
          <span
            class="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-2"
          >
            <Icon
              name="material-symbols:keyboard-arrow-down"
              class="h-5 w-5 text-gray-400"
              aria-hidden="true"
            />
          </span>
        </ListboxButton>
        <transition
          leave-active-class="transition ease-in duration-100"
          leave-from-class="opacity-100"
          leave-to-class="opacity-0"
        >
          <ListboxOptions
            class="absolute mt-1 max-h-60 w-full overflow-auto rounded-md bg-white py-1 text-base shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none sm:text-sm"
          >
            <ListboxOption
              v-for="option in options"
              :key="option.value"
              :value="option.value"
              class="cursor-default select-none relative py-2 pl-10 pr-4"
            >
              <span
                :class="{
                  'font-medium': selectedValue === option.value,
                  'font-normal': selectedValue !== option.value,
                }"
                class="block truncate"
              >
                {{ option.text }}
              </span>
              <span
                v-if="selectedValue === option.value"
                class="absolute inset-y-0 left-0 flex items-center pl-3 text-indigo-600"
              >
                <Icon
                  name="material-symbols:check"
                  class="h-5 w-5"
                  aria-hidden="true"
                />
              </span>
            </ListboxOption>
          </ListboxOptions>
        </transition>
      </div>
    </Listbox>
  </div>
</template>

<script lang="ts">
import {
  Listbox,
  ListboxButton,
  ListboxLabel,
  ListboxOption,
  ListboxOptions,
} from "@headlessui/vue";
import { computed, defineComponent, ref } from "vue";
import type { PropType } from "vue";

export default defineComponent({
  components: {
    Listbox,
    ListboxButton,
    ListboxLabel,
    ListboxOption,
    ListboxOptions,
  },
  props: {
    id: {
      type: String,
      required: true,
    },
    label: {
      type: String,
      required: true,
    },
    modelValue: {
      type: [String, Number, Object],
      required: true,
    },
    options: {
      type: Array as PropType<Array<{ value: any; text: string }>>,
      required: true,
    },
  },
  setup(props, { emit }) {
    const selectedValue = ref(props.modelValue);

    const selectedOption = computed(() => {
      return (
        props.options.find(
          (option) => option.value === selectedValue.value
        ) || { text: "" }
      );
    });

    const updateValue = (value: any) => {
      selectedValue.value = value;
      emit("update:modelValue", value);
    };

    return {
      selectedValue,
      selectedOption,
      updateValue,
    };
  },
});
</script>
