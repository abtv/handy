<script setup>
import { nextTick, onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Listbox from "primevue/listbox";
import "primeicons/primeicons.css";

const URL = "Url";
const PROGRAM = "Program";

const selectedItem = ref();
const items = ref([]);
const listboxRef = ref(null);

onMounted(() => {
    nextTick(async () => {
        await initItems();
        focusFilter();
    });
});

async function openItem() {
    const value = selectedItem?.value;
    if (!value) {
        return;
    }

    const { name, type } = value;
    switch (type) {
        case URL:
            await invoke("open_url", { name });
            break;
        case PROGRAM:
            await invoke("run_program", { name });
            break;
        default:
            console.warn(`Method ${type} not found`);
    }

    resetFilter();
    selectedItem.value = null;
    await hideWindow();
}

async function initItems() {
    items.value.length = 0;

    const urls = await invoke("get_urls");
    const urlItems = urls.map((x) => ({ name: x, type: URL }));

    const programs = await invoke("get_programs");
    const programItems = programs.map((x) => ({ name: x, type: PROGRAM }));

    const allItems = [...urlItems, ...programItems];
    allItems.sort((a, b) => a.name.localeCompare(b.name));

    items.value.push(...allItems);
}

function focusFilter() {
    const input = listboxRef.value?.$el.querySelector("input");
    if (!input) {
        return;
    }
    input.focus();
}

const resetFilter = () => {
    if (!listboxRef.value) {
        return;
    }

    const input = listboxRef.value.$el.querySelector("input");
    if (!input) {
        return;
    }

    input.value = ""; // Manually clear the input field
    input.dispatchEvent(new Event("input")); // Trigger Vue reactivity
    input.focus();
    selectedItem.value = null;
};

async function hideWindow() {
    await invoke("hide_window");
}
</script>

<template>
    <main class="container">
        <Listbox
            ref="listboxRef"
            v-model="selectedItem"
            :options="items"
            optionLabel="name"
            :filter="true"
            :multiple="false"
            @change="openItem"
            @keydown.esc="hideWindow"
            spellcheck="false"
        >
            <template #option="slotProps">
                <div class="flex" style="text-align: left">
                    <i
                        class="pi pi-external-link"
                        style="font-size: 1rem"
                        v-if="slotProps.option.type === URL"
                    ></i>
                    <i
                        class="pi pi-arrow-circle-right"
                        style="font-size: 1rem"
                        v-else-if="slotProps.option.type === PROGRAM"
                    ></i>
                    <i class="pi pi-code" style="font-size: 1rem" v-else></i>
                    &nbsp;
                    {{ slotProps.option.name }}
                </div>
            </template>
        </Listbox>
    </main>
</template>

<style>
body {
    padding: 0px;
    margin: 0px;
}

:root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
}

.container {
    margin: 0;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
}
</style>
