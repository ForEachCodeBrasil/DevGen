<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import {
  Search,
  Settings,
  FileText,
  User,
  Building2,
  Car,
  Wrench,
  Grid2X2
} from 'lucide-vue-next';

interface GeneratorDefinition {
  id: string;
  name: string;
  category: string;
  description: string;
}

const router = useRouter();
const route = useRoute();
const searchQuery = ref('');
const generators = ref<GeneratorDefinition[]>([]);
const selectedCategory = ref<string | null>(null);

// Map categories to icons
const getCategoryIcon = (category: string) => {
  switch (category.toLowerCase()) {
    case 'documents': return FileText;
    case 'person': return User;
    case 'company': return Building2;
    case 'vehicle': return Car;
    case 'utilities': return Wrench;
    default: return Grid2X2;
  }
};

const categories = computed(() => {
  const cats = new Set(generators.value.map(g => g.category));
  return Array.from(cats).sort();
});

const filteredGenerators = computed(() => {
  return generators.value.filter(g => {
    const matchesSearch = g.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
      g.description.toLowerCase().includes(searchQuery.value.toLowerCase());
    const matchesCategory = selectedCategory.value ? g.category === selectedCategory.value : true;
    return matchesSearch && matchesCategory;
  });
});

async function loadGenerators() {
  try {
    generators.value = await invoke('list_generators');
  } catch (e) {
    console.error('Failed to list generators', e);
  }
}

function selectGenerator(id: string) {
  router.push({ name: 'generator', params: { id } });
}

onMounted(() => {
  loadGenerators();
});
</script>

<template>
  <aside class="w-64 flex flex-col h-full border-r border-white/5 bg-black/20 backdrop-blur-xl" data-tauri-drag-region>

    <!-- Header / Native Traffic Lights Spacer + Search -->
    <div class="pt-10 px-3 pb-2 space-y-3" data-tauri-drag-region>
      <!-- Native-style Search Input (Darker, recessed) -->
      <div class="relative group">
        <Search
          class="absolute left-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-zinc-500 group-focus-within:text-zinc-300 transition-colors" />
        <input v-model="searchQuery" type="text" placeholder="Search" class="w-full bg-black/40 border border-white/5 rounded-[6px] pl-8 pr-2 py-1 text-[11px] text-zinc-300 placeholder-zinc-600 
                 shadow-inner focus:outline-none focus:bg-black/60 focus:border-white/10 transition-all font-medium" />
      </div>

      <!-- Categories as Segmented Control (CodexBar Style) -->
      <!-- We need a scrolling container because 55 gens have many categories, but we mimic the LOOK of tabs -->
      <div class="flex gap-0.5 overflow-x-auto no-scrollbar p-0.5 bg-black/20 rounded-lg border border-white/5">
        <button @click="selectedCategory = null" :class="[
          'flex-1 px-3 py-1 rounded-[5px] text-[10px] font-medium transition-all text-center whitespace-nowrap',
          selectedCategory === null
            ? 'bg-zinc-700/80 text-white shadow-sm'
            : 'text-zinc-500 hover:text-zinc-300 hover:bg-white/5'
        ]">
          All
        </button>
        <button v-for="cat in categories" :key="cat" @click="selectedCategory = cat" :class="[
          'flex-1 px-3 py-1 rounded-[5px] text-[10px] font-medium transition-all text-center whitespace-nowrap',
          selectedCategory === cat
            ? 'bg-zinc-700/80 text-white shadow-sm'
            : 'text-zinc-500 hover:text-zinc-300 hover:bg-white/5'
        ]">
          {{ cat }}
        </button>
      </div>
    </div>

    <!-- Generator List (High Density) -->
    <div class="flex-1 overflow-y-auto px-2 pb-2 mt-1">
      <!-- Section Header if needed, e.g. "Generators" (Skipping for minimalism) -->

      <div v-for="gen in filteredGenerators" :key="gen.id" @click="selectGenerator(gen.id)" :class="[
        'group flex items-center gap-2 px-2.5 py-1.5 mb-0.5 rounded-[5px] cursor-pointer transition-all duration-100 select-none',
        route.params.id === gen.id
          ? 'bg-blue-600 text-white shadow-sm' /* Active: Solid Blue like macOS selection */
          : 'text-zinc-400 hover:bg-white/5 hover:text-zinc-200'
      ]">
        <!-- Icon -->
        <component :is="getCategoryIcon(gen.category)" :class="[
          'w-3.5 h-3.5 flex-shrink-0 opacity-80',
          route.params.id === gen.id ? 'text-white' : 'text-zinc-500 group-hover:text-zinc-400'
        ]" />

        <!-- Name -->
        <span class="text-[11px] font-medium truncate flex-1 leading-none pt-0.5">
          {{ gen.name }}
        </span>

        <!-- Chevron Only on Hover (Subtle) -->
        <div v-if="route.params.id === gen.id" class="w-1 h-1 rounded-full bg-white/50"></div>
      </div>
    </div>

    <!-- Footer (Minimal settings gear) -->
    <div class="p-2 border-t border-white/5 mt-auto bg-black/20">
      <button @click="router.push('/settings')"
        class="flex items-center justify-center w-8 h-8 rounded-md text-zinc-600 hover:text-zinc-300 hover:bg-white/5 transition-colors">
        <Settings class="w-4 h-4" />
      </button>
    </div>
  </aside>
</template>

<style scoped>
.no-scrollbar::-webkit-scrollbar {
  display: none;
}

.no-scrollbar {
  -ms-overflow-style: none;
  scrollbar-width: none;
}
</style>
