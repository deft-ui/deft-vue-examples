/// <reference types="deft-sys" />
/// <reference types="deft-vue/types/index" />

declare const console;

declare module '*.vue' {
    import type { DefineComponent } from 'vue'
    const component: DefineComponent<{}, {}, any>
    export default component
}

declare interface SysInfo {
    cpuCount: number;
    memTotal: number;
    memFree: number;
    memUsed: number;
    sysName: string;
    sysKernelVersion: string;
    sysOsVersion: string;
    sysHostname: string;
}

declare function get_sys_info(): Promise<SysInfo>