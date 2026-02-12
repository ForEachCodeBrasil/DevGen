<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { useGeneratorsStore } from '../stores/generators';
import { Loader2, ArrowRight } from 'lucide-vue-next';

const router = useRouter();
const store = useGeneratorsStore();
const isLoading = ref(false);

const popularGenerators = [
  'cpf', 'cnpj', 'uuid', 'credit_card', 'password'
];

function selectGenerator(id: string) {
  router.push({ name: 'generator', params: { id } });
}

onMounted(async () => {
  if (store.generators.length === 0) {
    isLoading.value = true;
    await store.fetchGenerators();
    isLoading.value = false;
  }
});
</script>

<template>
  <div class="flex flex-col h-full overflow-hidden animate-in fade-in zoom-in-95 duration-300">

    <!-- Ultra-Compact Hero (Header Style) -->
    <div class="px-6 pt-10 pb-4 flex items-center justify-between border-b border-white/5 bg-white/[0.02]">
      <div class="flex items-center gap-3">
        <!-- Logo Icon -->
        <div
          class="flex items-center justify-center w-8 h-8 bg-blue-500/10 rounded-lg border border-blue-500/20 shadow-sm">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"
            stroke-linecap="round" stroke-linejoin="round" class="w-4 h-4 text-blue-400">
            <path d="m18 16 4-4-4-4" />
            <path d="m6 8-4 4 4 4" />
            <path d="m14.5 4-5 16" />
          </svg>
        </div>
        <div class="flex flex-col">
          <h1 class="text-sm font-bold text-zinc-100 tracking-tight leading-none">DevGen</h1>
          <span class="text-[10px] text-zinc-500 font-medium uppercase tracking-wider">Premium Utilities</span>
        </div>
      </div>
      <!-- Status Badge mimicking CodexBar's "Pro" or status -->
      <div
        class="px-2 py-0.5 rounded-full bg-green-500/10 border border-green-500/20 text-[9px] font-bold text-green-400 uppercase tracking-wide">
        v0.1.0-beta
      </div>
    </div>

    <!-- Scrollable Content Area -->
    <div class="flex-1 overflow-y-auto p-4 space-y-5">

      <!-- Quick Actions Section -->
      <div v-if="isLoading" class="flex justify-center py-4">
        <Loader2 class="w-5 h-5 text-zinc-600 animate-spin" />
      </div>

      <div v-else>
        <div class="flex items-center justify-between mb-2 px-1">
          <h2 class="text-[10px] font-bold text-zinc-500 uppercase tracking-widest">Quick Access</h2>
        </div>

        <div class="grid grid-cols-2 gap-2">
          <button v-for="genId in popularGenerators" :key="genId" @click="selectGenerator(genId)"
            class="group flex items-center gap-2.5 px-3 py-2 bg-zinc-900/40 hover:bg-white/5 border border-white/5 hover:border-white/10 rounded-md transition-all">
            <!-- Tiny Icon Placeholder -->
            <div class="w-1.5 h-1.5 rounded-full bg-zinc-700 group-hover:bg-blue-400 transition-colors"></div>

            <span class="text-xs font-medium text-zinc-400 group-hover:text-zinc-200 capitalize truncate">
              {{ genId.replace(/_/g, ' ') }}
            </span>

            <!-- Hidden arrow that appears -->
            <ArrowRight
              class="w-3 h-3 text-zinc-600 ml-auto opacity-0 group-hover:opacity-100 -translate-x-1 group-hover:translate-x-0 transition-all" />
          </button>
        </div>
      </div>

      <!-- Discovery / Info Section (Simulating CodexBar's usage stats look) -->
      <div>
        <div class="flex items-center justify-between mb-2 px-1">
          <h2 class="text-[10px] font-bold text-zinc-500 uppercase tracking-widest">System Status</h2>
        </div>

        <div class="bg-zinc-900/30 border border-white/5 rounded-lg p-3 space-y-3">
          <!-- Fake Progress Bars to match aesthetic -->
          <div class="space-y-1">
            <div class="flex justify-between text-[10px] text-zinc-400">
              <span>Offline Mode</span>
              <span class="text-green-400">Active</span>
            </div>
            <div class="h-1 w-full bg-zinc-800 rounded-full overflow-hidden">
              <div class="h-full bg-green-500/80 w-full rounded-full"></div>
            </div>
            <div class="flex justify-between text-[9px] text-zinc-600">
              <span>No external calls</span>
              <span>Secure</span>
            </div>
          </div>

          <div class="space-y-1 pt-1">
            <div class="flex justify-between text-[10px] text-zinc-400">
              <span>Performance</span>
              <span class="text-zinc-300">Optimal</span>
            </div>
            <div class="h-1 w-full bg-zinc-800 rounded-full overflow-hidden">
              <div class="h-full bg-blue-500/60 w-[94%] rounded-full"></div>
            </div>
          </div>
        </div>
      </div>

    </div>

    <!-- Footer Hint -->
    <div class="p-2 text-center border-t border-white/5 bg-black/20">
      <span class="text-[9px] text-zinc-600 font-medium">PRESS <kbd
          class="font-sans bg-white/10 px-1 rounded text-zinc-500">⌘K</kbd> TO SEARCH</span>
    </div>
  </div>
</template>
