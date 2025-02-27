<script setup>
import { nextTick, onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Listbox from "primevue/listbox";
import "primeicons/primeicons.css";

const URL = "Url";
const PROGRAM = "Program";

const selectedItem = ref(null);
const items = ref([]);
const listboxRef = ref(null);
let firstFilterItem = null;

window.selectedItem = selectedItem;
window.listboxRef = listboxRef;
window.items = items;

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

    if (allItems.length > 0) {
        itemToOpen = allItems[0];
    }
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
            @keydown.enter="(e) => {
                if (selectedItem.value) {
                    openItem();
                } else if (firstFilterItem) {
                    selectedItem.value = firstFilterItem;
                    openItem();
                }
            }"
            spellcheck="false"
            :selectionMode="'single'"
            :focusOnHover="true"
            @filter="(e) => { 
                console.log(e.filterValue);
                if (!e.filterValue || !e.filterValue.length === 0) {
                    return;
                }
                firstFilterItem = e.filterValue[0];
            }"
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

<style scoped>
body {
    margin: 0;
    padding: 0;
}

.container {
    display: flex;
    flex-direction: column;

    margin: 0;
    padding: 0;
    flex-direction: column;
    justify-content: center;
    text-align: center;
}

:deep(.p-listbox-list-container) {
    max-height: none !important; /* Removes default limitation */
    height: 332px; /* Fills available space */
}
</style>
