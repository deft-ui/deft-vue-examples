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
  <container :style="{
    alignItems: 'flex-start',
    justifyContent: 'center',
    height: '100%',
    width: '100%',
    gap: 20,
    padding: '0 20',

  }">
    <container :style="{width: '100%', gap: 30}">
      <container>
        <container>系统名称：{{sysInfo.sysName}}</container>
        <container>系统版本：{{sysInfo.sysOsVersion}}</container>
        <container>内核版本：{{sysInfo.sysKernelVersion}}</container>
        <container>处理器数：{{sysInfo.cpuCount}}</container>
        <container>总内存量：{{formatMem(sysInfo.memTotal)}}</container>
      </container>

      <container :style="{width: '100%', gap: 4}">
        <container :style="{width: '100%', justifyContent: 'space-between', flexDirection: 'row'}">
          <container>已用内存：{{ formatMem(sysInfo.memUsed) }}</container>
          <container>可用内存：{{ formatMem(sysInfo.memTotal - sysInfo.memUsed) }}</container>
        </container>
        <container :style="{width: '100%', height: 10, background: '#333'}">
          <container :style="{
          width: Math.round(100 * sysInfo.memUsed / sysInfo.memTotal) + '%',
          height: '100%',
          background: '#375fac'
        }"></container>
        </container>
      </container>
    </container>
  </container>
</template>