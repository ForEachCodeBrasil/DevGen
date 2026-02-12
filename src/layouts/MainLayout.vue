<script setup lang="ts">
</script>

<template>
  <div class="panel-layout" data-tauri-drag-region>
    <slot />
  </div>
</template>

<style scoped>
.panel-layout {
  min-height: 100vh;
  min-width: 100vw;
  background: transparent;
  overflow: hidden;
}

/* Minimal scrollbar */
:deep(::-webkit-scrollbar) {
  width: 5px;
}

:deep(::-webkit-scrollbar-track) {
  background: transparent;
}

:deep(::-webkit-scrollbar-thumb) {
  background: rgba(255, 255, 255, 0.08);
  border-radius: 3px;
}

:deep(::-webkit-scrollbar-thumb:hover) {
  background: rgba(255, 255, 255, 0.15);
}
</style>
