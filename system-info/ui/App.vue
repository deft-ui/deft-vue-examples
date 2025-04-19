<script setup lang="ts">
import {onUnmounted, ref} from "@vue/runtime-core";

const sysInfo = ref({} as SysInfo);

let stopped = false;

async function updateInfoLoop() {
  if (stopped) {
    return;
  }
  sysInfo.value = await get_sys_info();
  setTimeout(updateInfoLoop, 1000);
}

updateInfoLoop();

onUnmounted(() => {
  stopped = true;
})

function formatMem(value) {
  if (!value) {
    return 'NA'
  }
  return (value / 1024 / 1024 / 1024).toFixed(2) + "GB";
}

</script>

<template>
  <container class="main">
    <container :style="{width: '100%', gap: 30}">
      <container>
        <container>系统名称：{{sysInfo.sysName}}</container>
        <container>系统版本：{{sysInfo.sysOsVersion}}</container>
        <container>内核版本：{{sysInfo.sysKernelVersion}}</container>
        <container>处理器数：{{sysInfo.cpuCount}}</container>
        <container>总内存量：{{formatMem(sysInfo.memTotal)}}</container>
      </container>

      <container class="mem-info">
        <container class="mem-info-text-container">
          <container>已用内存：{{ formatMem(sysInfo.memUsed) }}</container>
          <container>可用内存：{{ formatMem(sysInfo.memTotal - sysInfo.memUsed) }}</container>
        </container>
        <container class="indicator">
          <container class="indicator-thumb" :style="{
            width: Math.round(100 * sysInfo.memUsed / sysInfo.memTotal) + '%',
          }"></container>
        </container>
      </container>
    </container>
  </container>
</template>

<style scoped>
.main {
  align-items: flex-start;
  justify-content: center;
  height: 100%;
  width: 100%;
  gap: 20px;
  padding: 0 20px;
}
.indicator {
  width: 100%;
  height: 10px;
  background: #bbb;
}
.indicator-thumb {
  height: 100%;
  background: #375fac;
}
.mem-info {
  width: 100%;
  gap: 4px;
}
.mem-info-text-container {
  width: 100%;
  justify-content: space-between;
  flex-direction: row;
}
</style>