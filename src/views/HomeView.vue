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
  <div class="flex flex-col items-center justify-center h-full max-w-2xl mx-auto text-center space-y-12">
    <!-- Hero Section -->
    <div class="space-y-4">
      <div class="inline-flex items-center justify-center p-3 bg-blue-500/10 rounded-2xl mb-4">
        <img src="/icon.png" alt="Logo" class="w-16 h-16 opacity-90" />
      </div>
      <h1 class="text-4xl font-bold bg-clip-text text-transparent bg-gradient-to-b from-white to-white/60">
        DevGen
      </h1>
      <p class="text-lg text-zinc-400 max-w-md mx-auto">
        Essential developer utilities, offline and privacy-first.
      </p>
    </div>

    <!-- Quick Links -->
    <div v-if="isLoading" class="flex justify-center">
      <Loader2 class="w-8 h-8 text-blue-500 animate-spin" />
    </div>

    <div v-else class="grid grid-cols-2 sm:grid-cols-3 gap-3 w-full px-4">
      <button v-for="genId in popularGenerators" :key="genId" @click="selectGenerator(genId)"
        class="glass-panel p-4 rounded-xl flex flex-col items-center gap-2 hover:bg-white/10 transition-all group">
        <span class="text-sm font-medium text-zinc-300 group-hover:text-white capitalize">
          {{ genId.replace('_', ' ') }}
        </span>
        <ArrowRight
          class="w-4 h-4 text-zinc-600 group-hover:text-blue-400 opacity-0 group-hover:opacity-100 transition-all transform group-hover:translate-x-1" />
      </button>
    </div>

    <!-- Keyboard Hint -->
    <div class="text-xs text-zinc-600 font-mono bg-white/5 px-3 py-1.5 rounded-full border border-white/5">
      Use sidebar to browse all categories
    </div>
  </div>
</template>
