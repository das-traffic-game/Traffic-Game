<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { Scene } from 'phaser';
import MainScene from './main_scene.js';

const game = ref(null);
const socket = onMounted(() => {
  let socket = new WebSocket(`ws://${location.hostname}:3100`);
  let config = {
    parent: 'canvas-container',
    scene: MainScene.new(socket),
  };
  game.value = new Phaser.Game(config);
});

const game_state = ref({});

onMounted(() => {});
</script>

<template>
  <div id="canvas-container"></div>
</template>
