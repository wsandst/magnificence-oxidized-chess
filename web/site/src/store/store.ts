
import { defineStore } from 'pinia';
import { ChessEngine } from '../../wasm/magnificence_oxidized_web';

export const useMainStore = defineStore('main', {
  state: () => ({
    sidebarVisible: false,
    gamePaused: false,
    player1: null,
    player2: null,
    availablePlayers: []
  }),
  actions: {
    initAvailablePlayers(engine : ChessEngine) {
      let human = {"name": "Human", "profile": "src/assets/images/human-profile.png", "type": "human"}
      this.availablePlayers.push(human);
      for (const engineName of ChessEngine.get_allowed_engines()) {
        this.availablePlayers.push({"name": engineName, "profile": "src/assets/images/robot-profile.png", "type": "engine"})
      }
      this.player1 = this.availablePlayers[1];
      this.player2 = human;
    },
    toggleSidebar() {
      this.sidebarVisible = !this.sidebarVisible;
    }
  }
});