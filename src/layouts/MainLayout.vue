<script setup lang="ts">
import Sidebar from '../components/Sidebar.vue';
</script>

<template>
  <div class="app-layout">
    <Sidebar />
    <main class="content-area">
      <slot></slot>
    </main>
  </div>
</template>

<style scoped>
.app-layout {
  display: flex;
  height: 100vh;
  width: 100vw;
  overflow: hidden;
  background-color: transparent;
  /* Essential for vibrancy */
}

.content-area {
  flex: 1;
  overflow-y: auto;
  padding: 1.5rem;
  background-color: transparent;
  /* Let the global app bg or vibrancy show through */
}

/* Custom scrollbar for webkit */
::-webkit-scrollbar {
  width: 6px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.1);
  border-radius: 3px;
}

::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.2);
}
</style>
