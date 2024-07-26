
import { defineStore } from 'pinia';

export const useUiStore = defineStore('ui', {
  state: () => ({
    sidebarVisible: false,
  }),
  actions: {
    toggleSidebar() {
      this.sidebarVisible = !this.sidebarVisible;
    },
  }
});